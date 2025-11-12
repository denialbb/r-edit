use crate::editor::Caret;
use crate::editor::Editor;
use crate::editor::Size;
use crate::editor::Terminal;
use crate::editor::buffer::Buffer;
use crate::editor::debug;
use crate::editor::info;
use crate::editor::read;
use crate::editor::terminal::Location;
use crate::editor::terminal::Position;
use std::io::Error;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct View {
    is_new_buffer: bool,
}

impl View {
    pub fn default() -> View {
        View {
            is_new_buffer: true,
        }
    }
    pub fn new() -> View {
        View {
            is_new_buffer: true,
        }
    }

    /// Render the current state of the editor, called in the main loop
    pub fn render(
        view: &mut View,
        caret: &mut Caret,
        current_buffer: &Buffer,
    ) -> Result<(), Error> {
        debug!("Rendering editor");

        if view.is_new_buffer {
            Self::welcome_message(caret)?;
            read()?;
            view.is_new_buffer = false;
            Self::set_size(caret)?;
            let location: Location = Location { x: 0, y: 0 };
            caret.location = location;
            Self::draw_buffer(&current_buffer, caret)?;
        }

        view.refresh_screen(caret, current_buffer)?;
        Ok(())
    }

    pub fn refresh_screen(
        self: &mut Self,
        caret: &mut Caret,
        current_buffer: &Buffer,
    ) -> Result<(), Error> {
        info!("Refreshing screen");
        debug!("Caret location: {}", caret.location);

        Terminal::hide_caret()?;
        Self::set_size(caret)?;

        let current_line = caret.location.y;
        Terminal::move_caret_to(Position {
            x: 0,
            y: current_line,
        })?;

        let string = current_buffer.get_line(current_line);
        match string {
            Some(string) => {
                debug!("Current line: {}", string);
                Terminal::clear_current_line()?;
                Terminal::print(string.as_str())?;
            }
            None => {
                debug!("Current line: None");
            }
        }

        Terminal::move_caret_to(caret.location.into())?;
        Terminal::show_caret()?;
        Terminal::execute()?;
        Ok(())
    }

    pub fn draw_buffer(
        buffer: &Buffer,
        caret: &mut Caret,
    ) -> Result<(), Error> {
        info!("Drawing buffer");

        let size = Terminal::size()?;
        let lines: &Vec<String> = &buffer.lines;

        Self::clear_screen(caret, size)?;
        Terminal::move_caret_to(Position { x: 0, y: 0 })?;

        for line in lines {
            debug!("Line: {}", line);
            Terminal::print(line.as_str())?;
            Terminal::print("\r\n")?;
        }

        Terminal::move_caret_to(caret.location.into())?;
        Ok(())
    }

    fn set_size(caret: &mut Caret) -> Result<(), Error> {
        let size = Terminal::size()?;
        caret.size = size;
        Ok(())
    }

    fn clear_screen(caret: &mut Caret, size: Size) -> Result<(), Error> {
        let current_line = caret.location.y + 1;
        Terminal::move_caret_to(Position {
            x: 0,
            y: current_line,
        })?;
        Terminal::clear_screen()?;
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

        Terminal::hide_caret()?;
        Terminal::clear_screen()?;

        let Size { width, height } = Terminal::size()?;
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
