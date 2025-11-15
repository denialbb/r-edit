pub mod buffer;
pub mod caret;
pub mod logger;
pub mod terminal;
pub mod view;

use buffer::Buffer;
use caret::{Caret, Direction};
use crossterm::event::KeyCode::{
    Backspace, Char, Down, End, Enter, Home, Left, PageDown, PageUp, Right, Up,
};
use crossterm::event::{Event, Event::Key, KeyEvent, KeyModifiers, read};
use log::debug;
use log::info;
use std::io::Error;
use std::thread::sleep;
use std::time::Duration;
use terminal::{Size, Terminal};
use view::View;

pub struct Editor {
    should_quit: bool,
    caret: Caret,
    view: View,
    // buffers: Vec<&Buffer>,
    current_buffer: Buffer,
    filename: String,
}

impl Editor {
    pub fn default(filename: String) -> Self {
        Self {
            should_quit: false,
            caret: caret::Caret::default(),
            // TODO implement multiple buffers
            // buffers: Vec::new(),
            current_buffer: Buffer::default(),
            view: View::default(),
            filename: filename,
        }
    }

    pub fn run(&mut self) {
        info!("--------------------------------------------");
        info!("Editor is running");
        Terminal::initialize().unwrap();
        self.current_buffer = Buffer::read_file(&self.filename);
        // self.buffers.push(self.current_buffer);

        self.view = View::new();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
        info!("Editor finished running");
        info!("--------------------------------------------");
    }

    pub fn repl(&mut self) -> Result<(), Error> {
        info!("Starting read-evaluate-print loop");

        loop {
            View::render(
                &mut self.view,
                &mut self.caret,
                &mut self.current_buffer,
            )?;

            if self.should_quit {
                info!("Quitting editor");
                View::goodbye_message(&mut self.caret)?;
                sleep(Duration::from_millis(1000));
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
                    info!("Ctrl-Q pressed, quitting");
                }
                Char('s') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                    info!("Ctrl-S pressed, quitting and saving buffer");
                    self.current_buffer.write_file(&self.filename);
                }
                Char(c) => {
                    Buffer::insert(
                        &mut self.current_buffer,
                        *c,
                        self.caret.location,
                    );

                    self.caret.shift(Direction::Right);
                    self.view.needs_redraw = true;
                }
                Enter => {
                    Buffer::insert(
                        &mut self.current_buffer,
                        '\n',
                        self.caret.location,
                    );

                    View::draw_buffer(&self.current_buffer, &mut self.caret)
                        .unwrap();
                    self.caret.shift(Direction::Down);
                    self.caret.location.x = 0;
                    self.view.needs_redraw = true;
                }
                Backspace => {
                    Buffer::backspace(
                        &mut self.current_buffer,
                        self.caret.location,
                    );

                    self.caret.shift(Direction::Left);
                    self.view.needs_redraw = true;
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
