use std::collections::HashMap;

pub fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

pub fn gcd_list(numbers: &Vec<usize>) -> usize {
    numbers.iter().cloned().reduce(|a, b| gcd(a, b)).unwrap_or(1)
}

pub fn find_repeated_sequences(ciphertext: &str, seq_length: usize) -> Vec<usize> {
    let mut sequences: HashMap<&str, Vec<usize>> = HashMap::new();
    let mut distances = Vec::new();
    for i in 0..=ciphertext.len().saturating_sub(seq_length) {
        let seq = &ciphertext[i..i + seq_length];

        if let Some(positions) = sequences.get_mut(seq) {
            positions.push(i);
        } else {
            sequences.insert(seq, vec![i]);
        }
    }

    for positions in sequences.values() {
        if positions.len() > 1 {
            for j in 0..positions.len() - 1 {
                let distance = positions[j + 1] - positions[j];
                distances.push(distance);
            }
        }
    }

    distances
}

pub fn kasiski_examination(ciphertext: &str) -> Option<usize> {
    let seq_length = 3; // trigrams
    let distances = find_repeated_sequences(ciphertext, seq_length);

    if distances.is_empty() {
        None
    } else {
        Some(gcd_list(&distances))
    }
}

