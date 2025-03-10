use num::integer::{gcd};
use modinverse::modinverse;
const ALPHABET_SIZE: u8 = 26;
pub fn encrypt_affine(plaintext: &str, a : u8, b : u8) -> String {
    if gcd(a, ALPHABET_SIZE) != 1 {
        panic!("Key `a` must be coprime with the alphabet size (26).");
    }
    plaintext.chars().map(|c| {
        if c.is_ascii_alphabetic(){
            let x = c.to_ascii_uppercase() as u8 - 'A' as u8;
            let y = (a * x + b) % ALPHABET_SIZE as u8;
            (y + 'A' as u8) as char
        }else{
            c
        }
    }).collect()
}
pub fn decrypt_affine(ciphertext: &str, a : u8, b : u8) -> String {
    if gcd(a, ALPHABET_SIZE) != 1 {
        panic!("Key `a` must be coprime with the alphabet size (26).");
    }
    let a_inv = modinverse(a as i64, ALPHABET_SIZE as i64).unwrap() as u16;
    ciphertext.chars().map(|c| {
        if c.is_ascii_alphabetic(){
            let y = c.to_ascii_uppercase() as u16 - 'A' as u16;
            let x = (((y + ALPHABET_SIZE  as u16 - b as u16 ) * a_inv) % ALPHABET_SIZE as u16) as u8;
            (x + 'A' as u8) as char
        }else{
            c
        }
    }).collect()
}