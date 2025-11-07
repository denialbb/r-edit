use crossterm::cursor::MoveTo;
use crossterm::execute;
use crossterm::terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode, size};
use std::io::stdout;

pub struct Terminal {}

impl Terminal {
    pub fn initialize() -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_cursor_to(0, 0)?;
        Ok(())
    }

    pub fn terminate() -> Result<(), std::io::Error> {
        disable_raw_mode()?;
        Ok(())
    }

    pub fn clear_screen() -> Result<(), std::io::Error> {
        execute!(stdout(), Clear(ClearType::All))?;
        Ok(())
    }

    pub fn move_cursor_to(x: u16, y: u16) -> Result<(), std::io::Error> {
        execute!(stdout(), MoveTo(x, y))?;
        Ok(())
    }

    pub fn size() -> Result<(u16, u16), std::io::Error> {
        size()
    }

    pub fn draw_rows() -> Result<(), std::io::Error> {
        let height = size()?.1;
        let pos = crossterm::cursor::position()?;

        for current_line in 0..height {
            print!("~");
            if current_line < height - 1 {
                execute!(stdout(), MoveTo(0, current_line + 1))?;
                print!("\r\n");
            }
        }

        Ok(())
    }
}
