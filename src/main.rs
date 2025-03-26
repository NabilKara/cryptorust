#![allow(nonstandard_style)]
#![allow(dead_code)]

use num::Integer;

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
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    loop {

        let mut PATH = String::from("/");
        printMenu();
        let r = menu::getInput(PATH.clone(), 1, 4);
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        match r {
            1 => {
                classical_ciphers::menu::Menu(&mut PATH);
                println!("------------------------------");
            },
            2 => {
                symmetric_encryption::menu::Menu(&mut PATH);
                println!("------------------------------");
            }
            4 => {
                println!("Good Bye !! ");
                break;
            },
            _ => {
                println!("This option isn't yet available");
            }
        }
    }
}

fn outputBytes(buf: Vec<u8>) {
    if buf.len().is_even() {
        for i in (0..buf.len() - 1).step_by(2) { print!("{:02x}{:02x} ", buf[i], buf[i+1]);/* printing in big endian order, swap endianness to verify with openssl command */}
    }
    else {
        for i in (0..buf.len() - 2).step_by(2) {
            print!("{:02x}{:02x} ", buf[i], buf[i+1]);
        }
        print!("{:02x} ", buf[buf.len() - 1]);
    }
    println!();
}