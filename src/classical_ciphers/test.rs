#[cfg(test)]
mod tests {
    use crate::classical_ciphers::caesar::{decrypt_caesar, encrypt_caesar};
    use crate::classical_ciphers::frequency_analysis::frequency_counter;
    use crate::classical_ciphers::one_time_pad::{decrypt_otp, encrypt_otp};
    use crate::classical_ciphers::random_substitution::{decrypt_random_substitution, encrypt_random_substitution, generate_random_substitution_key};
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
    fn test_otp_cipher() {
        let plaintext = "HELLO WORLD";
        let (ciphertext, key) = encrypt_otp(plaintext);
        let decrypted = decrypt_otp(&ciphertext, &key);

        assert_eq!(decrypted, plaintext, "Decryption should return the original plaintext");
        assert_ne!(ciphertext, plaintext, "Ciphertext should not be the same as plaintext");
        assert_ne!(ciphertext, key, "Ciphertext should not be the same as the key");
    }
    #[test]
    fn test_random_substitution_cipher() {
        let key = generate_random_substitution_key();
        let plaintext = "HELLO WORLD";
        assert_eq!(decrypt_random_substitution(&encrypt_random_substitution(plaintext, &key), &key), plaintext);
    }
    #[test]
    fn test_frequency_analysis(){
        let ciphertext = "HELLO WORLD";
        let freq_map = frequency_counter(ciphertext);

        assert_eq!(freq_map[&'H'], 1);
        assert_eq!(freq_map[&'E'], 1);
        assert_eq!(freq_map[&'L'], 3);
        assert_eq!(freq_map[&'O'], 2);
        assert_eq!(freq_map[&'W'], 1);
        assert_eq!(freq_map[&'R'], 1);
        assert_eq!(freq_map[&'D'], 1);

        assert_eq!(freq_map[&'X'], 0);
        assert_eq!(freq_map[&'Z'], 0);
    }
}