use crossterm::event::{Event::Key, KeyCode::Char, read};
use crossterm::terminal::disable_raw_mode;
use crossterm::terminal::enable_raw_mode;

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
        loop {
            match read() {
                Ok(Key(event)) => {
                    println!("Event: {event:?}\r");

                    match event.code {
                        Char(c) => {
                            if c == 'q' {
                                break;
                            }
                        }
                        _ => (),
                    }
                }
                Err(e) => println!("Error: {e}"),
                _ => (),
            }
        }
        disable_raw_mode().unwrap();
    }
}
