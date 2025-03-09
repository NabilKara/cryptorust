use std::io;

const ALPHABET: &str = "ABCDEFGHIKLMNOPQRSTUVWXYZ"; // 'J' is merged with 'I'

fn generate_matrix(key: &str) -> [[char; 5]; 5] {
    let mut matrix = [[' '; 5]; 5];
    let key_clean = key.to_uppercase().replace("J", "I").replace(" ", "");
    
    let mut used_letters = [false; 26];
    let mut row = 0;
    let mut col = 0;

    // add the key to matrix
    for c in key_clean.chars() {
        let idx = (c as u8 - b'A') as usize;
        if !used_letters[idx] {
            matrix[row][col] = c;
            used_letters[idx] = true;
            col += 1;
            if col == 5 {
                col = 0;
                row += 1;
            }
        }
    }

    // Add remaining alphabet
    for c in ALPHABET.chars() {
        let idx = (c as u8 - b'A') as usize;
        if !used_letters[idx] {
            matrix[row][col] = c;
            used_letters[idx] = true;
            col += 1;
            if col == 5 {
                col = 0;
                row += 1;
            }
        }
    }

    matrix
}

fn prepare_text(plaintext: &str) -> String {
    let mut prepared = String::new();
    let text_clean = plaintext.to_uppercase().replace("J", "I").replace(" ", "");
    let mut chars = text_clean.chars().peekable();
    while let Some(a) = chars.next() {
        prepared.push(a);
        match chars.peek() {
            Some(&b) if b == a => {
                prepared.push('X');
            }
            Some(&b) => {
                prepared.push(b);
                chars.next();
            }
            None => {
                prepared.push('X');
            }
        }
    }

    prepared
}

fn find_position(c: char, matrix: &[[char; 5]; 5]) -> (usize, usize) {
    for (row_idx, row) in matrix.iter().enumerate() {
        for (col_idx, &val) in row.iter().enumerate() {
            if val == c {
                return (row_idx, col_idx);
            }
        }
    }
    panic!("Character not found in matrix: {}", c);
}

pub fn encrypt_playfair(plaintext: &str, key: &str) -> String {
    let matrix = generate_matrix(key);
    let prepared = prepare_text(plaintext);
    
    let mut ciphertext = String::new();
    for pair in prepared.chars().collect::<Vec<_>>().chunks(2) {
        let (a, b) = (pair[0], pair[1]);
        let (ra, ca) = find_position(a, &matrix);
        let (rb, cb) = find_position(b, &matrix);

        let chars = match (ra == rb, ca == cb) {
            (true, _) => [
                matrix[ra][(ca + 1) % 5],
                matrix[rb][(cb + 1) % 5],
            ],
            (_, true) => [
                matrix[(ra + 1) % 5][ca],
                matrix[(rb + 1) % 5][cb],
            ],
            _ => [
                matrix[ra][cb],
                matrix[rb][ca],
            ],
        };
        ciphertext.push_str(&chars.iter().collect::<String>());
    }

    ciphertext
}

pub fn decrypt_playfair(ciphertext: &str, key: &str) -> String {
    let matrix = generate_matrix(key);
    let text_clean = ciphertext.to_uppercase()
        .replace("J", "I")
        .replace(" ", "");
    
    let mut decrypted = String::new();
    for pair in text_clean.chars().collect::<Vec<_>>().chunks(2) {
        let (a, b) = (pair[0], pair[1]);
        let (ra, ca) = find_position(a, &matrix);
        let (rb, cb) = find_position(b, &matrix);

        let chars = match (ra == rb, ca == cb) {
            (true, _) => [
                matrix[ra][(ca + 4) % 5],
                matrix[rb][(cb + 4) % 5],
            ],
            (_, true) => [
                matrix[(ra + 4) % 5][ca],
                matrix[(rb + 4) % 5][cb],
            ],
            _ => [
                matrix[ra][cb],
                matrix[rb][ca],
            ],
        };
        decrypted.push_str(&chars.iter().collect::<String>());
    }

    // remove paddng X
    let mut processed = String::new();
    let mut chars = decrypted.chars().peekable();
    
    while let Some(c) = chars.next() {
        processed.push(c);
        if let Some(&next) = chars.peek() {
            if next == 'X' && c == chars.clone().nth(1).unwrap_or(' ') {
                chars.next();
            }
        }
    }

    if processed.len() % 2 == 1 && processed.ends_with('X') {
        processed.pop();
    }

    processed
}

fn display_matrix(matrix: &[[char; 5]; 5]) {
    println!("\nPlayfair Matrix:");
    for row in matrix {
        for &c in row {
            print!("{} ", c);
        }
        println!();
    }
}

fn main() {
    let mut key = String::new();
    let mut plaintext = String::new();

    println!("Enter the key:");
    io::stdin().read_line(&mut key).expect("Failed to read key");
    let key = key.trim();

    println!("Enter the plaintext:");
    io::stdin().read_line(&mut plaintext).expect("Failed to read plaintext");
    let plaintext = plaintext.trim();

    let matrix = generate_matrix(key);
    display_matrix(&matrix);

    let ciphertext = encrypt_playfair(plaintext, key);
    let decrypted = decrypt_playfair(&ciphertext, key);
    
    println!("\nOriginal: {}", plaintext);
    println!("Encrypted: {}", ciphertext);
    println!("Decrypted: {}", decrypted);
}