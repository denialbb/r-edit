pub mod logger;
mod terminal;

use crossterm::event::KeyCode::{Char, Enter};
use crossterm::event::{Event, Event::Key, KeyEvent, KeyModifiers, read};
use log::info;
use std::io::Error;
use terminal::{Position, Size, Terminal};

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub const fn default() -> Self {
        Self { should_quit: false }
    }

    pub fn run(&mut self) {
        info!("Editor is running");
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
        info!("Editor finished running");
    }

    pub fn repl(&mut self) -> Result<(), Error> {
        info!("Starting read-evaluate-print loop");
        self.welcome_message();
        read()?;
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                info!("Quitting editor");
                break;
            }
            let event = read()?;
            self.evaluate_event(&event);
        }
        info!("Exiting read-evaluate-print loop");
        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) {
        info!("Evaluating event: {:?}", event);
        if let Key(KeyEvent {
            code,
            modifiers,
            .. 
        }) = event
        {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                    info!("Ctrl-Q pressed, setting should_quit to true");
                }
                Char(c) => {
                    Terminal::print(&c.to_string()).unwrap();
                    info!("Printed character: {}", c);
                },
                Enter => {
                    Terminal::print("\r\n").unwrap();
                    info!("Printed newline");
                },
                _ => info!("Unhandled key event: {:?}", code),
            }
        }
    }

    fn refresh_screen(&mut self) -> Result<(), Error> {
        info!("Refreshing screen");
        Terminal::hide_cursor();
        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("Goodbye.\r\n")?;
            info!("Displayed goodbye message");
        } else {
            Self::draw_rows()?;
            Terminal::move_cursor_to(Position { x: 0, y: 0 })?;
            info!("Drew rows and moved cursor to origin");
        }
        Terminal::show_cursor();
        Terminal::execute()?;
        info!("Screen refreshed");
        Ok(())
    }

    fn draw_rows() -> Result<(), Error> {
        info!("Drawing rows");
        let Size { height, .. } = Terminal::size()?;

        for current_line in 0..height {
            Terminal::clear_current_line()?;
            Terminal::print("~")?;
            if current_line < height - 1 {
                Terminal::print("\r\n")?;
            }
        }
        info!("Rows drawn");
        Ok(())
    }

    fn welcome_message(&self) -> Result<(), Error> {
        info!("Displaying welcome message");
        let Size { width, height } = Terminal::size()?;
        Terminal::hide_cursor()?;
        Terminal::clear_screen()?;

        let version = env!("CARGO_PKG_VERSION");
        let message = format!("Welcome to R-EDIT -- version {}", version);
        let row = height / 3;
        let column = width / 2;
        let msg_len = message.len() as u16;

        info!("Welcome message: {}", message);
        info!("Welcome message length: {}", msg_len);
        info!("Welcome message row: {}", row);
        info!("Welcome message column: {}", column);

        Terminal::move_cursor_to(Position {
            x: column - msg_len / 2,
            y: row,
        })?;
        Terminal::print(&message)?;
        Terminal::print("\r\n\r\n")?;

        Terminal::show_cursor()?;
        Terminal::execute()?;
        info!("Welcome message displayed");
        Ok(())
    }
}
