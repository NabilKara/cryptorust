use std::io;
use std::io::{Read, Write};
use std::net::TcpStream;

pub fn start_chat(peer: TcpStream, key: [u8; 16], isServer: bool) {
    // Spawn a thread for receiving messages
    let mut peer_recv = peer.try_clone().expect("Failed to clone peer");
    let mut peer_send = peer.try_clone().expect("Failed to clone peer");
    std::thread::spawn(move || {
        loop {
            let mut buffer = [0u8; 1024];
            match peer_recv.read(&mut buffer) {
                Ok(msg) => {},
                Err(e) => {
                    eprintln!("Error receiving message: {}", e);
                    break;
                }
            }
            println!("{}:{}> {}", peer.local_addr().unwrap().ip(), peer.local_addr().unwrap().port(), String::from_utf8_lossy(&buffer))

        }
    });

    // Main thread handles sending
    loop {
        let mut input = String::new();

        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if peer_send.write(input.as_bytes()).is_err() {
            eprintln!("Failed to send message");
            break;
        }
    }
}
