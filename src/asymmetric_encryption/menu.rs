use super::RSA::Menu as RSAMenu;

const RETURN_STATUS: usize = 0;

const options: [&str; 4] = [
    "1- RSA",
    "2- ElGamal",
    "3- Diffie–Hellman key exchange",
    "4- Return"
];

pub(crate) fn printMenu(){
    println!("PLease choose an encryption algorithm:");
    for i in 0..options.len(){
        println!("{}", options[i]);
    }
}

pub fn Menu(PATH: &mut String){
    const PREFIX: &str = "asymmetric_encryption/";
    PATH.push_str(PREFIX);
    loop {
        printMenu();
        let mut r = super::getInput(PATH.clone(), 1, options.len());
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        r = match r {   
            1 => RSAMenu(PATH),
            2 => todo!(),
            _ => return
        };

        if r == RETURN_STATUS { break; }
    }

    PATH.drain(PATH.len() - PREFIX.len()..);
    return;
}