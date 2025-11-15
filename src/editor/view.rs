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
    size: Size,
    pub needs_redraw: bool,
}

impl View {
    pub fn default() -> View {
        View {
            is_new_buffer: true,
            size: Size {
                height: 40,
                width: 80,
            },
            needs_redraw: false,
        }
    }
    pub fn new(size: Size) -> View {
        View {
            is_new_buffer: true,
            size: size,
            needs_redraw: false,
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

        view.draw_caret(caret)?;

        if view.needs_redraw {
            view.refresh_screen(caret, current_buffer)?;
            view.needs_redraw = false;
        }

        Terminal::execute()?;

        Ok(())
    }

    pub fn draw_caret(self: &mut Self, caret: &mut Caret) -> Result<(), Error> {
        Terminal::move_caret_to(caret.location.into())?;
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
        Ok(())
    }

    pub fn draw_buffer(
        buffer: &Buffer,
        caret: &mut Caret,
    ) -> Result<(), Error> {
        info!("Drawing buffer");

        let size = Terminal::size()?;
        let lines: &Vec<String> = &buffer.lines;

        Terminal::clear_screen()?;
        Terminal::move_caret_to(Position { x: 0, y: 0 })?;

        for line in lines {
            debug!("Line: {}", line);
            Terminal::print(line.as_str())?;
            Terminal::print("\r\n")?;
        }

        if lines.len() < size.height {
            Self::filling_empty_lines(lines.len(), size)?;
        }

        Terminal::move_caret_to(caret.location.into())?;
        Ok(())
    }

    pub fn resize(&mut self, to: Size) {
        self.size = to;
        self.needs_redraw = true;
    }

    pub fn set_size(caret: &mut Caret) -> Result<(), Error> {
        let size = Terminal::size()?;
        caret.size = size;
        Ok(())
    }

    fn filling_empty_lines(
        current_line: usize,
        size: Size,
    ) -> Result<(), Error> {
        let height = size.height;

        for line in current_line + 1..height {
            Terminal::print("~")?;
            if line < height - 1 {
                Terminal::move_caret_to(Position { x: 0, y: line })?;
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
