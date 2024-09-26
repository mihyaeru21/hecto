use std::io::{self, Read};

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

#[derive(Debug, Default)]
pub struct Editor {}

impl Editor {
    pub fn run(&self) {
        enable_raw_mode().unwrap();

        for b in io::stdin().bytes() {
            let b = match b {
                Ok(b) => b,
                Err(err) => {
                    println!("Error: {err}");
                    continue;
                }
            };

            let c = b as char;
            if c.is_control() {
                println!("Binary: {0:08b} ASCII: {0:#03} \r", b);
            } else {
                println!("Binary: {0:08b} ASCII: {0:#03} Character: {1:#?}\r", b, c);
            }

            if c == 'q' {
                break;
            }
        }

        disable_raw_mode().unwrap();
    }
}
