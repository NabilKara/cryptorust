#![allow(nonstandard_style)]
#![allow(dead_code)]

mod classical_ciphers;
mod menu;
mod symmetric_encryption;
mod asymmetric_encryption;

const options: [&str; 5] = [
    "Ciphers",
    "Symmetric Encryption Systems",
    "Asymmetric Encryption Systems",
    "Help",
    "Quit",
];

fn printMenu(){
    println!("PLease choose an option:");
    for i in 0..options.len(){ println!("{}- {} ", i + 1, options[i]); }
}


fn main() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    loop {
        let mut PATH = String::from("/");
        printMenu();
        let r = menu::getInput(PATH.clone(), 1, options.len());
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        match r {
            1 => classical_ciphers::menu::Menu(&mut PATH),
            2 => symmetric_encryption::menu::Menu(&mut PATH),
            3 => asymmetric_encryption::menu::Menu(&mut PATH),
            5 => {
                println!("Good Bye !! ");
                break;
            },
            _ => println!("This option isn't yet available"),
        }
    }
}