use std::io::{self, Write};

mod color;
mod format;

pub struct Output {}

impl Output {
    pub fn new() -> Self {
        Self {}
    }

    pub fn write<T: ToString>(&self, content: T) {
        print!("{}", content.to_string());
        self.flush();
    }

    pub fn flush(&self) {
        io::stdout().flush().unwrap();
    }
}
