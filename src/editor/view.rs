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
use std::thread::sleep;
use std::time::Duration;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct View<'a> {
    is_new_buffer: bool,
    buffer: &'a Buffer,
}

impl View<'_> {
    pub fn new(buffer: &Buffer) -> View {
        View {
            is_new_buffer: true,
            buffer: buffer,
        }
    }

    /// Render the current state of the editor, called in the main loop
    pub fn render(self: &mut Self, editor: &mut Editor) -> Result<(), Error> {
        debug!("Rendering editor");

        if self.is_new_buffer {
            Self::welcome_message(&mut editor.caret)?;
            read()?;
            self.is_new_buffer = false;
            Self::set_size(&mut editor.caret)?;
            let size = Terminal::size()?;
            Self::clear_screen(&mut editor.caret, size)?;
            Self::draw_buffer(self.buffer, &mut editor.caret)?;
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
            Self::set_size(&mut editor.caret)?;
            Terminal::move_caret_to(editor.caret.location.into())?;
            // Self::draw_rows(&mut editor.caret)?;
        }

        Terminal::show_caret()?;
        Terminal::execute()?;
        Ok(())
    }

    pub fn draw_buffer(
        buffer: &Buffer,
        caret: &mut Caret,
    ) -> Result<(), Error> {
        info!("Drawing buffer");

        caret.location = Location { x: 0, y: 0 };
        Terminal::move_caret_to(caret.location.into())?;
        let lines: &Vec<String> = &buffer.lines;
        for line in lines {
            Terminal::print(line.as_str())?;
            Terminal::print("\r\n")?;
        }

        let location: Location = Location { x: 0, y: 0 };
        caret.location = location;
        Ok(())
    }

    fn set_size(caret: &mut Caret) -> Result<(), Error> {
        let size = Terminal::size()?;
        caret.size = size;
        Ok(())
    }

    pub fn draw_rows(caret: &mut Caret) -> Result<(), Error> {
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
