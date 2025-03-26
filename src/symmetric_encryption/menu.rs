use num::Integer;

const RETURN_STATUS: usize = 0;

const options: [&str; 4] = [
    "1- AES",
    "2- DES",
    "3- RC4",
    "4- Return"
];

pub(crate) fn printMenu(){
    println!("PLease choose an encryption method:");
    for i in 0..options.len(){
        println!("{}", options[i]);
    }
}

pub fn outputBytes(buf: Vec<u8>) {
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

pub fn Menu(PATH: &mut String){
    const PREFIX: &str = "symmetric_encryption/";
    PATH.push_str(PREFIX);
    loop {
        printMenu();
        let mut r = super::getInput(PATH.clone(), 1, options.len());
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        r = match r {
            1 => crate::symmetric_encryption::aes::menu::Menu(PATH),
            _ => return
        };

        if r == RETURN_STATUS { break; }
    }

    PATH.drain(PATH.len() - PREFIX.len()..);
    return;
}