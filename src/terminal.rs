use crossterm::cursor::MoveTo;
use crossterm::execute;
use crossterm::terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode, size};

pub struct Terminal;

impl Terminal {
    pub fn initialize() -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        Self::clear_screen()
    }

    pub fn terminate() -> Result<(), std::io::Error> {
        disable_raw_mode()
    }

    pub fn clear_screen() -> Result<(), std::io::Error> {
        let mut stdout = std::io::stdout();
        execute!(stdout, MoveTo(1, 1))?;
        execute!(stdout, Clear(ClearType::All))
    }

    pub fn draw_rows() -> Result<(), std::io::Error> {
        // TODO draw ~ every row
        let mut stdout = std::io::stdout();
        let height = size()?;
        let pos = crossterm::cursor::position()?;

        execute!(stdout, MoveTo(0, pos.1 + 2))?;

        for _ in pos.1 + 1..height.1 {
            println!("\r~");
        }

        execute!(stdout, MoveTo(pos.0, pos.1))?;

        Ok(())
    }
}
