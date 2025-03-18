use num::pow;

const BLOCK_SIZE: usize = 8;
const HALF_SIZE: usize = BLOCK_SIZE / 2;
const EXPANDED_SIZE: usize = HALF_SIZE + HALF_SIZE / 2;
const ip_l: [usize; 32] = [
    58, 50, 42, 34, 26, 18, 10, 2,
    60, 52, 44, 36, 28, 20, 12, 4,
    62, 54, 46, 38, 30, 22, 14, 6,
    64, 56, 48, 40, 32, 24, 16, 8,
];
const ip_r: [usize; 32] = [
    57, 49, 41, 33, 25, 17, 9, 1,
    59, 51, 43, 35, 27, 19, 11, 3,
    61, 53, 45, 37, 29, 21, 13, 5,
    63, 55, 47, 39, 31, 23, 15, 7,
];

const expansion_table: [usize; 48] = [
    32,  1,  2,  3,  4,  5,
    4,  5,  6,  7,  8,  9,
    8,  9, 10, 11, 12, 13,
    12, 13, 14, 15, 16, 17,
    16, 17, 18, 19, 20, 21,
    20, 21, 22, 23, 24, 25,
    24, 25, 26, 27, 28, 29,
    28, 29, 30, 31, 32,  1,
];

// 'pos' from 1 .. 64
pub fn setBit_64(num: u64, pos: usize) -> u64 { num | pow(2, pos - 1) }
pub fn setBit_32(num: u32, pos: usize) -> u32 { num | pow(2, pos - 1) }
pub fn getBit_64(num: u64, pos: usize) -> u64 { if num & pow(2, pos - 1) != 0 { 1 } else { 0 } }
pub fn getBit_32(num: u32, pos: usize) -> u32 { if num & pow(2, pos - 1) != 0 { 1 } else { 0 } }

pub fn convert8to32(nums: &[u8; HALF_SIZE]) -> u32 {
    (nums[0] as u32) << 24 |
    (nums[1] as u32) << 16 |
    (nums[2] as u32) << 8  |
    (nums[3] as u32) << 0
}

pub fn convert32to8(nums: u32) -> [u8; HALF_SIZE] {
    [
        ((nums & 0xff000000) >> 24) as u8,
        ((nums & 0x00ff0000) >> 16) as u8,
        ((nums & 0x0000ff00) >>  8) as u8,
        ((nums & 0x000000ff) >>  0) as u8,
    ]
}

pub fn convert8to64(nums: &[u8; BLOCK_SIZE]) -> u64 {
    (nums[0] as u64) << 56 |
    (nums[1] as u64) << 48 |
    (nums[2] as u64) << 40 |
    (nums[3] as u64) << 32 |
    (nums[4] as u64) << 24 |
    (nums[5] as u64) << 16 |
    (nums[6] as u64) << 8  |
    (nums[7] as u64) << 0
}

pub fn convert64to8(nums: u64) -> [u8; BLOCK_SIZE] {
    [
        ((nums & 0xff00000000000000) >> 56) as u8,
        ((nums & 0x00ff000000000000) >> 48) as u8,
        ((nums & 0x0000ff0000000000) >> 40) as u8,
        ((nums & 0x000000ff00000000) >> 32) as u8,
        ((nums & 0x00000000ff000000) >> 24) as u8,
        ((nums & 0x0000000000ff0000) >> 16) as u8,
        ((nums & 0x000000000000ff00) >>  8) as u8,
        ((nums & 0x00000000000000ff) >>  0) as u8,
    ]
}

pub fn reverse_bits_u64(num: u64) -> u64 {
    let mut reversed = 0u64;
    for i in 0..64 {
        // Shift the current bit to its reversed position
        reversed |= ((num >> i) & 1) << (63 - i);
    }
    reversed
}

// Arrays' elements must be of value 1..64; not 0..63
fn permute(input: u64, l_arr: &[usize; 32], r_arr: &[usize; 32]) -> u64 {
    let mut rslt: u64 = 0;

    println!("---------------------------------- Initial Permutation -----------------------------------");
    println!("src: {:064b}", input);

    for i in 00..32 { rslt |= getBit_64(input, l_arr[i]) << (63 - i); }
    for i in 32..64 { rslt |= getBit_64(input, r_arr[i - 32]) << (63 - i); }

    println!("dst: {:064b}", rslt);
    println!("------------------------------------------------------------------------------------------");

    rslt
}

pub fn initialPermutation(input: u64) -> u64 { permute(input, &ip_l, &ip_r) }

pub fn expansion(input: u32) -> u64 {
    let mut rslt: u64 = 0;

    println!("--------------------------------- Expansive Permutation ----------------------------------");
    println!("src: {}{:032b}", [" "; 32].join("") ,input);

    for i in 0..expansion_table.len() { rslt |= (getBit_32(input, expansion_table[i]) as u64) << i; }
    
    println!("dst: {}{:048b}", [" "; 16].join(""), rslt);
    println!("------------------------------------------------------------------------------------------");

    rslt
}

pub fn encrypt_block(block: &[u8; BLOCK_SIZE], key: &[u8; EXPANDED_SIZE]) -> [u8; BLOCK_SIZE] {
    let rslt = [0x00; BLOCK_SIZE];
    let key = [0, 0, key[0], key[1], key[2], key[3], key[4], key[5]];
    let mut data = convert8to64(block);
    data = reverse_bits_u64(data);
    data = initialPermutation(data);

    let LPT: u32 = (data >> 32) as u32;
    let RPT: u32 = (data & 0xFFFFFFFF) as u32;
    let mut eRPT = expansion(RPT);
    let numKey = convert8to64(&key);

    println!("---------------------------------------- XOR Key -----------------------------------------");
    println!("src: {}{:048b}", [" "; 16].join(""), eRPT);
    eRPT ^= numKey;
    println!("dst: {}{:048b}", [" "; 16].join(""), eRPT);
    println!("------------------------------------------------------------------------------------------");
    
    rslt
}