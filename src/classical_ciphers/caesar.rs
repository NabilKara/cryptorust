use std::io;
use std::io::Write;

const ALPHABET_SIZE: u8 = 26;
pub fn encrypt_caesar(plaintext: &str, shift: u8) -> String {
    plaintext.chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_lowercase() {b'a'} else {b'A'};
                let offset = ((c as u8) - base + shift)  % ALPHABET_SIZE;
                (base + offset) as char
            } else {
                c
            }
        })
        .collect()
}
pub fn decrypt_caesar(plaintext: &str,shift: u8) -> String {
    encrypt_caesar(plaintext, ALPHABET_SIZE - (shift % ALPHABET_SIZE))
}

fn printMenu(){
    println!("What do you want to do?");
    println!("1- Encrypt");
    println!("2- Decrypt");
    println!("3- Return");
}

pub fn Menu(PATH: &mut String) -> u8 {
    let mut buf = String::new();
    let mut shift = String::new();

    PATH.push_str("caesar_cipher/");
    printMenu();
    let r = super::getInput(PATH.clone(), 1, 3);
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    match r {
        3 => return 1,
        _ => {}
    }

    print!("Enter text: ");               io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buf).unwrap();

    print!("Enter size of shift:");       io::stdout().flush().unwrap();
    io::stdin().read_line(&mut shift).unwrap();
    let shift = shift.trim().parse::<u8>().unwrap();

    match r {
        1 => buf = encrypt_caesar(buf.as_str(), shift),
        2 => buf = decrypt_caesar(buf.as_str(), shift),
        _ => {}
    }

    println!("\nResult: {buf}");
    return 0;
}