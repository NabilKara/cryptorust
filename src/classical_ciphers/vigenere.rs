use crate::classical_ciphers::caesar::{encrypt_caesar, decrypt_caesar};

pub fn encrypt_vigenere(plaintext: &str, key: &str) -> String {
    let key_bytes = key.as_bytes();
    let key_len = key_bytes.len();
    let mut i = 0;

    plaintext.chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let key_char = key_bytes[i % key_len];
                let shift = if key_char.is_ascii_lowercase() {
                    key_char - b'a'
                } else {
                    key_char - b'A'
                };
                let encrypted_char = encrypt_caesar(&c.to_string(), shift);
                i += 1;
                encrypted_char.chars().next().unwrap()
            } else {
                c
            }
        })
        .collect()
}
pub fn decrypt_vigenere(ciphertext: &str, key: &str) -> String {
    let key_bytes = key.as_bytes();
    let key_len = key_bytes.len();
    let mut i = 0;

    ciphertext.chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let key_char = key_bytes[i % key_len];
                let shift = if key_char.is_ascii_lowercase() {
                    key_char - b'a'
                } else {
                    key_char - b'A'
                };
                let encrypted_char = decrypt_caesar(&c.to_string(), shift);
                i += 1;
                encrypted_char.chars().next().unwrap()
            } else {
                c
            }
        })
        .collect()
}