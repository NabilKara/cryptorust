#![allow(nonstandard_style)]
#![allow(dead_code)]


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
    // A
    // encrypt_block(&s0, &key);

    // let key = ['A' as u8, 'B' as u8, 'C' as u8, 'D' as u8, 'E' as u8, 'F' as u8, 'G' as u8, 'H' as u8];
    // let _cleartext = "ABCDEFGH";
    // let cleartext: Vec<u8> = _cleartext.as_bytes().to_vec();
    // let ciphertext = symmetric_encryption::des::encrypt::encryptECB(cleartext, &key);
    // let mut rslt = Vec::new();
    //
    // for i in 0..ciphertext.len() {
    //     rslt.extend(ciphertext[i]);
    // }
    //
    // outputBytes(rslt);

    let key = [0x75, 0x28, 0x78, 0x39, 0x74, 0x93, 0xCB, 0x70];
    let _cleartext = [0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88];
    let cleartext: Vec<u8> = _cleartext.to_vec();
    let ciphertext = symmetric_encryption::des::encrypt::encryptECB(cleartext, &key);
    let mut rslt = Vec::new();

    for i in 0..ciphertext.len() {
        rslt.extend(ciphertext[i]);
    }

    outputBytes(rslt);



    // let v0: u32 = 0b0000_0101_0001_0000_1111_1111_1111_0101;
    // let v1 = Rotate_28(v0, 2);
    // println!("0: {:028b}", v0);
    // println!("1: {:028b}", v1);
}

fn outputBytes(buf: Vec<u8>) {
    for i in (0..buf.len() - 1).step_by(2) { print!("{:02x}{:02x} ", buf[i], buf[i+1]);/* printing in big endian order, swap endianness to verify with openssl command */}
    println!();
}