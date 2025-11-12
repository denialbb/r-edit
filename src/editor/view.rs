use crate::editor::Caret;
use crate::editor::Editor;
use crate::editor::Size;
use crate::editor::Terminal;
use crate::editor::debug;
use crate::editor::info;
use crate::editor::terminal::Location;
use crate::editor::terminal::Position;
use std::io::Error;
use std::thread::sleep;
use std::time::Duration;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct View {}

impl View {
    pub fn default() -> Self {
        Self {}
    }

    pub fn render(editor: &mut Editor) -> Result<(), Error> {
        Self::draw_rows(&mut editor.caret)?;
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
            Terminal::move_caret_to(editor.caret.location.into())?;
        }
        Terminal::show_caret()?;
        Terminal::execute()?;
        Ok(())
    }

    pub fn draw_rows(caret: &mut Caret) -> Result<(), Error> {
        let Size { height, width } = Terminal::size()?;
        caret.size = Size { height, width };

        Terminal::print("\r\n")?;
        if caret.location.x == 0 {
            if caret.location.y == 0 {
                Terminal::clear_down()?;
                for current_line in caret.location.y + 1..height {
                    Terminal::print("~")?;
                    if current_line < height - 1 {
                        Terminal::print("\r\n")?;
                    }
                }
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
