// A File containing all base structs and constants needed for the work of the protocol

use std::{fmt, io};
use num_bigint::BigUint;
use rand::random;
use crate::secureChannel::util::cmp_vec;
use sha2::{Sha256, Digest};
use crate::symmetric_encryption::aes::aes_encryption::encrypt_cbc as AES_CBC_Encrypt;
use crate::symmetric_encryption::aes::aes_decryption::decrypt_cbc as AES_CBC_Decrypt;
use crate::asymmetric_encryption::RSA::encrypt as RSA_Encrypt;
use crate::asymmetric_encryption::RSA::decrypt as RSA_Decrypt;

pub const PROTOCOL_PORT: u16 = 42069;

pub const IV_SIZE: usize = 16;                  // AES-128 IV

pub const SEPARATOR_SIZE: usize = 8;
pub const SEPARATOR: [u8; DELIMITER_SIZE] = [0xFFu8; DELIMITER_SIZE];

pub const HASH_SIZE: usize = 32;                // SHA2-256

pub const DELIMITER_SIZE: usize = 8;            // Changeable
pub const FRAME_DELIMITER: [u8; DELIMITER_SIZE] = [0u8; DELIMITER_SIZE];  // Changeable

pub const PACKET_MIN_SIZE: usize = IV_SIZE + SEPARATOR_SIZE + HASH_SIZE + DELIMITER_SIZE;

pub const EXIT_MESSAGE: &str = "STD_EXIT";

pub struct RSA_KEY {
    pub(crate) d: BigUint,
    pub(crate) e: BigUint,
    pub(crate) n: BigUint,
}

#[derive(Clone)]
pub struct Message {
    IV: [u8; 16],                       // AES-128 CBC IV
    Data: Vec<u8>,
    Separator: [u8; SEPARATOR_SIZE],    // Separator between Data and Signature (Both are Vec<u8>)
    Signature: Vec<u8>,                 // Basic RSA Signing
    Hash: [u8; HASH_SIZE],              // Rust's Built-In SHA2-256 Hash, While we implement it 
    Delimiter: [u8; DELIMITER_SIZE],    // Delimiter to end Packet
}

impl fmt::Debug for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Message {{")?;
        writeln!(f, "\tIV       : {:?}", self.IV)?;
        writeln!(f, "\tData     : {:?}", self.Data)?;
        writeln!(f, "\tSeparator: {:?}", self.Separator)?;

        writeln!(f, "\tSignature: {:?}", self.Signature)?;
        writeln!(f, "\tHash     : {:?}", self.Hash)?;
        writeln!(f, "\tDelimiter: {:?}", self.Delimiter)?;
        writeln!(f, "}}")
    }
}

impl Message {
    pub fn new(data: &Vec<u8>, aes_key: &[u8; 16], rsa_key: &RSA_KEY) -> Message {
        let iv: [u8; 16] = random::<[u8; 16]>();
        let encrypted_data = AES_CBC_Encrypt(data, &iv, &aes_key);

        // Payload to Hash
        let mut payload = iv.clone().to_vec();
        payload.extend(&iv);

        // Compute SHA2-256 hash of the payload
        let mut hasher = Sha256::new();
        hasher.update(&payload);
        let hash: [u8; HASH_SIZE] = hasher.finalize_reset().into();

        // RSA Signature
        let signature = RSA_Encrypt(&hash.to_vec(), &rsa_key.d, &rsa_key.n);

        Message {
            IV: iv,
            Data: encrypted_data,
            Separator: SEPARATOR,
            Signature: signature,
            Hash: hash,
            Delimiter: FRAME_DELIMITER,
        }
    }

    pub(crate) fn toBytes(self) -> Vec<u8> {
        let mut output = Vec::new();
        output.extend(&self.IV);
        output.extend(&self.Data);
        output.extend(&self.Separator);
        output.extend(&self.Signature);
        output.extend(&self.Hash);
        output.extend(&self.Delimiter);
        output
    }

    pub(crate) fn fromBytes(data: &[u8], rsa_key: &RSA_KEY) -> io::Result<Message> {
        if data.len() < PACKET_MIN_SIZE { panic!("Message too small to contain base structure ({} < {}).", data.len(), PACKET_MIN_SIZE); }

        let delimiter_index = match data.windows(DELIMITER_SIZE).position(|window| window == FRAME_DELIMITER) {
            Some(pos) => pos,
            None => panic!("FRAME_DELIMITER Not Found."),
        };

        let separator_index = match data.windows(SEPARATOR_SIZE).position(|window| window == SEPARATOR) {
            Some(pos) => pos,
            None => panic!("SEPARATOR Not Found."),
        };

        let iv: [u8; IV_SIZE]                =  data[..IV_SIZE].try_into().unwrap();
        let encrypted_data: Vec<u8>          =  data[IV_SIZE..separator_index].to_vec();
        let separator: [u8; SEPARATOR_SIZE]  =  data[separator_index..separator_index + SEPARATOR_SIZE].try_into().unwrap();
        let signature: Vec<u8>               =  data[separator_index + SEPARATOR_SIZE..delimiter_index - HASH_SIZE].to_vec();
        let hash: [u8; HASH_SIZE]            =  data[delimiter_index - HASH_SIZE..delimiter_index].try_into().unwrap();
        let delimiter: [u8; DELIMITER_SIZE]  =  data[delimiter_index..delimiter_index + DELIMITER_SIZE].try_into().unwrap();

        // Payload to Hash
        let mut payload = iv.clone().to_vec();
        payload.extend(&iv);

        // Compute SHA2-256 hash of the payload
        let mut hasher = Sha256::new();
        hasher.update(&payload);
        let expected_hash: [u8; HASH_SIZE] = hasher.finalize_reset().into();

        // Verify Hash
        if expected_hash != hash {
            panic!("Hash mismatch.\nExpected: {:?}\nGot:      {:?}", expected_hash, hash);
        }

        // Verify Signature
        let signature = RSA_Decrypt(&signature, &rsa_key.e, &rsa_key.n);
        if !cmp_vec(&hash.to_vec(), &signature) {
            panic!("Signature Mismatch.\nExpected: {:?}\nGot:      {:?}", hash.to_vec(), signature.to_vec());
        }

        Ok(Message {
            IV: iv,
            Data: encrypted_data,
            Separator: separator,
            Signature: signature,
            Hash: hash,
            Delimiter: delimiter,
        })
    }

    pub(crate) fn getClearText(&self, key: &[u8; 16]) -> Vec<u8> {
        match AES_CBC_Decrypt(self.clone().Data, &self.IV, key) {
            Ok(cleartext) => cleartext,
            Err(e) => panic!("Error Decrypting message: {}", e)
        }
    }
}
