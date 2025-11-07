mod terminal;

use crossterm::event::KeyCode::{Char, Enter};
use crossterm::event::{Event, Event::Key, KeyEvent, KeyModifiers, read};
use std::io::{self, Write};
use terminal::Terminal;

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub const fn default() -> Self {
        Self { should_quit: false }
    }

    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    pub fn repl(&mut self) -> Result<(), std::io::Error> {
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
            let event = read()?;
            self.evaluate_event(event);
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
        Terminal::draw_rows().unwrap();
    }

    fn refresh_screen(&mut self) -> Result<(), std::io::Error> {
        if self.should_quit {
            Terminal::clear_screen()?;
            print!("Goodbye.\r\n");
        } else {
            Self::draw_rows()?;
            Terminal::move_cursor_to(0, 0)?;
        }
        Ok(())
    }

    fn draw_rows() -> Result<(), std::io::Error> {
        Terminal::draw_rows()?;
        Terminal::move_cursor_to(0, 0)?;
        Ok(())
    }
}
