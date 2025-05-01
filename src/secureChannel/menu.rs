use std::collections::HashMap;
use std::io;
use std::io::{Read, Write};
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream};
use std::str::FromStr;
use std::sync::Mutex;
use crate::asymmetric_encryption::ElGamal::{ElGamal_decrypt, ElGamal_encrypt, ElGamal_generate_keys};
use num_bigint::BigUint;
use rand::random;

const PROTOCOL_PORT: u16 = 42069;
const options: [&str; 3] = [
    "1- Host",
    "2- Connect",
    "3- Return"
];

#[derive(Debug, Clone)]
struct Header {
    room_id: u8,
    aes_key: [u8; 16],
}

struct Client {
    id: u8,
    stream: TcpStream,
}

struct Server {
    clients: Mutex<HashMap<u8, TcpStream>>,
    header: Header,
}

impl Server {
    fn broadcast(&self, sender_id: u8, message: &[u8]) {
        let clients = self.clients.lock().unwrap();
        for (&id, mut stream) in clients.iter() {
            if id != sender_id { let _ = stream.write_all(message); }
        }
    }
}

fn write_biguint(writer: &mut impl Write, num: &BigUint) -> io::Result<()> {
    let bytes = num.to_bytes_be();
    writer.write_all(&(bytes.len() as u32).to_be_bytes())?;
    writer.write_all(&bytes)
}

fn read_biguint(reader: &mut impl Read) -> io::Result<BigUint> {
    let mut len_buf = [0u8; 4];
    reader.read_exact(&mut len_buf)?;
    let len = u32::from_be_bytes(len_buf) as usize;

    let mut buf = vec![0u8; len];
    reader.read_exact(&mut buf)?;
    Ok(BigUint::from_bytes_be(&buf))
}

fn printMenu(){
    println!("Please choose an option:");
    for i in 0..options.len(){
        println!("{}", options[i]);
    }
}

pub fn Menu(PATH: &mut String){
    const PREFIX: &str = "channel/";
    let listener: bool = false;

    PATH.push_str(PREFIX);
    // loop {
        printMenu();
        let mut r = super::getInput(PATH.clone(), 1, options.len());
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        match r {
            1 => Host(),
            2 => Connect(),
            _ => return
        };
    // }

    PATH.drain(PATH.len() - PREFIX.len()..);
    return;
}

fn Connect() {
    let mut addr = String::new();
    print!("Enter IPv4 Address: ");               io::stdout().flush().unwrap();
    io::stdin().read_line(&mut addr).expect("Failed to read plaintext");
    addr = addr.trim().to_string();

    let addr = Ipv4Addr::from_str(&addr).expect(format!("Invalid Address '{addr}'").as_str());
    let sock = SocketAddrV4::from_str(format!("{addr}:{PROTOCOL_PORT}").as_str())
        .expect(format!("Invalid socket address '{addr}'").as_str());

    print!("Connecting to {}:{}...", sock.ip(), sock.port());  io::stdout().flush().unwrap();  // Force flush
    match TcpStream::connect(sock) {
        Ok(mut stream) => {
            establishSession_connector(&mut stream);
            println!(" Done.");
        },
        Err(e) => panic!("\nError Connecting to {sock}: {e}")
    };
}

fn establishSession_connector(stream: &mut TcpStream) {
    let (pub_key, prv_key, p, g) = ElGamal_generate_keys(256, 10);

    // Send public parameters to client
    write_biguint(stream, &p).expect("Failed to send p");
    write_biguint(stream, &g).expect("Failed to send g");
    write_biguint(stream, &pub_key).expect("Failed to send public key");

    // Receive encrypted AES key from client
    let encrypted_aes = read_biguint(stream).expect("Failed to read ciphertext");
    let ephemeral_pk = read_biguint(stream).expect("Failed to read ephemeral_pk");

    // Decrypt the AES key
    let aes_key = ElGamal_decrypt(&encrypted_aes, &ephemeral_pk, &p, &prv_key);
    let aes_key = aes_key.to_bytes_be();

    // Now both parties have the same AES key for secure communication
    println!("Established session with AES key: {:?}", aes_key);
}

fn Host() {
    let listener = TcpListener::bind(("127.0.0.1", PROTOCOL_PORT)).expect("Failed to bind port");
    println!("Server listening on  {}:{}...", listener.local_addr().unwrap().ip(), listener.local_addr().unwrap().port());

    // Generate a random AES key (32 bytes for AES-256)
    let id = random::<u8>();
    let aes_key = random::<[u8; 16]>();
    let header = Header { room_id: id, aes_key};

    let mut streams = Vec::new();

    std::thread::spawn(move || {
        for stream in listener.incoming(){
            match stream{
                Ok(mut stream) => {
                    let header = header.clone();
                    println!("New connection from {:?}...", stream);
                    establishConnection_host(&mut stream, &header);
                    streams.push(stream);
                }
                Err(e) => eprintln!("Error: {}", e)
            }
        }
    });

}

fn establishConnection_host(stream: &mut TcpStream, header: &Header) {
    // Receive ElGamal parameters from server
    let p = read_biguint(stream).expect("Failed to read p");
    let g = read_biguint(stream).expect("Failed to read g");
    let pub_key = read_biguint(stream).expect("Failed to read client's public key");

    let aes_key = BigUint::from_bytes_be(&header.aes_key);

    // Encrypt the AES key with client's public key
    let (ciphertext, ephemeral_pk) = ElGamal_encrypt(&aes_key, &pub_key, &p, &g);

    // Send encrypted AES key to server
    write_biguint(stream, &ciphertext).expect("Failed to send ciphertext");
    write_biguint(stream, &ephemeral_pk).expect("Failed to send ephemeral_pk");

    println!("{:?}", stream);
}