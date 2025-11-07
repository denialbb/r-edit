use crossterm::cursor::MoveTo;
use crossterm::execute;
use crossterm::terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode, size};

pub struct Terminal {
    stdout: std::io::Stdout,
}

impl Terminal {
    pub fn default() -> Self {
        Terminal {
            stdout: std::io::stdout(),
        }
    }

    pub fn initialize(&mut self) -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        self.clear_screen()
    }

    pub fn terminate() -> Result<(), std::io::Error> {
        disable_raw_mode()
    }

    pub fn clear_screen(&mut self) -> Result<(), std::io::Error> {
        execute!(self.stdout, MoveTo(1, 1))?;
        execute!(self.stdout, Clear(ClearType::All))
    }

    pub fn draw_rows(&mut self) -> Result<(), std::io::Error> {
        let height = size()?;
        let pos = crossterm::cursor::position()?;

        execute!(self.stdout, MoveTo(0, pos.1 + 2))?;

        for _ in pos.1 + 1..height.1 {
            println!("\r~");
        }

        execute!(self.stdout, MoveTo(pos.0, pos.1))?;

        Ok(())
    }
}
