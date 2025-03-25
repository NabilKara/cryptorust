use super::constants;
use super::base;
fn pkcs5_pad(data: &mut Vec<u8>) -> &mut Vec<u8> {
    let pad_len = constants::BLOCK_SIZE - (data.len() % constants::BLOCK_SIZE);
    data.extend(vec![pad_len as u8; pad_len]);
    data
}

pub fn encryptECB(_plaintext: Vec<u8>, key: &[u8; constants::BLOCK_SIZE]) -> Vec<[u8; constants::BLOCK_SIZE]>  {
    let mut rslt = Vec::new();
    let plaintext = _plaintext.clone();
    // pkcs5_pad(&mut plaintext);

    for (_i, block) in plaintext.chunks(constants::BLOCK_SIZE).enumerate() {
        let block_slice = <&[u8; constants::BLOCK_SIZE]>::try_from(block).unwrap();

        rslt.push(base::encrypt_block(block_slice, key));
    }

    rslt
}

fn xorIV(data: &[u8; constants::BLOCK_SIZE], iv: &[u8; constants::BLOCK_SIZE]) -> [u8; constants::BLOCK_SIZE] {
    let mut rslt = [0u8; constants::BLOCK_SIZE];
    for i in 0..rslt.len() {
        rslt[i] = data[i] ^ iv[i];
    }

    rslt
}

fn encryptCBC(_plaintext: Vec<u8>, iv: &[u8; constants::BLOCK_SIZE], key: &[u8; constants::BLOCK_SIZE]) -> Vec<[u8; constants::BLOCK_SIZE]> {
    let mut rslt = Vec::new();
    let mut plaintext = _plaintext.clone();
    pkcs5_pad(&mut plaintext);
    let iv = iv.clone();
    
    for (_i, block) in plaintext.chunks(constants::BLOCK_SIZE).enumerate() {
        let block_slice = <&[u8; constants::BLOCK_SIZE]>::try_from(block).unwrap();
        let mut encrypted_block = base::encrypt_block(block_slice, key);
        encrypted_block = xorIV(&encrypted_block, &iv);
        
        rslt.push(encrypted_block);
    }

    rslt
}