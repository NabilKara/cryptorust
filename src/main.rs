
#![allow(nonstandard_style)]
#![allow(dead_code)]

use std::process::exit;

use crate::classical_ciphers::caesar::encrypt_caesar;
use crate::classical_ciphers::frequency_analysis::{decrypt_using_freq_analysis};
use crate::classical_ciphers::index_of_coincidence::index_of_coincidence_counter;


mod classical_ciphers;
mod menu;

fn printMenu(){
    println!("PLease choose an option:");
    println!("1- Ciphers");
    println!("2- Encryption Systems");
    println!("3- Help");
    println!("4- Quit");
    return;
}

fn main() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    loop {
        let mut PATH = String::from("/");
        printMenu();
        let r = menu::getInput(PATH.clone(), 1, 4);
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        match r {
            1 => classical_ciphers::menu::Menu(&mut PATH),
            4 => { println!("Bye..."); exit(0); },
            _ => { println!("Undefined Choice"); exit(0); },
        }
    }
}
