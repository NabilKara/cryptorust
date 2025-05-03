use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::thread;
use num_bigint::BigUint;
use num_traits::Zero;
use rand::random;
use crate::asymmetric_encryption::RSA::rsa_generate_key_pair;
use crate::secureChannel::util::*;
use crate::symmetric_encryption::aes::aes_encryption::encrypt_cbc as AES_CBC_Encrypt;
use crate::symmetric_encryption::aes::aes_decryption::decrypt_cbc as AES_CBC_Decrypt;
use crate::asymmetric_encryption::RSA::encrypt as RSA_Encrypt;
use crate::asymmetric_encryption::RSA::decrypt as RSA_Decrypt;

const IV_SIZE: usize = 16;                  // AES-128 IV

const SEPARATOR_SIZE: usize = 8;
const SEPARATOR: [u8; DELIMITER_SIZE] = [0xFFu8; DELIMITER_SIZE];

const HASH_SIZE: usize = 32;                // SHA2-256

const DELIMITER_SIZE: usize = 8;            // Changeable
const FRAME_DELIMITER: [u8; DELIMITER_SIZE] = [0u8; DELIMITER_SIZE];  // Changeable

const PACKET_MIN_SIZE: usize = IV_SIZE + SEPARATOR_SIZE + HASH_SIZE + DELIMITER_SIZE;

struct RSA_KEY {
    d: BigUint,
    e: BigUint,
    n: BigUint,
}

#[derive(Debug, Clone)]
struct Message {
    IV: [u8; 16],
    Data: Vec<u8>,
    Separator: [u8; SEPARATOR_SIZE],
    Signature: Vec<u8>,
    Hash: [u8; HASH_SIZE],
    Delimiter: [u8; DELIMITER_SIZE],
}

impl Message {
    fn new(data: &Vec<u8>, aes_key: &[u8; 16], rsa_key: &RSA_KEY) -> Message {
        let iv: [u8; 16] = random::<[u8; 16]>();
        let encrypted_data = AES_CBC_Encrypt(data, &iv, &aes_key);
        let mut payload = iv.clone().to_vec();
        payload.extend(&iv);

        let hash = [0x90u8; HASH_SIZE];
        let signature = RSA_Encrypt(&hash.to_vec(), &rsa_key.d, &rsa_key.n);

        Message {
            IV: iv,
            Data: encrypted_data,
            Separator: SEPARATOR,
            Signature: signature,
            Hash: hash,       // Example
            Delimiter: FRAME_DELIMITER,
        }
    }

    fn toBytes(self) -> Vec<u8> {
        let mut output = Vec::new();
        output.extend(&self.IV);
        output.extend(&self.Data);
        output.extend(&self.Separator);
        output.extend(&self.Signature);
        output.extend(&self.Hash);
        output.extend(&self.Delimiter);
        output
    }

    fn fromBytes(data: &[u8], rsa_key: RSA_KEY) -> io::Result<Message> {
        if data.len() < PACKET_MIN_SIZE { panic!("Message too small to contain base structure."); }

        let delimiter_index = match data.windows(DELIMITER_SIZE).position(|window| window == FRAME_DELIMITER) {
            Some(pos) => pos,
            None => panic!("FRAME_DELIMITER Not Found."),
        };

        let separator_index = match data.windows(SEPARATOR_SIZE).position(|window| window == SEPARATOR) {
            Some(pos) => pos,
            None => panic!("SEPARATOR Not Found."),
        };

        let iv: [u8; IV_SIZE]                =  data[..IV_SIZE].try_into().unwrap();
        let encrypted_data: Vec<u8>          =  data[IV_SIZE..separator_index].to_vec();
        let separator: [u8; SEPARATOR_SIZE]  =  data[separator_index..separator_index + SEPARATOR_SIZE].try_into().unwrap();
        let signature: Vec<u8>               =  data[separator_index + SEPARATOR_SIZE..delimiter_index - HASH_SIZE].to_vec();
        let hash: [u8; HASH_SIZE]            =  data[delimiter_index - HASH_SIZE..delimiter_index].try_into().unwrap();
        let delimiter: [u8; DELIMITER_SIZE]  =  data[delimiter_index..delimiter_index + DELIMITER_SIZE].try_into().unwrap();

        let signature = RSA_Decrypt(&signature, &rsa_key.e, &rsa_key.n);
        if !cmp_vec(&hash.to_vec(), &signature) { panic!("Signature Hash Mismatch."); }
        
        Ok(Message {
            IV: iv,
            Data: encrypted_data,
            Separator: separator,
            Signature: signature,
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

    print!("Generating RSA Key pair... ");          io::stdout().flush().unwrap();
    let (local_mod, local_pub, local_prv) = rsa_generate_key_pair(512, 5);
    let local_rsa = RSA_KEY {
        n: local_mod,
        e: local_pub,
        d: local_prv,
    };
    println!("Done.");

    // Get peer address once at start
    let peer_addr = stream.peer_addr().unwrap();
    let local_addr = stream.local_addr().unwrap();

    let mut input = String::new();

    print!("Exchanging RSA Credentials... ");       io::stdout().flush().unwrap();
    let mut recv_stream = stream.try_clone().expect("Failed to clone stream");
    let mut send_stream = stream.try_clone().expect("Failed to clone stream");

    // Send public key to peer
    write_biguint(&mut send_stream, &local_rsa.n).expect("Failed to send modulus");
    write_biguint(&mut send_stream, &local_rsa.e).expect("Failed to send public key");

    // Receive RSA Public key from peer
    let peer_mod = read_biguint(&mut recv_stream).expect("Failed to read peer's modulus");
    let peer_pub = read_biguint(&mut recv_stream).expect("Failed to read peer's public key");

    let peer_rsa = RSA_KEY {
        n: peer_mod,
        e: peer_pub,
        d: BigUint::zero()
    };
    println!("Done.");

    println!("\nLocal RSA Credentials:");
    println!("\tModulus    : {}", local_rsa.n);
    println!("\tPublic Key : {}", local_rsa.e);
    println!("\tPrivate Key: {}", local_rsa.d);

    println!("Peer RSA Credentials:");
    println!("\tModulus   : {}", peer_rsa.n);
    println!("\tPublic Key: {}", peer_rsa.e);
    println!("\n");

    // Spawn receiver thread
    thread::spawn(move || {
        let mut buffer = [0u8; 1024];
        // Main Receiving and printing loop
        loop {
            match recv_stream.read(&mut buffer) {
                Ok(n) if n > 0 => {
                    print!("\x1B[2K\r"); // Clear current line
                    let msg = Message::fromBytes(&buffer).expect("Invalid Message.");

                    println!("Received: {:?}", msg);

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
                let message = Message::new(&input.clone().into_bytes(), &key, &local_rsa);

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