use crossterm::{
    cursor::{MoveTo, MoveToColumn},
    event::{Event, KeyCode, KeyEventKind, poll, read},
    execute,
    terminal::{Clear, ClearType, size},
};
use std::io;
use std::time::Duration;

use crate::cmd::output;

pub struct Terminal {
    prompt: String,
    state: String,
    enter: bool,
    output: output::Output,
}

impl Default for Terminal {
    fn default() -> Self {
        Self {
            state: String::new(),
            prompt: "$ ".into(),
            enter: false,
            output: output::Output::new(),
        }
    }
}

impl Terminal {
    pub fn set_prompt<T: ToString>(&mut self, prompt: T) {
        self.prompt = prompt.to_string();
    }

    pub fn get_size() -> (u16, u16) {
        match size() {
            Ok((w, h)) => (w, h),
            Err(_) => (80, 24),
        }
    }

    pub fn read_line(&self) -> String {
        self.state.clone()
    }

    pub fn read(&mut self) -> String {
        loop {
            if poll(Duration::from_millis(10)).unwrap() {
                if let Event::Key(event) = read().unwrap() {
                    if event.kind == KeyEventKind::Press {
                        match event.code {
                            KeyCode::Char(c) => {
                                self.state.push(c);
                                return c.to_string();
                            }
                            KeyCode::Enter => {
                                self.enter = true;
                                self.state.push('\n');
                                return "\n".into();
                            }
                            KeyCode::Backspace => {
                                self.state.pop();
                                return "\x08".into();
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    pub fn new_line(&mut self) {
        self.output.write("\n");
        self.state.clear();
        self.enter = false;
        self.write_with_prompt("");
    }

    pub fn write<T: ToString>(&self, msg: T) {
        self.clear_line();

        self.output.write(msg);
    }

    pub fn write_with_prompt<T: ToString>(&self, msg: T) {
        self.write(format!("{}{}", self.prompt, msg.to_string()));
    }

    pub fn clear_line(&self) {
        let mut stdout = io::stdout();

        execute!(stdout, MoveToColumn(0), Clear(ClearType::CurrentLine)).unwrap();
    }

    pub fn clear_screen(&self) {
        let mut stdout = io::stdout();

        execute!(stdout, MoveTo(0, 0), Clear(ClearType::All)).unwrap();
    }

    pub fn is_enter(&mut self) -> bool {
        self.state.ends_with("\n")
    }
}
