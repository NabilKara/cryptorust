#![allow(nonstandard_style)]
#![allow(dead_code)]

use crate::symmetric_encryption::des::{encrypt_block, generateKeys};

mod classical_ciphers;
mod menu;
mod symmetric_encryption;

fn printMenu(){
    println!("PLease choose an option:");
    println!("1- Ciphers");
    println!("2- Encryption Systems");
    println!("3- Help");
    println!("4- Quit");
}


fn main() {
    // print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    // loop {
    //
    //     let mut PATH = String::from("/");
    //     printMenu();
    //     let r = menu::getInput(PATH.clone(), 1, 4);
    //     print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    //     match r {
    //         1 => {
    //             classical_ciphers::menu::Menu(&mut PATH);
    //             println!("------------------------------");
    //         },
    //         4 => {
    //             println!("Good Bye !! ");
    //             break;
    //         },
    //         _ => {
    //             println!("This option isn't yet available");
    //         }
    //     }
    // }
    
    // let s0 = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
    // let mtr: [[u8; 16]; 4];
    // let s0: [u8; 8] = [
    //     0x61,
    //     0x62,
    //     0x63,
    //     0x64,
    //     0x65,
    //     0x66,
    //     0x67,
    //     0x68,
    // ];
    // 
    // let key: [u8; 6] = [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
    // 
    // encrypt_block(&s0, &key);

    let row1: [u8; 8] = [0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01];
    let row2: [u8; 8] = [0x1F, 0x1F, 0x1F, 0x1F, 0x0E, 0x0E, 0x0E, 0x0E];
    let row3: [u8; 8] = [0xE0, 0xE0, 0xE0, 0xE0, 0xF1, 0xF1, 0xF1, 0xF1];
    let row4: [u8; 8] = [0xFE, 0xFF, 0xFE, 0xFF, 0xFE, 0xFF, 0xFE, 0xFF];

    generateKeys(&row1);
    generateKeys(&row2);
    generateKeys(&row3);
    generateKeys(&row4);
}