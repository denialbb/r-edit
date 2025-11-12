pub mod caret;
pub mod logger;
pub mod terminal;
pub mod view;

use caret::{Caret, Direction};
use crossterm::event::KeyCode::{
    Backspace, Char, Down, End, Enter, Home, Left, PageDown, PageUp, Right, Up,
};
use crossterm::event::{Event, Event::Key, KeyEvent, KeyModifiers, read};
use log::debug;
use log::info;
use std::io::Error;
use terminal::{Size, Terminal};
use view::View;

pub struct Editor {
    should_quit: bool,
    caret: Caret,
}

impl Editor {
    pub fn default() -> Self {
        Self {
            should_quit: false,
            caret: caret::Caret::default(),
        }
    }

    pub fn run(&mut self) {
        info!("__________________________________________");
        info!("Editor is running");
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
        info!("Editor finished running");
        info!("__________________________________________");
    }

    pub fn repl(&mut self) -> Result<(), Error> {
        info!("Starting read-evaluate-print loop");
        let mut view = View::new();
        loop {
            View::render(&mut view, self)?;
            if self.should_quit {
                info!("Quitting editor");
                break;
            }
            let event = read()?;
            self.evaluate_event(&event);
        }
        info!("Exiting REPL loop");
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
                    if self.caret.location.x == 0 {
                        Terminal::clear_current_line().unwrap();
                    }
                    Terminal::print(&c.to_string()).unwrap();
                    self.caret.shift(Direction::Right);
                    info!("Printed character: {}", c);
                }
                Enter => {
                    Terminal::print("\r\n").unwrap();
                    self.caret.shift(Direction::Down);
                    self.caret.location.x = 0;
                    Terminal::clear_current_line().unwrap();
                    info!("Printed newline");
                }
                Backspace => {
                    self.caret.shift(Direction::Left);
                    Terminal::print(" ").unwrap();
                    // self.caret.shift(Direction::Left);
                    info!("Backspace pressed");
                }
                Left => {
                    self.caret.shift(Direction::Left);
                }
                Right => {
                    self.caret.shift(Direction::Right);
                }
                Up => {
                    self.caret.shift(Direction::Up);
                }
                Down => {
                    self.caret.shift(Direction::Down);
                }
                Home => {
                    self.caret.go_start_of_line();
                }
                End => {
                    self.caret.go_end_of_line();
                }
                PageUp => {
                    self.caret.page_up();
                }
                PageDown => {
                    self.caret.page_down();
                }
                _ => info!("Unhandled key event: {:?}", code),
            }
        }
    }
}
