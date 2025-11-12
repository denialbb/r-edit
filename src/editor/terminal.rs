use core::fmt::Display;
use crossterm::Command;
use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::queue;
use crossterm::style::Print;
use crossterm::terminal::{
    Clear, ClearType, disable_raw_mode, enable_raw_mode, size,
};
use std::fmt;
use std::io::{Error, Write, stdout};

#[derive(Debug, Copy, Clone)]
///  Height and Width of a Terminal
pub struct Size {
    pub height: usize,
    pub width: usize,
}
#[derive(Debug, Copy, Clone)]
///  Represents coordinates on a Screen
pub struct Position {
    pub x: usize,
    pub y: usize,
}
#[derive(Debug, Copy, Clone)]
///  Represents coordinates on a Document
pub struct Location {
    pub x: usize,
    pub y: usize,
}

impl Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Into<Position> for Location {
    fn into(self) -> Position {
        Position {
            x: self.x,
            y: self.y,
        }
    }
}

impl Into<Location> for Position {
    fn into(self) -> Location {
        Location {
            x: self.x,
            y: self.y,
        }
    }
}

pub struct Terminal;

impl Terminal {
    pub fn initialize() -> Result<(), Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_caret_to(Position { x: 0, y: 0 })?;
        Self::execute()?;
        Ok(())
    }

    pub fn terminate() -> Result<(), Error> {
        Self::execute()?;
        disable_raw_mode()?;
        Ok(())
    }

    pub fn queue_command(command: impl Command) -> Result<(), Error> {
        queue!(stdout(), command)?;
        Ok(())
    }

    pub fn clear_screen() -> Result<(), Error> {
        log::info!("Clearing screen");
        Self::queue_command(Clear(ClearType::All))?;
        Ok(())
    }

    pub fn clear_current_line() -> Result<(), Error> {
        log::info!("Clearing current line");
        Self::queue_command(Clear(ClearType::CurrentLine))?;
        Ok(())
    }

    pub fn clear_up() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::FromCursorUp))?;
        Ok(())
    }

    pub fn clear_down() -> Result<(), Error> {
        log::info!("Clearing down");
        Self::queue_command(Clear(ClearType::FromCursorDown))?;
        Ok(())
    }

    pub fn clear_right() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::UntilNewLine))?;
        Ok(())
    }

    pub fn move_caret_to(Position { x, y }: Position) -> Result<(), Error> {
        Self::queue_command(MoveTo(x as u16, y as u16))?;
        Ok(())
    }

    pub fn show_caret() -> Result<(), Error> {
        Self::queue_command(Show)?;
        Ok(())
    }

    pub fn hide_caret() -> Result<(), Error> {
        Self::queue_command(Hide)?;
        Ok(())
    }

    /// Returns the current size of this Terminal.
    /// Edge Case for systems with `usize` < `u16`:
    /// * A `Size` representing the terminal size.
    /// Any coordinate `z` truncated to `usize` if `usize` < `z` < `u16`
    pub fn size() -> Result<Size, Error> {
        let (width, height) = size()?;
        // clippy::as_conversions: see doc above
        #[allow(clippy::as_conversions)]
        let width = width as usize;
        #[allow(clippy::as_conversions)]
        let height = height as usize;
        Ok(Size { height, width })
    }

    pub fn print(string: &str) -> Result<(), Error> {
        Self::queue_command(Print(string))?;
        Ok(())
    }

    pub fn execute() -> Result<(), Error> {
        stdout().flush()?;
        Ok(())
    }
}
