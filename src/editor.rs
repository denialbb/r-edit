pub mod buffer;
pub mod caret;
pub mod logger;
pub mod terminal;
pub mod view;

use buffer::Buffer;
use caret::{Caret, Direction};
use crossterm::event::Event::Resize;
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

impl Drop for Editor {
    fn drop(&mut self) {
        debug!("Dropping Editor!");
        match Terminal::terminate() {
            Ok(_) => {}
            Err(e) => {
                debug!("Error terminating terminal: {}", e);
            }
        }
    }
}

impl Editor {
    pub fn default() -> Self {
        Self {
            should_quit: false,
            caret: caret::Caret::default(),
            // TODO implement multiple buffers
            // buffers: Vec::new(),
            current_buffer: Buffer::default(),
            view: View::default(),
            filename: String::from("./test/test.txt"),
        }
    }
    pub fn new(filename: String) -> Self {
        Self::set_up_panic_hook();
        Self {
            should_quit: false,
            caret: caret::Caret::default(),
            current_buffer: Buffer::default(),
            view: View::default(),
            filename: filename,
        }
    }

    pub fn run(&mut self) {
        info!("--------------------------------------------");
        info!("Editor is running");
        match Terminal::initialize() {
            Ok(_) => {}
            Err(e) => {
                debug!("Error initializing terminal: {e}");
                panic!();
            }
        }

        match Buffer::read_file(&self.filename) {
            Ok(buffer) => self.current_buffer = buffer,
            Err(e) => {
                debug!("Error opening file: {e}");
                // TODO(dan): for now panic is fine, in the future maybe
                // open some kind of dashboard
                panic!();
            }
        }
        // self.buffers.push(self.current_buffer);

        self.view = View::new(Terminal::size().unwrap());
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
        info!("Editor finished running");
        info!("--------------------------------------------");
    }

    pub fn repl(&mut self) -> Result<(), Error> {
        info!("Starting read-evaluate-print loop");

        loop {
            match View::render(
                &mut self.view,
                &mut self.caret,
                &mut self.current_buffer,
            ) {
                Ok(_) => {}
                Err(e) => {
                    debug!("Error rendering: {e}");
                }
            }

            if self.should_quit {
                info!("Quitting editor");
                match View::goodbye_message(&mut self.caret) {
                    Ok(_) => {
                        sleep(Duration::from_millis(1000));
                    }
                    Err(e) => {
                        debug!("Error printing goodbye message: {e}");
                    }
                };
                break;
            }

            match read() {
                Ok(event) => self.evaluate_event(&event),
                Err(e) => {
                    debug!("Error handling event: {}", e);
                }
            }
        }

        info!("Exiting REPL loop");
        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) {
        info!("Evaluating event: {:?}", event);
        if let Resize(x, y) = event {
            self.view.resize(Size {
                height: *y as usize,
                width: *x as usize,
            });
            self.caret.size = Size {
                height: *y as usize,
                width: *x as usize,
            };
        }
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

    fn set_up_panic_hook() {
        let current_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |panic_info| {
            match Terminal::terminate() {
                Ok(_) => {}
                Err(e) => {
                    println!("{}", e);
                }
            }
            current_hook(panic_info);
        }));
    }
}
