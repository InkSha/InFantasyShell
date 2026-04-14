use crossterm::{
    cursor::MoveToColumn,
    event::{Event, KeyCode, KeyEventKind, poll, read},
    execute,
    terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode},
};
use std::io::{self, Write};
use std::time::Duration;

pub struct Context {
    prompt: String,
    state: String,
    enter: bool,
}

pub enum ContextSignal {
    NONE,
    EXIT,
}

impl Context {
    pub fn default() -> Self {
        Self {
            state: String::new(),
            prompt: "INFS> ".into(),
            enter: false,
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
        self.write_with_prompt(self.read_line());
        self.state.clear();
        self.write_with_prompt("");
    }

    pub fn write<T: ToString>(&self, msg: T) {
        self.clear();

        print!("{}", msg.to_string());

        self.flush();
    }

    pub fn write_with_prompt<T: ToString>(&self, msg: T) {
        self.write(format!("{}{}", self.prompt, msg.to_string()));
    }

    pub fn flush(&self) {
        io::stdout().flush().unwrap();
    }

    pub fn clear(&self) {
        let mut stdout = io::stdout();

        execute!(stdout, MoveToColumn(0), Clear(ClearType::CurrentLine)).unwrap();
    }

    pub fn is_enter(&mut self) -> bool {
        self.state.ends_with("\n")
    }
}

pub type ExecuteHandle = fn(context: &mut Context) -> ContextSignal;

pub fn execute(handle: ExecuteHandle) {
    enable_raw_mode().unwrap();

    let mut ctx = Context::default();

    ctx.write_with_prompt("");

    loop {
        match handle(&mut ctx) {
            ContextSignal::EXIT => break,
            ContextSignal::NONE => (),
        }
    }

    disable_raw_mode().unwrap();
}
