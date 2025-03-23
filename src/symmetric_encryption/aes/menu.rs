use std::io;
use std::io::Write;
use crate::symmetric_encryption::aes::aes_decryption::{decrypt_cbc, decrypt_ecb};
use crate::symmetric_encryption::aes::aes_encryption::{encrypt_cbc, encrypt_ecb};

fn printModesMenu(){
    println!("Choose mode:");
    println!("1- ECB");
    println!("2- CBC");
    println!("3- Return");
}

fn parseBytes(buf: String) -> Vec<u8> {
    let mut rslt = Vec::new();
    let mut buffer: String;
    if buf.len() % 2 == 1 {
        buffer = String::from("0");
        buffer.push_str(buf.as_str().trim());
    }
    else { buffer = buf.clone(); }
    buffer = buffer.trim().to_string();

    for i in (0..buffer.len() - 1).step_by(2) {
        let numStr = &buf[i..i+2];
        rslt.push(
            u8::from_str_radix(numStr, 16)
                .expect(format!("Invalid hex sequence '{}'.", numStr).as_str())
        );
    }

    rslt
}

fn outputBytes(buf: Vec<u8>) {
    for i in (0..buf.len() - 1).step_by(2) { print!("{:02x}{:02x} ", buf[i], buf[i+1]); /* printing in big endian order, swap endianness to verify with openssl command */}
}

pub fn Menu(PATH: &mut String) -> u8 {
    let mut buf = String::new();
    let mut key = String::new();
    let mut iv = String::new();
    let mut iv_arr: [u8; 16] = [0; 16];
    let r;
    let m;
    const PREFIX: &str = "AES/";

    PATH.push_str(PREFIX);
    r = super::getGenericOption(PATH.clone());
    if r == 3 {
        PATH.drain(PATH.len() - PREFIX.len()..);
        return 1;
    }

    printModesMenu();
    m = super::getInput(PATH.clone(), 1, 2);
    if m == 3 {
        PATH.drain(PATH.len() - PREFIX.len()..);
        return 1;
    }

    print!("Enter text as series of hex (ABCDE12...): ");               io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buf).expect("Failed to read plaintext");
    let buf = parseBytes(buf);

    print!("Enter key as series of hex (ABCDE12...): ");                  io::stdout().flush().unwrap();
    io::stdin().read_line(&mut key).expect("Failed to read key");
    let key: [u8; 16] = match parseBytes(key).try_into() {
        Ok(k) => k,
        Err(t) => panic!("Key length must be 16 but got '{}'.", t.len()),
    };

    if m == 2 {
        print!("Enter IV as series of hex (ABCDE12...): ");               io::stdout().flush().unwrap();
        io::stdin().read_line(&mut iv).expect("Failed to read plaintext");
        iv_arr = match parseBytes(iv).try_into() {
            Ok(k) => k,
            Err(t) => panic!("IV length must be 16 but got '{}'.", t.len()),
        };
    }

    let output = match r {
        1 => match m {
                1 => encrypt_ecb(buf, &key),
                2 => encrypt_cbc(&buf, &iv_arr, &key),
                _ => panic!("Undefined mode for Encryption."),
            },
        2 => match m {
                1 => decrypt_ecb(&buf, &key),
                2 => decrypt_cbc(buf, &iv_arr, &key),
                _ => panic!("Undefined mode"),
            }.expect("Undefined mode for Decryption."),
        _ => panic!("Undefined Encryption/Decryption choice."),
    };

    print!("\nResult: ");       outputBytes(output);        println!();
    
    PATH.drain(PATH.len() - PREFIX.len()..);
    0
}