#![allow(nonstandard_style)]
#![allow(dead_code)]

use crate::symmetric_encryption::des::{expansion, initialPermutation};

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
    let buf = "aaaabaaacaaadaaaeaaafaaagaaahaaaiaaajaaakaaalaaamaaanaaaoaaapaaa";
    let buf: [char; 64] = ['a', 'a', 'a', 'a', 'b', '\n', 'a', 'a', '\x1b', 'a', 'a', 'a', 'd', 'a', 'a', 'a', 'e', 'a', 'a', 'a', 'f', 'a', 'a', 'a', 'g', 'a', 'a', 'a', 'h', 'a', 'a', 'a', 'i', 'a', 'a', 'a', 'j', 'a', 'a', 'a', 'k', 'a', 'a', 'a', 'l', 'a', 'a', 'a', 'm', 'a', 'a', 'a', 'n', 'a', 'a', 'a', 'o', 'a', 'a', 'a', 'p', 'a', 'a', 'a'];
    println!("{:?}", buf);
    let rslt = initialPermutation(&buf.map(|c| c as u8));
    println!("------------------------------------------------------------------------");
    println!("{:?}", rslt);

    println!("------------------------------------------------------------------------");
    let l0: [u8; 32] = rslt[00..32].try_into().unwrap();        let e0 = expansion(&l0);
    let l1: [u8; 32] = rslt[32..64].try_into().unwrap();        let e1 = expansion(&l1);
    println!("e0: {:02X?}\n", e0.clone());
    println!("e1: {:02X?}", e1.clone());
}