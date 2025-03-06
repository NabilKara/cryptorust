#![allow(nonstandard_style)]
#![allow(dead_code)]

use std::process::exit;

mod classical_ciphers;
mod menu;

fn printMenu(){
    println!("PLease choose an option:");
    println!("1- Cyphers");
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
