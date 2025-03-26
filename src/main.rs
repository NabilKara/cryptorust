#![allow(nonstandard_style)]
#![allow(dead_code)]

use num::Integer;

mod classical_ciphers;
mod menu;
mod symmetric_encryption;

fn printMenu(){
    println!("PLease choose an option:");
    println!("1- Ciphers");
    println!("2- Encryption Systems");
    println!("3- Help");
    println!("4- Quit");
}


fn main() {
    let _cleartext = [0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88];
    let _key = [0x75, 0x28, 0x78, 0x39, 0x74, 0x93, 0xCB, 0x70];
    let cleartext: Vec<u8> = _cleartext.to_vec();
    let ciphertext = symmetric_encryption::des::encrypt::encryptECB(cleartext.clone(), &_key);
    let mut rslt = Vec::new();

    print!("Cleartext: ");      outputBytes(_cleartext.to_vec());
    print!("key: ");            outputBytes(_key.to_vec());
    println!("------------------------------------------------------------------------");

    for i in 0..ciphertext.len() {
        rslt.extend(ciphertext[i]);
    }

    print!("ECB Encrypt: "); outputBytes(rslt.clone());

    let decrypted_rslt = symmetric_encryption::des::decrypt::decryptECB(rslt, &_key);
    let iv = [0x61u8, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68];

    print!("ECB Decrypt: "); outputBytes(decrypted_rslt.clone());

    println!("------------------------------------------------------------------------");

    let ciphertext = symmetric_encryption::des::encrypt::encryptCBC(cleartext, &iv, &_key);
    let mut rslt = Vec::new();

    for i in 0..ciphertext.len() {
        rslt.extend(ciphertext[i]);
    }

    print!("CBC Encrypt: "); outputBytes(rslt.clone());

    let decrypted_rslt = symmetric_encryption::des::decrypt::decryptCBC(rslt, &iv, &_key);

    print!("CBC Decrypt: "); outputBytes(decrypted_rslt.clone());
}

fn outputBytes(buf: Vec<u8>) {
    if buf.len().is_even() {
        for i in (0..buf.len() - 1).step_by(2) { print!("{:02x}{:02x} ", buf[i], buf[i+1]);/* printing in big endian order, swap endianness to verify with openssl command */}
    }
    else {
        for i in (0..buf.len() - 2).step_by(2) {
            print!("{:02x}{:02x} ", buf[i], buf[i+1]);
        }
        print!("{:02x} ", buf[buf.len() - 1]);
    }
    println!();
}