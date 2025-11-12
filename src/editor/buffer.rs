use super::terminal::Location; // Added this line
use std::io::Write;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub struct Buffer {
    pub lines: Vec<String>,
}

impl Buffer {
    pub fn default() -> Buffer {
        Buffer { lines: Vec::new() }
    }
    pub fn new() -> Buffer {
        Buffer { lines: Vec::new() }
    }

    pub fn push(&mut self, line: String) {
        self.lines.push(line);
    }

    pub fn pop(&mut self) -> Option<String> {
        self.lines.pop()
    }

    pub fn get_line(&self, index: usize) -> Option<String> {
        self.lines.get(index).cloned()
    }

    pub fn read_file(path: &str) -> Buffer {
        let mut buffer = Buffer::new();
        let file = File::open(path);
        let reader = BufReader::new(file.unwrap());

        for line in reader.lines() {
            let line = line.unwrap();
            buffer.push(line);
        }

        buffer
    }

    pub fn write_file(&self, path: &str) {
        let mut file = File::create(path).unwrap();
        for line in &self.lines {
            file.write_all(line.as_bytes()).unwrap();
        }
    }

    pub fn insert(&mut self, c: char, at: Location) {
        // Ensure the y-coordinate is within bounds or at the end for a new line
        if at.y > self.lines.len() {
            return; // Invalid y-coordinate
        }

        if c == '\n' {
            if at.y == self.lines.len() {
                // If at the end of the buffer, add a new empty line
                self.lines.push("".to_string());
            } else {
                // If in an existing line, split the line
                let current_line = &mut self.lines[at.y];
                let x_pos = std::cmp::min(at.x, current_line.len());

                let rest_of_line =
                    current_line.drain(x_pos..).collect::<String>();
                self.lines.insert(at.y + 1, rest_of_line);
            }
        } else {
            if at.y == self.lines.len() {
                // If at the end of the buffer, add a new line with the character
                self.lines.push(c.to_string());
            } else {
                let current_line = &mut self.lines[at.y];
                let x_pos = std::cmp::min(at.x, current_line.len());
                current_line.insert(x_pos, c);
            }
        }
    }

    pub fn backspace(&mut self, at: Location) {
        if at.y >= self.lines.len() {
            return;
        }

        if at.x > 0 {
            let line = &mut self.lines[at.y];
            if at.x <= line.len() {
                line.remove(at.x - 1);
            }
        } else if at.y > 0 {
            let line_to_move = self.lines.remove(at.y);
            let prev_line = &mut self.lines[at.y - 1];
            prev_line.push_str(&line_to_move);
        }
    }
}
