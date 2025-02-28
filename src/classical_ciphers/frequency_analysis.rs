use std::collections::HashMap;
use crate::classical_ciphers::index_of_coincidence::index_of_coincidence_counter;

pub fn frequency_counter(ciphertext: &str) -> HashMap<char, usize> {
    let mut freq_table = HashMap::new();
    for  c in 'A'..='Z' {
        freq_table.insert(c, 0);
    }
    for c in ciphertext.chars() {
        if let Some(freq) = freq_table.get_mut(&c) {
            *freq += 1;
        }
    }
    freq_table
}