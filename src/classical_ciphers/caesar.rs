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