use crate::classical_ciphers::caesar::Menu as CaesarMenu;
use crate::classical_ciphers::vigenere::Menu as VigenereMenu;
use crate::classical_ciphers::frequency_analysis::Menu as FrequencyAnalysisMenu;
use crate::classical_ciphers::rail_fence::Menu as RailFenceMenu;
use crate::classical_ciphers::playfair::Menu as PlayfairMenu;
const RETURN_STATUS: u8 = 0;

const options: [&str; 6] = [
    "Caesar Cipher",
    "Vigenere Cipher",
    "Rail Fence Cipher",
    "Playfair Cipher",
    "Frequency Analysis",
    "Return"
];

fn printMenu(){
    println!("PLease choose a cipher:");
    for i in 0..options.len(){
        println!("{}- {}", i + 1, options[i]);
    }
}

pub fn Menu(PATH: &mut String){
    const PREFIX: &str = "classical_ciphers/";
    PATH.push_str(PREFIX);
    loop {
        printMenu();
        let mut r = super::getInput(PATH.clone(), 1, options.len() as u8);
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        r = match r {
            1 => CaesarMenu(PATH),
            2 => VigenereMenu(PATH),
            3 => RailFenceMenu(PATH),
            4 => PlayfairMenu(PATH),
            5 => FrequencyAnalysisMenu(PATH),
            _ => return
        };

        if r == RETURN_STATUS { break; }
    }

    PATH.drain(PATH.len() - PREFIX.len()..);
    return;
}