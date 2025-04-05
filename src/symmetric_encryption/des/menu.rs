use std::io;
use std::io::Write;
use crate::symmetric_encryption::des::encrypt::{encryptCBC, encryptECB};
use crate::symmetric_encryption::des::decrypt::{decryptCBC, decryptECB};
use super::super::menu::{outputBytes, parseBytes};

fn printModesMenu(){
    println!("Choose mode:");
    println!("1- ECB");
    println!("2- CBC");
    println!("3- Return");
}

pub fn Menu(PATH: &mut String) -> usize {
    let mut buf = String::new();
    let mut key = String::new();
    let mut iv = String::new();
    let mut iv_arr: [u8; 8] = [0; 8];
    let r;
    let m;
    const PREFIX: &str = "DES/";

    PATH.push_str(PREFIX);
    r = super::getGenericOption(PATH.clone());
    if r == 3 {
        PATH.drain(PATH.len() - PREFIX.len()..);
        return 1;
    }

    printModesMenu();
    m = super::getInput(PATH.clone(), 1, 3);
    if m == 3 {
        PATH.drain(PATH.len() - PREFIX.len()..);
        return 1;
    }

    print!("Enter text as series of hex (ABCDE12...): ");               io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buf).expect("Failed to read plaintext");
    let buf = parseBytes(buf);

    print!("Enter key as series of hex (ABCDE12...): ");                  io::stdout().flush().unwrap();
    io::stdin().read_line(&mut key).expect("Failed to read key");
    let key: [u8; 8] = match parseBytes(key).try_into() {
        Ok(k) => k,
        Err(t) => panic!("Key length must be 8 bytes but got '{}'.", t.len()),
    };

    if m == 2 {
        print!("Enter IV as series of hex (ABCDE12...): ");               io::stdout().flush().unwrap();
        io::stdin().read_line(&mut iv).expect("Failed to read plaintext");
        iv_arr = match parseBytes(iv).try_into() {
            Ok(k) => k,
            Err(t) => panic!("IV length must be 8 bytes but got '{}'.", t.len()),
        };
    }

    let output = match r {
        1 =>  match m {
            1 => encryptECB(buf, &key),
            2 => encryptCBC(buf, &iv_arr, &key),
            _ => panic!("Undefined mode for Encryption."),
        },
        2 => match m {
            1 => decryptECB(buf, &key),
            2 => decryptCBC(buf, &iv_arr, &key),
            _ => panic!("Undefined mode"),
        },
        _ => panic!("Undefined Encryption/Decryption choice."),
    };

    print!("\nResult: ");       outputBytes(output);

    PATH.drain(PATH.len() - PREFIX.len()..);
    0
}