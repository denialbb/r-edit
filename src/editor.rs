use crate::terminal::Terminal;
use crossterm::event::KeyCode::{Char, Enter};
use crossterm::event::{Event, Event::Key, KeyEvent, KeyModifiers, read};
use std::io::{self, Write};

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
}

impl Editor {
    pub fn default() -> Self {
        Editor {
            should_quit: false,

            terminal: Terminal::default(),
        }
    }

    pub fn new() -> Editor {
        Editor {
            should_quit: false,
            terminal: Terminal::default(),
        }
    }

    pub fn run(&mut self) {
        self.terminal.initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    pub fn repl(&mut self) -> Result<(), std::io::Error> {
        loop {
            let event = read()?;
            self.evaluate_event(event);
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
        }
        Ok(())
    }

    fn evaluate_event(&mut self, event: Event) {
        if let Key(KeyEvent {
            code, modifiers, ..
        }) = event
        {
            match (code, modifiers) {
                (Char('q'), KeyModifiers::CONTROL) => self.should_quit = true,
                (Enter, _) => self.print('\n'),
                (Char(c), _) => self.print(c),
                _ => (),
            }
        }
    }

    fn print(&mut self, c: char) {
        print!("{}", c);
        self.terminal.draw_rows().unwrap();
    }

    fn refresh_screen(&mut self) -> Result<(), std::io::Error> {
        if self.should_quit {
            self.terminal.clear_screen()?;
            print!("Goodbye.\r\n");
        }
        Ok(())
    }
}
