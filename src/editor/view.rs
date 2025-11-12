use crate::editor::Caret;
use crate::editor::Size;
use crate::editor::Terminal;
use std::io::Error;

pub struct View {}

impl View {
    pub fn default() -> Self {
        Self {}
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
}
