use crossterm::{
    cursor::{MoveTo, MoveToColumn},
    event::{Event, KeyCode, KeyEventKind, poll, read},
    execute,
    terminal::{Clear, ClearType},
};
use std::io::{self, Write};
use std::time::Duration;

pub struct Context {
    prompt: String,
    state: String,
    enter: bool,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            state: String::new(),
            prompt: "$ ".into(),
            enter: false,
        }
    }
}

impl Context {
    pub fn set_prompt<T: ToString>(&mut self, prompt: T) {
        self.prompt = prompt.to_string();
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
        print!("\n");
        self.flush();
        self.state.clear();
        self.enter = false;
        self.write_with_prompt("");
    }

    pub fn write<T: ToString>(&self, msg: T) {
        self.clear_line();

        print!("{}", msg.to_string());

        self.flush();
    }

    pub fn write_with_prompt<T: ToString>(&self, msg: T) {
        self.write(format!("{}{}", self.prompt, msg.to_string()));
    }

    pub fn flush(&self) {
        io::stdout().flush().unwrap();
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
