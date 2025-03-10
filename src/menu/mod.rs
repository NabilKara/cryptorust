use std::io;
use std::io::Write;

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
