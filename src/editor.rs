use crossterm::terminal::disable_raw_mode;
use crossterm::terminal::enable_raw_mode;
use std::io::{self, Read};

pub struct Editor {}

impl Editor {
    pub fn default() -> Self {
        Editor {}
    }

    pub fn new() -> Editor {
        Editor {}
    }

    pub fn run(&self) {
        enable_raw_mode().unwrap();
        for b in io::stdin().bytes() {
            match b {
                Ok(b) => {
                    let c = b as char;
                    if c.is_control() {
                        println!("Binary: {:08b} ASCII: {0:#03}\r", b);
                    } else {
                        println!("Binary: {:08b} ASCII: {0:#03} Char: {1:#?}\r", b, c);
                    }

                    if c == 'q' {
                        break;
                    }
                }
                Err(e) => println!("Error: {}", e),
            }
        }
        disable_raw_mode().unwrap();
    }
}
