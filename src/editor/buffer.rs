use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub struct Buffer {
    pub lines: Vec<String>,
}

impl Buffer {
    pub fn new() -> Buffer {
        Buffer { lines: Vec::new() }
    }

    pub fn push(&mut self, line: String) {
        self.lines.push(line);
    }

    pub fn pop(&mut self) -> Option<String> {
        self.lines.pop()
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
}
