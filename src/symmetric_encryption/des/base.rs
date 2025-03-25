use super::constants;

// 'pos' from 1 .. 64
pub fn getBit_64(num: u64, pos: usize) -> u64 {
    (num & (1 << (64 - pos))) >> (64 - pos)
}

pub fn getBit_56(num: u64, pos: usize) -> u64 {
    (num & (1 << (56 - pos))) >> (56 - pos)
}

pub fn getBit_32(num: u32, pos: usize) -> u32 {
    (num & (1 << (32 - pos))) >> (32 - pos)
}

pub fn reverse_bits_u8(num: u8) -> u8 {
    let mut reversed = 0u8;
    for i in 0..8 {
        // Shift the current bit to its reversed position
        reversed |= ((num >> i) & 1) << (7 - i);
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

fn PC2Permutation(input: u64, arr: &[usize; 48]) -> u64 {
    let mut rslt: u64 = 0;

    for i in 00..48 { rslt |= getBit_56(input, arr[i]) << (47 - i); }

    rslt & 0xFFFF_FFFF_FFFF
}

fn permute_32(input: u32, arr: &[usize; 32]) -> u32 {
    let mut rslt: u32 = 0;

    for i in 00..32 { rslt |= getBit_32(input, arr[i]) << (31 - i); }

    rslt
}

pub fn initialPermutation(input: u64) -> u64 {
    println!("---------------------------------- Initial Permutation -----------------------------------");
    println!("src: {:064b}", input);

    let rslt = permute_64(input, &constants::IP);

    println!("dst: {:064b}", rslt);
    println!("------------------------------------------------------------------------------------------");

    rslt
}

pub fn expansion(input: u32) -> u64 {
    let mut rslt: u64 = 0;

    println!("--------------------------------- Expansive Permutation ----------------------------------");
    println!("src: {}{:032b}", [" "; 32].join("") ,input);

    for i in 0..constants::expansion_table.len() {
        rslt |= (getBit_32(input, constants::expansion_table[i]) as u64) << (47 - i);
    }

    println!("dst: {}{:048b}", [" "; 16].join(""), rslt);
    println!("------------------------------------------------------------------------------------------");

    rslt
}

fn XOR_48(input: u64, key: u64) -> u64 {
    println!("---------------------------------------- XOR Key -----------------------------------------");
    println!("src: {}{:048b}", [" "; 16].join(""), input);
    let rslt = (input ^ key) & 0xFFFF_FFFF_FFFF;
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
        let slice_rslt = format!("{:04b}", SBox_Transform(slice, &constants::S_BOXES[i]));
        rslt.push_str(slice_rslt.as_str());

    }
    let rslt = u32::from_str_radix(rslt.as_str(), 2).unwrap();
    println!("dst: {}{:032b}", [" "; 32].join(""), rslt);
    println!("------------------------------------------------------------------------------------------");

    rslt
}

fn SBox_Transform(input: u8, table: &[[usize; 16]; 4]) -> u8 {
    let row = (((input & 0x20) >> 4) | input & 0x01) as usize;  // 1st & 6th bits
    let column = ((input & 0x1E) >> 1) as usize;                // Remaining 4 bits

    (table[row][column] & 0x0F) as u8                                 // & with 4 bits for safety
}

fn PBox_Permutation(input: u32) -> u32 {
    println!("----------------------------------- P-Box Permutation ------------------------------------");
    println!("src: {}{:032b}", [" "; 32].join(""), input);

    let rslt = permute_32(input, &constants::P_BOX);

    println!("dst: {}{:032b}", [" "; 32].join(""), rslt);
    println!("------------------------------------------------------------------------------------------");

    rslt
}

fn FinalPermutation(input: u64) -> u64 {
    println!("----------------------------------- Final Permutation ------------------------------------");
    println!("src: {:064b}", input);

    let rslt = permute_64(input, &constants::FINAL_PERMUTATION);

    println!("dst: {:064b}", rslt);
    println!("------------------------------------------------------------------------------------------");

    rslt
}

pub fn reduceKey(input: u64) -> u64 {
    // println!("------------------------------------- Key Reduction --------------------------------------");
    // println!("src: {:064b}", input);
    //
    // println!("dst: {}{:056b}", [" "; 8].join(""), permute_56(input, &constants::PC1));
    // println!("------------------------------------------------------------------------------------------");
    permute_56(input, &constants::PC1)

}

pub fn Rotate_28(data: u32, count: usize) -> u32 {
    let count = count % 28;
    ((data << count) | (data >> (28 - count))) & 0xF_FFF_FFF
}

pub fn permute_subKey(input: u64) -> u64 {
    // println!("-----------------------------------Sub-Key Permutation -----------------------------------");
    // println!("src: {}{:056b}", [" "; 8].join(""), input);
    //
    // println!("dst: {}{:048b}", [" "; 16].join(""), PC2Permutation(input, &constants::PC2));
    // println!("------------------------------------------------------------------------------------------");
    PC2Permutation(input, &constants::PC2)
}

pub fn generateKeys(_key: &[u8; 8]) -> [u64; 16] {
    let mut rslt: [u64; 16] = [0; 16];
    let mut key = u64::from_be_bytes(*_key);
    // println!("------------------------------------ Key Generation --------------------------------------");
    // println!("src: {:064b}\n", key);
    key = reduceKey(key);
    let mut L_Half: u32 = (key >> 28) as u32;
    let mut R_Half: u32 = (key & 0xFFFFFFF) as u32;

    for i in 0..16 {
        // println!("------------------------------------ Rotate Halves ---------------------------------------");
        // println!("src: {}{:056b}", [" "; 8].join(""), ((L_Half as u64) << 28) | R_Half as u64);
        L_Half = Rotate_28(L_Half, constants::KEY_ROTATIONS[i]);
        R_Half = Rotate_28(R_Half, constants::KEY_ROTATIONS[i]);
        // println!("dst: {}{:056b}", [" "; 8].join(""), ((L_Half as u64) << 28) | R_Half as u64);
        // println!("------------------------------------------------------------------------------------------");

        key = ((L_Half as u64) << 28) | R_Half as u64;
        rslt[i] = permute_subKey(key);
    }
    // println!("------------------------------------------------------------------------------------------\n\n");
    rslt
}

pub fn encrypt_block(block: &[u8; constants::BLOCK_SIZE], keys: &[u64; constants::ITERATION_NB]) -> [u8; constants::BLOCK_SIZE] {
    // let keys = generateKeys(&_key);
    let mut data = u64::from_be_bytes(*block);
    data = initialPermutation(data);

    println!("------------------------------------ Split LPR RPT --------------------------------------");
    let mut LPT: u32 = (data >> 32) as u32;
    let mut RPT_0: u32 = (data & 0xFFFFFFFF) as u32;    // Original RPT, goes Into LPT at end of iteration
    let mut RPT: u32 = RPT_0;                             // The RPT that will be modified through the iteration
    println!("LPT: {}{:032b}", [" "; 32].join(""), LPT);
    println!("RPT: {}{:032b}", [" "; 32].join(""), RPT_0);
    println!("------------------------------------------------------------------------------------------");

    for iteration in 0..constants::ITERATION_NB {
        println!("------------------------------------- Iteration N°{} -------------------------------------", iteration);
        RPT_0 = RPT;
        let mut eRPT = expansion(RPT);

        eRPT = XOR_48(eRPT, keys[iteration]);
        RPT = SBox(eRPT);
        RPT = PBox_Permutation(RPT);
        println!("------------------------------- XOR wih left and grouping -------------------------------");
        println!("src: {}{:032b}", [" "; 32].join(""), RPT);

        RPT ^= LPT;
        LPT = RPT_0;

        println!("dst: {:032b}{:032b}", RPT, RPT_0);
        println!("------------------------------------------------------------------------------------------");
        println!("------------------------------------------------------------------------------------------\n\n");
    }

    data = ((RPT as u64) << 32) | RPT_0 as u64;       // Group the halves, I really don't know why RPT and RPT_0, but it is what it is, verified answer.
    data = FinalPermutation(data);

    data.to_be_bytes()
}