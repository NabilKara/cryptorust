const BLOCK_SIZE: usize = 64;
const HALF_SIZE: usize = BLOCK_SIZE / 2;
const EXPANDED_SIZE: usize = HALF_SIZE + HALF_SIZE / 2;
const ip_l: [u8; 32] = [
    58, 50, 42, 34, 26, 18, 10, 2,
    60, 52, 44, 36, 28, 20, 12, 4,
    62, 54, 46, 38, 30, 22, 14, 6,
    64, 56, 48, 40, 32, 24, 16, 8,
];
const ip_r: [u8; 32] = [
    57, 49, 41, 33, 25, 17, 9, 1,
    59, 51, 43, 35, 27, 19, 11, 3,
    61, 53, 45, 37, 29, 21, 13, 5,
    63, 55, 47, 39, 31, 23, 15, 7,
];

const expansion_table: [u8; 48] = [
    32,  1,  2,  3,  4,  5,
     4,  5,  6,  7,  8,  9,
     8,  9, 10, 11, 12, 13,
    12, 13, 14, 15, 16, 17,
    16, 17, 18, 19, 20, 21,
    20, 21, 22, 23, 24, 25,
    24, 25, 26, 27, 28, 29,
    28, 29, 30, 31, 32,  1,
];

// Arrays' elements must be of value 1..64; not 0..63
fn permute(input: &[u8], l_arr: &[u8; 32], r_arr: &[u8; 32]) -> [u8; BLOCK_SIZE] {
    let mut rslt = [0x00; BLOCK_SIZE];
    for i in 0..32  { rslt[i] = input[(l_arr[i] - 1) as usize]; }
    for i in 32..64 { rslt[i] = input[(r_arr[i - 32] - 1) as usize]; }

    rslt
}

pub fn initialPermutation(input: &[u8; BLOCK_SIZE]) -> [u8; BLOCK_SIZE] {
    permute(input, &ip_l, &ip_r)
}

pub fn expansion(input: &[u8; HALF_SIZE]) -> [u8; EXPANDED_SIZE] {
    let mut rslt = [0x00; EXPANDED_SIZE];
    for i in 0..rslt.len() {
        rslt[i] = input[(expansion_table[i] - 1) as usize];
    }
    rslt
}

fn xor_48(input: &mut [u8; 48], key: &[u8; 48]){
    for i in 0..48 { input[i] = input[i] ^ key[i]; }
}
    // NEED TO ASK TEACHER IF WE'LL USE BLOCKS OF 64-BITS (8 BYTES), OR 64 BYTES FOR SIMPLIFICATION AND STRING-STRING INPUT-OUTPUT
fn encrypt_block(block: &[u8; BLOCK_SIZE], key: &[u8; EXPANDED_SIZE]) -> [u8; BLOCK_SIZE] {
    let rslt = [0x00; BLOCK_SIZE];
    let permuted_data = initialPermutation(block);
    let l0: [u8; 32] = permuted_data[00..32].try_into().unwrap();
    let r0: [u8; 32] = permuted_data[32..64].try_into().unwrap();        let mut expanded_r0 = expansion(&r0);

    xor_48(&mut expanded_r0, key);
    
    rslt
}