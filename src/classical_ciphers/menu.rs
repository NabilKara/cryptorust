use crate::classical_ciphers::caesar::Menu as CaesarMenu;
use crate::classical_ciphers::vigenere::Menu as VigenereMenu;
use crate::classical_ciphers::frequency_analysis::Menu as FrequencyAnalysisMenu;

const RETURN_STATUS: u8 = 1;

fn printMenu(){
    println!("PLease choose a cipher:");
    println!("1- Caesar Cipher");
    println!("2- Frequency Analysis");
    println!("3- Vigenere cipher");
    println!("4- Return");
}

pub fn Menu(PATH: &mut String){
    PATH.push_str("classical_ciphers/");
    loop {
        printMenu();
        let mut r = super::getInput(PATH.clone(), 1, 4);
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        r = match r {
            1 => CaesarMenu(PATH),
            2 => FrequencyAnalysisMenu(PATH),
            3 => VigenereMenu(PATH),
            _ => return
        };

        if r == RETURN_STATUS { break; }
    }

    return;
}