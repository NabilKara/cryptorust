#[cfg(test)]
mod tests {
    use crate::classical_ciphers::caesar::{decrypt_caesar, encrypt_caesar};
    use crate::classical_ciphers::one_time_pad::{decrypt_otp, encrypt_otp};
    use crate::classical_ciphers::vigenere::{decrypt_vigenere, encrypt_vigenere};
    #[test]
    fn test_caesar_cipher() {
        assert_eq!(encrypt_caesar("HELLO", 3), "KHOOR");
        assert_eq!(decrypt_caesar("KHOOR", 3), "HELLO");
    }
    #[test]
    fn test_vigenere_cipher() {
        assert_eq!(encrypt_vigenere("HELLO WORLD", "KEY"), "RIJVS UYVJN", "Encryption failed!");
        assert_eq!(decrypt_vigenere("RIJVS UYVJN","KEY"),"HELLO WORLD", "Encryption failed!");
    }
    #[test]
    fn test_otp_encryption_decryption() {
        let plaintext = "HELLO WORLD";
        let (ciphertext, key) = encrypt_otp(plaintext);
        let decrypted = decrypt_otp(&ciphertext, &key);

        assert_eq!(decrypted, plaintext, "Decryption should return the original plaintext");
        assert_ne!(ciphertext, plaintext, "Ciphertext should not be the same as plaintext");
        assert_ne!(ciphertext, key, "Ciphertext should not be the same as the key");
    }
}