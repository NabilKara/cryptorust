use std::collections::HashMap;

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

fn printMenu(){
    println!("in frequency_analysis");
}

pub fn Menu(PATH: &mut String) -> u8 {
    PATH.push_str("frequency_analysis/");
    printMenu();
    return 1;
}