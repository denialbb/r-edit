use crate::editor::Caret;
use crate::editor::Editor;
use crate::editor::Size;
use crate::editor::Terminal;
use crate::editor::debug;
use crate::editor::info;
use crate::editor::read;
use crate::editor::terminal::Location;
use crate::editor::terminal::Position;
use std::io::Error;
use std::thread::sleep;
use std::time::Duration;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct View {
    is_new_buffer: bool,
}

impl View {
    pub fn new() -> View {
        View {
            is_new_buffer: true,
        }
    }
    /// Render the current state of the editor, called in the main loop
    pub fn render(self: &mut Self, editor: &mut Editor) -> Result<(), Error> {
        debug!("Rendering editor");
        if self.is_new_buffer {
            Self::welcome_message(&mut editor.caret)?;
            read()?;
            self.is_new_buffer = false;
            let size = Terminal::size()?;
            Self::clear_screen(&mut editor.caret, size)?;
        }
        Self::refresh_screen(editor)?;
        Ok(())
    }

    pub fn refresh_screen(editor: &mut Editor) -> Result<(), Error> {
        info!("Refreshing screen");
        debug!("Caret location: {}", editor.caret.location);
        Terminal::hide_caret()?;
        if editor.should_quit {
            Self::goodbye_message(&mut editor.caret)?;
            sleep(Duration::from_millis(1000));
        } else {
            Self::draw_rows(&mut editor.caret)?;
            Terminal::move_caret_to(editor.caret.location.into())?;
        }
        Terminal::show_caret()?;
        Terminal::execute()?;
        Ok(())
    }

    pub fn draw_rows(caret: &mut Caret) -> Result<(), Error> {
        let Size { height, width } = Terminal::size()?;
        caret.size = Size { height, width };

        // Self::clear_screen(caret, height, width)?;
        Terminal::print("\r\n")?;

        Ok(())
    }

    fn clear_screen(caret: &mut Caret, size: Size) -> Result<(), Error> {
        let current_line = caret.location.y + 1;
        Terminal::move_caret_to(Position {
            x: 0,
            y: current_line,
        })?;
        Terminal::clear_down()?;
        let height = size.height;
        for current_line in caret.location.y + 1..height {
            Terminal::print("~")?;
            if current_line < height - 1 {
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
    }

    pub fn welcome_message(caret: &mut Caret) -> Result<(), Error> {
        info!("Displaying welcome message");
        let Size { width, height } = Terminal::size()?;

        Terminal::hide_caret()?;
        Terminal::clear_screen()?;

        let message = format!("R-EDIT -- v{}", VERSION);
        let row = height / 3;
        let column = width / 2;
        let msg_len = message.len() as usize;

        match (column).checked_sub(msg_len / 2) {
            Some(col) => {
                Terminal::move_caret_to(Position { x: col, y: row })?;
            }
            None => {
                info!("Underflow");
                Terminal::move_caret_to(Position { x: 0, y: row })?;
            }
        }

        Terminal::print(&message)?;
        Terminal::print("\r\n\r\n")?;

        Terminal::move_caret_to(Position { x: 0, y: 0 })?;
        caret.location = Location { x: 0, y: 0 };
        Terminal::show_caret()?;
        Terminal::execute()?;
        Ok(())
    }

    pub fn goodbye_message(caret: &mut Caret) -> Result<(), Error> {
        info!("Displaying message");
        let Size { width, height } = Terminal::size()?;
        Terminal::hide_caret()?;
        Terminal::clear_screen()?;

        let mut message = String::from("Goodbye.");
        let row = height / 3;
        let column = width / 2;
        let msg_len = message.len();

        match (column).checked_sub(msg_len / 2) {
            Some(col) => {
                Terminal::move_caret_to(Position { x: col, y: row })?;
            }
            None => {
                info!("Underflow");
                Terminal::move_caret_to(Position { x: 0, y: row })?;
            }
        }
        message.truncate(width as usize);
        Terminal::print(&message)?;
        Terminal::print("\r\n\r\n")?;

        Terminal::move_caret_to(Position { x: 0, y: 0 })?;
        caret.location = Location { x: 0, y: 0 };
        Terminal::show_caret()?;
        Terminal::execute()?;
        Ok(())
    }
}
