// rust
use std::io::stdin;


pub struct Settings;

impl Settings {
    
    pub fn initial_menu (&self) {
        println!("----- Initial Menu -----");
        println!("\t1 - Start Game.");
        println!("\t2 - Exit.");
    }

    pub fn end_menu (&self) {
        println!("----- Nice to Spend Time With You -----");
        println!("----- Bye -----");
    }

    pub fn read_input (&self) -> u8 {
        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        return input
            .trim()
            .parse()
            .expect("Failed to parse input");
            
    }

    pub fn clear_terminal (&self) {
        println!("{}[2J", 27 as char);
    }
}