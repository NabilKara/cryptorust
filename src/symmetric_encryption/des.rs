use std::fmt::format;
use std::mem::swap;
use num::pow;

const BLOCK_SIZE: usize = 8;
const HALF_SIZE: usize = BLOCK_SIZE / 2;
const EXPANDED_SIZE: usize = HALF_SIZE + HALF_SIZE / 2;
const IP: [usize; 64] = [
    58, 50, 42, 34, 26, 18, 10, 2,
    60, 52, 44, 36, 28, 20, 12, 4,
    62, 54, 46, 38, 30, 22, 14, 6,
    64, 56, 48, 40, 32, 24, 16, 8,
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

const S_BOXES: [[[usize; 16]; 4]; 8] = [
    // S-Box 1
    [
        [14, 4, 13, 1, 2, 15, 11, 8, 3, 10, 6, 12, 5, 9, 0, 7],
        [0, 15, 7, 4, 14, 2, 13, 1, 10, 6, 12, 11, 9, 5, 3, 8],
        [4, 1, 14, 8, 13, 6, 2, 11, 15, 12, 9, 7, 3, 10, 5, 0],
        [15, 12, 8, 2, 4, 9, 1, 7, 5, 11, 3, 14, 10, 0, 6, 13],
    ],
    // S-Box 2
    [
        [15, 1, 8, 14, 6, 11, 3, 4, 9, 7, 2, 13, 12, 0, 5, 10],
        [3, 13, 4, 7, 15, 2, 8, 14, 12, 0, 1, 10, 6, 9, 11, 5],
        [0, 14, 7, 11, 10, 4, 13, 1, 5, 8, 12, 6, 9, 3, 2, 15],
        [13, 8, 10, 1, 3, 15, 4, 2, 11, 6, 7, 12, 0, 5, 14, 9],
    ],
    // S-Box 3
    [
        [10, 0, 9, 14, 6, 3, 15, 5, 1, 13, 12, 7, 11, 4, 2, 8],
        [13, 7, 0, 9, 3, 4, 6, 10, 2, 8, 5, 14, 12, 11, 15, 1],
        [13, 6, 4, 9, 8, 15, 3, 0, 11, 1, 2, 12, 5, 10, 14, 7],
        [1, 10, 13, 0, 6, 9, 8, 7, 4, 15, 14, 3, 11, 5, 2, 12],
    ],
    // S-Box 4
    [
        [7, 13, 14, 3, 0, 6, 9, 10, 1, 2, 8, 5, 11, 12, 4, 15],
        [13, 8, 11, 5, 6, 15, 0, 3, 4, 7, 2, 12, 1, 10, 14, 9],
        [10, 6, 9, 0, 12, 11, 7, 13, 15, 1, 3, 14, 5, 2, 8, 4],
        [3, 15, 0, 6, 10, 1, 13, 8, 9, 4, 5, 11, 12, 7, 2, 14],
    ],
    // S-Box 5
    [
        [2, 12, 4, 1, 7, 10, 11, 6, 8, 5, 3, 15, 13, 0, 14, 9],
        [14, 11, 2, 12, 4, 7, 13, 1, 5, 0, 15, 10, 3, 9, 8, 6],
        [4, 2, 1, 11, 10, 13, 7, 8, 15, 9, 12, 5, 6, 3, 0, 14],
        [11, 8, 12, 7, 1, 14, 2, 13, 6, 15, 0, 9, 10, 4, 5, 3],
    ],
    // S-Box 6
    [
        [12, 1, 10, 15, 9, 2, 6, 8, 0, 13, 3, 4, 14, 7, 5, 11],
        [10, 15, 4, 2, 7, 12, 9, 5, 6, 1, 13, 14, 0, 11, 3, 8],
        [9, 14, 15, 5, 2, 8, 12, 3, 7, 0, 4, 10, 1, 13, 11, 6],
        [4, 3, 2, 12, 9, 5, 15, 10, 11, 14, 1, 7, 6, 0, 8, 13],
    ],
    // S-Box 7
    [
        [4, 11, 2, 14, 15, 0, 8, 13, 3, 12, 9, 7, 5, 10, 6, 1],
        [13, 0, 11, 7, 4, 9, 1, 10, 14, 3, 5, 12, 2, 15, 8, 6],
        [1, 4, 11, 13, 12, 3, 7, 14, 10, 15, 6, 8, 0, 5, 9, 2],
        [6, 11, 13, 8, 1, 4, 10, 7, 9, 5, 0, 15, 14, 2, 3, 12],
    ],
    // S-Box 8
    [
        [13, 2, 8, 4, 6, 15, 11, 1, 10, 9, 3, 14, 5, 0, 12, 7],
        [1, 15, 13, 8, 10, 3, 7, 4, 12, 5, 6, 11, 0, 14, 9, 2],
        [7, 11, 4, 1, 9, 12, 14, 2, 0, 6, 10, 13, 15, 3, 5, 8],
        [2, 1, 14, 7, 4, 10, 8, 13, 15, 12, 9, 0, 3, 5, 6, 11],
    ],
];

const P_BOX: [usize; 32] = [
    16,  7, 20, 21, 29, 12, 28, 17,
     1, 15, 23, 26,  5, 18, 31, 10,
     2,  8, 24, 14, 32, 27,  3,  9,
    19, 13, 30,  6, 22, 11,  4, 25,
];

const FINAL_PERMUTATION: [usize; 64] = [
    40, 8, 48, 16, 56, 24, 64, 32,
    39, 7, 47, 15, 55, 23, 63, 31,
    38, 6, 46, 14, 54, 22, 62, 30,
    37, 5, 45, 13, 53, 21, 61, 29,
    36, 4, 44, 12, 52, 20, 60, 28,
    35, 3, 43, 11, 51, 19, 59, 27,
    34, 2, 42, 10, 50, 18, 58, 26,
    33, 1, 41,  9, 49, 17, 57, 25,
];

// 56x56 Key permutation
const PC1: [usize; 56] = [
    57, 49, 41, 33, 25, 17,  9,
     1, 58, 50, 42, 34, 26, 18,
    10,  2, 59, 51, 43, 35, 27,
    19, 11,  3, 60, 52, 44, 36,
    63, 55, 47, 39, 31, 23, 15,
     7, 62, 54, 46, 38, 30, 22,
    14,  6, 61, 53, 45, 37, 29,
    21, 13,  5, 28, 20, 12,  4,
];

const KEY_ROTATIONS: [usize; 16] = [
    1, 1, 2, 2, 2, 2, 2, 2, 1, 2, 2, 2, 2, 2, 2, 1,
];

const PC2: [usize; 48] = [
    14, 17, 11, 24,  1,  5,  3, 28,
    15,  6, 21, 10, 23, 19, 12,  4,
    26,  8, 16,  7, 27, 20, 13,  2,
    41, 52, 31, 37, 47, 55, 30, 40,
    51, 45, 33, 48, 44, 49, 39, 56,
    34, 53, 46, 42, 50, 36, 29, 32,
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
fn permute_64(input: u64, arr: &[usize; 64]) -> u64 {
    let mut rslt: u64 = 0;

    for i in 00..64 { rslt |= getBit_64(input, arr[i]) << (63 - i); }

    rslt
}

fn permute_56(input: u64, arr: &[usize; 56]) -> u64 {
    let mut rslt: u64 = 0;

    for i in 00..56 { rslt |= getBit_64(input, arr[i]) << (55 - i); }

    rslt & 0xFFFFFFFFFFFFFF
}

fn permute_48(input: u64, arr: &[usize; 48]) -> u64 {
    let mut rslt: u64 = 0;

    for i in 00..48 { rslt |= getBit_64(input, arr[i]) << (47 - i); }

    rslt & 0xFFFFFFFFFFFF
}

fn permute_32(input: u32, arr: &[usize; 32]) -> u32 {
    let mut rslt: u32 = 0;

    for i in 00..32 { rslt |= getBit_32(input, arr[i]) << (31 - i); }

    rslt
}

pub fn initialPermutation(input: u64) -> u64 {
    println!("---------------------------------- Initial Permutation -----------------------------------");
    println!("src: {:064b}", input);

    let rslt = permute_64(input, &IP);

    println!("dst: {:064b}", rslt);
    println!("------------------------------------------------------------------------------------------");

    rslt
}

pub fn expansion(input: u32) -> u64 {
    let mut rslt: u64 = 0;

    println!("--------------------------------- Expansive Permutation ----------------------------------");
    println!("src: {}{:032b}", [" "; 32].join("") ,input);

    for i in 0..expansion_table.len() { rslt |= (getBit_32(input, expansion_table[i]) as u64) << i; }
    
    println!("dst: {}{:048b}", [" "; 16].join(""), rslt);
    println!("------------------------------------------------------------------------------------------");

    rslt
}

fn XOR_48(input: u64, key: u64) -> u64 {
    println!("---------------------------------------- XOR Key -----------------------------------------");
    println!("src: {}{:048b}", [" "; 16].join(""), input);
    let rslt = (input ^ key) & 0xFFFFFF;
    println!("dst: {}{:048b}", [" "; 16].join(""), rslt);
    println!("------------------------------------------------------------------------------------------");
    rslt
}

pub fn SBox(input: u64) -> u32 {
    let mut rslt = String::new();

    println!("--------------------------------- S-Box Transformation -----------------------------------");
    println!("src: {}{:048b}", [" "; 16].join("") ,input);
    let str_num = format!("{:048b}", input);

    for i in 0..8 {
        let slice = &str_num.as_str()[i * 6..i * 6 + 6];
        let slice = u8::from_str_radix(slice, 2).unwrap();
        let slice_rslt = format!("{:04b}", SBox_Transform(slice, &S_BOXES[i]));
        rslt.push_str(slice_rslt.as_str());

    }
    let rslt = u32::from_str_radix(rslt.as_str(), 2).unwrap();
    println!("dst: {}{:032b}", [" "; 32].join(""), rslt);
    println!("------------------------------------------------------------------------------------------");

    rslt
}

fn SBox_Transform(input: u8, table: &[[usize; 16]; 4]) -> u8 {
    let row = (((input & 0x20) >> 4) | input & 0x01) as usize;  // 1st & 6th bits
    let column = (input & 0x1E >> 1) as usize;                  // Remaining 4 bits

    (table[row][column] & 0x3F) as u8                                 // & with 6 bits for safety
}

fn PBox_Permutation(input: u32) -> u32 {
    println!("----------------------------------- P-Box Permutation ------------------------------------");
    println!("src: {}{:032b}", [" "; 32].join(""), input);

    let rslt = permute_32(input, &P_BOX);

    println!("dst: {}{:032b}", [" "; 32].join(""), rslt);
    println!("------------------------------------------------------------------------------------------");

    rslt
}

fn FinalPermutation(input: u64) -> u64 {
    println!("----------------------------------- Final Permutation ------------------------------------");
    println!("src: {:064b}", input);

    let rslt = permute_64(input, &FINAL_PERMUTATION);

    println!("dst: {:064b}", rslt);
    println!("------------------------------------------------------------------------------------------");

    rslt
}

fn reduceKey(input: &[u8; 8]) -> u64 {
    let mut rslt = String::new();

    for i in 0..8 {
        rslt.push_str(format!("{:07b}", input[i] >> 1).as_str());
    }
    // println!("{}", rslt.as_str());
    u64::from_str_radix(rslt.as_str(), 2).unwrap()
}

fn permute_key_56(input: u64) -> u64 {
    permute_56(input, &PC1)
}

fn Rotate_28(data: u32, count: usize) -> u32 {
    let count = count % 32;
    let mut rslt = String::new();
    let data = format!("{:028b}", data);

    rslt.push_str("0000");
    rslt.push_str(&data.as_str()[28 - count..]);
    rslt.push_str(&data.as_str()[..28 - count]);
    
    u32::from_str_radix(rslt.as_str(), 2).unwrap()
}

fn permute_subKey(input: u64) -> u64 {
    permute_48(input, &PC2)
}

pub fn generateKeys(key: &[u8; 8]) -> [u64; 16] {
    let mut rslt: [u64; 16] = [0; 16];
    let mut key = reduceKey(key);
    key = permute_56(key, &PC1);
    let mut L_Half: u32 = ((key >> 28) & 0xFFFFFFF) as u32;
    let mut R_Half: u32 = (key & 0xFFFFFFF) as u32;
    
    println!("L_Half: {:07X}", L_Half);
    println!("R_Half: {:07X}\n", R_Half);
    
    for i in 0..16 {
        L_Half = Rotate_28(L_Half, KEY_ROTATIONS[i]);
        R_Half = Rotate_28(R_Half, KEY_ROTATIONS[i]);
        key = ((L_Half << 28) as u64) | R_Half as u64;
        rslt[i] = permute_subKey(key);
    }
    rslt
}

pub fn encrypt_block(block: &[u8; BLOCK_SIZE], key: &[u8; EXPANDED_SIZE]) -> [u8; BLOCK_SIZE] {
    let key = [0, 0, key[0], key[1], key[2], key[3], key[4], key[5]];
    let mut data = convert8to64(block);
    data = reverse_bits_u64(data);            // sus, may be removed later
    data = initialPermutation(data);

    let mut LPT: u32 = (data >> 32) as u32;
    let mut RPT: u32 = (data & 0xFFFFFFFF) as u32;

    for i in 0..16 {
        let mut eRPT = expansion(RPT);
        let numKey = convert8to64(&key);

        eRPT = XOR_48(eRPT, numKey);
        RPT = SBox(eRPT);
        RPT = PBox_Permutation(RPT);
        RPT ^= LPT;

        swap(&mut RPT, &mut LPT);
    }

    data = ((LPT as u64) << 32) | RPT as u64;       // Group the halves
    data = FinalPermutation(data);

    convert64to8(data)
}