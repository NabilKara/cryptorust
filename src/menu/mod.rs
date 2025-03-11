#![allow(nonstandard_style)]
use std::io;
use std::io::Write;

pub fn printGenericMenu(){
    println!("What do you want to do?");
    println!("1- Encrypt");
    println!("2- Decrypt");
    println!("3- Return");
}

pub fn getGenericOption(PATH: String) -> u8 {
    printGenericMenu();
    let r = getInput(PATH, 1, 3);
    println!();
    r
}

pub fn getInput(PATH: String, minVal: u8, maxVal: u8) -> u8 {
    loop {
        let mut input: String = String::new();
        print!("{PATH}> ");        io::stdout().flush().unwrap();
        if io::stdin().read_line(&mut input).unwrap() == 1usize { continue; }
        match input.trim().parse::<u8>() {
            Ok(t) => {
                if t < minVal || t > maxVal { println!("Invalid Choice."); continue; }
                return t;
            },
            Err(_) => { println!("Invalid Input."); continue; }
        }
    }
}
