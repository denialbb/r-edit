use crate::editor::terminal::Size;
use crate::editor::terminal::{Location, Terminal};
use std::io::Error;

#[derive(Debug, Copy, Clone)]
pub struct Caret {
    // position should probably be responsibility of the terminal
    pub location: Location,
    pub size: Size,
}
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Caret {
    pub fn default() -> Self {
        Self {
            location: Location { x: 0, y: 0 },
            size: Size {
                height: 0,
                width: 0,
            },
        }
    }

    pub fn shift(&mut self, direction: Direction) {
        match direction {
            Direction::Up => {
                log::info!("Shifting up");
                if self.location.y > 0 {
                    self.location.y -= 1;
                }
            }
            Direction::Down => {
                log::info!("Shifting down");
                if self.location.y < self.size.height - 1 {
                    self.location.y += 1;
                }
            }
            Direction::Left => {
                log::info!("Shifting left");
                if self.location.x > 0 {
                    self.location.x -= 1;
                }
            }
            Direction::Right => {
                log::info!("Shifting right");
                if self.location.x < self.size.width - 1 {
                    self.location.x += 1
                }
            }
        }
        Terminal::move_caret_to(self.location.into()).unwrap();
    }
}
