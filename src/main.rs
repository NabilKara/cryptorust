#![allow(nonstandard_style)]
#![allow(dead_code)]

mod classical_ciphers;
mod menu;
mod symmetric_encryption;

mod asymmetric_encryption;

use crate::asymmetric_encryption::*;
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
    //         2 => {
    //             symmetric_encryption::menu::Menu(&mut PATH);
    //             println!("------------------------------");
    //         }
    //         4 => {
    //             println!("Good Bye !! ");
    //             break;
    //         },
    //         _ => {
    //             println!("This option isn't yet available");
    //         }
    //     }
    // }
    
    let (n,e,d) = RSA::rsa_generate_key_pair(512,1);
}