pub mod logger;
mod terminal;

use crossterm::event::KeyCode::{Backspace, Char, Enter};
use crossterm::event::{Event, Event::Key, KeyEvent, KeyModifiers, read};
use log::info;
use std::io::Error;
use terminal::{Position, Size, Terminal};

pub struct Editor {
    should_quit: bool,
    cursor_position: Position,
}

impl Editor {
    pub fn default() -> Self {
        Self {
            should_quit: false,
            cursor_position: Position { x: 0, y: 0 },
        }
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
        self.welcome_message()?;
        read()?;
        Terminal::clear_screen()?;
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
            code, modifiers, ..
        }) = event
        {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                    info!("Ctrl-Q pressed, setting should_quit to true");
                }
                Char(c) => {
                    if self.cursor_position.x == 0 {
                        info!("Clearing current line");
                        Terminal::clear_current_line().unwrap();
                    }
                    Terminal::print(&c.to_string()).unwrap();
                    self.cursor_position.x += 1;
                    info!("Printed character: {}", c);
                    self.refresh_screen().unwrap();
                }
                Enter => {
                    Terminal::print("\r\n").unwrap();
                    self.cursor_position.y += 1;
                    self.cursor_position.x = 0;
                    info!("Printed newline");
                    self.refresh_screen().unwrap();
                }
                Backspace => {
                    if self.cursor_position.x > 0 {
                        Terminal::move_cursor_to(Position {
                            x: self.cursor_position.x - 1,
                            y: self.cursor_position.y,
                        })
                        .unwrap();
                        Terminal::print(" ").unwrap();
                        Terminal::move_cursor_to(Position {
                            x: self.cursor_position.x - 1,
                            y: self.cursor_position.y,
                        })
                        .unwrap();
                        self.cursor_position.x -= 1;
                        info!("Backspace pressed");
                        self.refresh_screen().unwrap();
                    }
                }
                _ => info!("Unhandled key event: {:?}", code),
            }
        }
    }

    fn refresh_screen(&mut self) -> Result<(), Error> {
        info!("Refreshing screen");
        Terminal::hide_cursor()?;
        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("Goodbye.\r\n")?;
            info!("Displayed goodbye message");
        } else {
            self.draw_rows()?;
            Terminal::move_cursor_to(self.cursor_position)?;
        }
        Terminal::show_cursor()?;
        Terminal::execute()?;
        Ok(())
    }

    fn draw_rows(&mut self) -> Result<(), Error> {
        let Size { height, .. } = Terminal::size()?;

        if self.cursor_position.x == 0 {
            info!("Clearing current line");
            Terminal::clear_current_line()?;
            if self.cursor_position.y == 0 {
                info!("Drawing rows");
                Terminal::clear_down()?;
                for current_line in self.cursor_position.y + 1..height {
                    Terminal::print("~")?;
                    if current_line < height - 1 {
                        Terminal::print("\r\n")?;
                    }
                }
            }
        }

        Ok(())
    }

    fn welcome_message(&mut self) -> Result<(), Error> {
        info!("Displaying welcome message");
        let Size { width, height } = Terminal::size()?;
        Terminal::hide_cursor()?;
        Terminal::clear_screen()?;

        let version = env!("CARGO_PKG_VERSION");
        let message = format!("R-EDIT -- v{}", version);
        let row = height / 3;
        let column = width / 2;
        let msg_len = message.len() as u16;

        Terminal::move_cursor_to(Position {
            x: column - msg_len / 2,
            y: row,
        })?;
        Terminal::print(&message)?;
        Terminal::print("\r\n\r\n")?;

        Terminal::move_cursor_to(Position { x: 0, y: 0 })?;
        self.cursor_position = Position { x: 0, y: 0 };
        Terminal::show_cursor()?;
        Terminal::execute()?;
        Ok(())
    }
}

