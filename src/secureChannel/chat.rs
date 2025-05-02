use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::thread;

struct Message {
    IV: String,
    Data: String,
}

pub fn chat_loop(stream: TcpStream, key: [u8; 16], is_server: bool) {
    // Get peer address once at start
    let peer_addr = stream.peer_addr().unwrap();
    let local_addr = stream.local_addr().unwrap();

    println!("\nChat session established with {} (AES key: {:?})", peer_addr, key);
    println!("Type your messages below (Ctrl+C to exit)\n");
    
    let mut input = String::new();

    // Spawn receiver thread
    let mut recv_stream = stream.try_clone().expect("Failed to clone stream");
    thread::spawn(move || {
        let mut buffer = [0u8; 1024];
        loop {
            match recv_stream.read(&mut buffer) {
                Ok(n) if n > 0 => {
                    print!("\x1B[2K\r"); // Clear current line
                    let msg = String::from_utf8_lossy(&buffer[..n]);
                    print!("{}:{}> {}\n{}:{}> ", 
                           peer_addr.ip(), peer_addr.port(),
                           msg,
                           local_addr.ip(), local_addr.port(),
                    );
                    
                    io::stdout().flush().unwrap();
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
                if send_stream.write_all(input.as_bytes()).is_err() {
                    eprintln!("Failed to send message");
                    break;
                }
                input.clear();
                break;
            }
        }
        // match io::stdin().read_line(&mut input) {
        //     Ok(_) => {
        //         let input = input.trim();
        //         if input.is_empty() { continue; }
        // 
        //         if send_stream.write_all(input.as_bytes()).is_err() {
        //             eprintln!("Failed to send message");
        //             break;
        //         }
        //     }
        //     Err(e) => {
        //         eprintln!("Input error: {}", e);
        //         break;
        //     }
        // }
    }
}