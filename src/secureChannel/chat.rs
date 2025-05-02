use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::thread;
use rand::random;
use crate::symmetric_encryption::aes::aes_encryption::encrypt_cbc as AES_CBC_Encrypt;
use crate::symmetric_encryption::aes::aes_decryption::decrypt_cbc as AES_CBC_Decrypt;

const IV_SIZE: usize = 16;                  // AES-128 IV
const HASH_SIZE: usize = 32;                // SHA2-256

const DELIMITER_SIZE: usize = 8;            // Changeable
const FRAME_DELIMITER: [u8; DELIMITER_SIZE] = [0u8; DELIMITER_SIZE];  // Changeable

#[derive(Debug, Clone)]
struct Message {
    IV: [u8; 16],
    Data: Vec<u8>,
    Hash: [u8; HASH_SIZE],
    Delimiter: [u8; DELIMITER_SIZE],
}

impl Message {
    fn new(data: &Vec<u8>, key: &[u8; 16]) -> Message {
        let iv: [u8; 16] = random::<[u8; 16]>();
        let encrypted_data = AES_CBC_Encrypt(data, &iv, &key);
        Message {
            IV: iv,
            Data: encrypted_data,
            Hash: [0x90u8; HASH_SIZE],       // Example
            Delimiter: FRAME_DELIMITER,
        }
    }
    fn toBytes(self) -> Vec<u8> {
        let mut output = Vec::new();
        output.extend(&self.IV);
        output.extend(&self.Data);
        output.extend(&self.Hash);
        output.extend(&self.Delimiter);
        output
    }

    fn fromBytes(data: &[u8]) -> io::Result<Message> {
        if data.len() < IV_SIZE + HASH_SIZE + DELIMITER_SIZE { panic!("Message too small to contain base structure."); }

        let delimiter_index = match data.windows(DELIMITER_SIZE).position(|window| window == FRAME_DELIMITER) {
            Some(pos) => pos,
            None => panic!("FRAME_DELIMITER Not Found."),
        };

        let iv: [u8; IV_SIZE]                =  data[..IV_SIZE].try_into().unwrap();
        let encrypted_data          =  data[IV_SIZE..delimiter_index - HASH_SIZE].to_vec();
        let hash: [u8; HASH_SIZE]            =  data[delimiter_index - HASH_SIZE..delimiter_index].try_into().unwrap();
        let delimiter: [u8; DELIMITER_SIZE]  =  data[delimiter_index..delimiter_index + DELIMITER_SIZE].try_into().unwrap();

        Ok(Message {
            IV: iv,
            Data: encrypted_data,
            Hash: hash,
            Delimiter: delimiter,
        })
    }

    fn getClearText(&self, key: &[u8; 16]) -> Vec<u8> {
        match AES_CBC_Decrypt(self.clone().Data, &self.IV, key) {
            Ok(cleartext) => cleartext,
            Err(e) => panic!("Error Decrypting message: {}", e)
        }
    }
}

pub fn chat_loop(stream: TcpStream, key: [u8; 16], is_server: bool) {
    // Get peer address once at start
    let peer_addr = stream.peer_addr().unwrap();
    let local_addr = stream.local_addr().unwrap();

    let mut input = String::new();

    // Spawn receiver thread
    let mut recv_stream = stream.try_clone().expect("Failed to clone stream");
    thread::spawn(move || {
        let mut buffer = [0u8; 1024];
        loop {
            match recv_stream.read(&mut buffer) {
                Ok(n) if n > 0 => {
                    print!("\x1B[2K\r"); // Clear current line
                    let msg = Message::fromBytes(&buffer).expect("Invalid Message.");

                    println!("Text before Decrypting: {:?}", msg.Data);
                    println!("Text  after Decrypting: {:?}", msg.getClearText(&key));

                    let cleartext = msg.getClearText(&key);
                    let cleartext = String::from_utf8_lossy(cleartext.as_slice());

                    print!("{}:{}> {}\n{}:{}> ",
                           peer_addr.ip(), peer_addr.port(),
                           cleartext,
                           local_addr.ip(), local_addr.port(),
                    );
                    io::stdout().flush().unwrap();
                    buffer = [0u8; 1024];
                },
                Ok(_) => break, // Connection closed
                Err(e) => {
                    eprintln!("\nConnection error: {}", e);
                    break;
                }
            }
        }
    });

    // Main sending loop
    let mut send_stream = stream.try_clone().expect("Failed to clone stream");
    loop {
        let mut c = [0u8; 1];

        print!("{}:{}> ", local_addr.ip(), local_addr.port());
        io::stdout().flush().unwrap();

        loop {
            match io::stdin().read_exact(&mut c) {
                Ok(_) => if c[0] as char != '\n' { input.push(c[0] as char); },
                Err(e) => {
                    eprintln!("Input char: {}", e);
                    break;
                }
            }

            if c[0] as char == '\n' {
                if input.is_empty() { continue; }

                input.push(0 as char); // For termination
                let message = Message::new(&input.clone().into_bytes(), &key);

                println!("Sending {:?}...", message);
                println!("Sending {:?}...", message.clone().toBytes().as_slice());

                if send_stream.write_all(message.toBytes().as_slice()).is_err() {
                    eprintln!("Failed to send message");
                    break;
                }
                input.clear();
                break;
            }
        }
    }
}