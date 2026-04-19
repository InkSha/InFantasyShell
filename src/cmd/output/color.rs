#[derive(Debug, Clone)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Color {
    pub fn default() -> Self {
        Color::new(0, 0, 0)
    }

    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }

    pub fn use_color<T: ToString>(&self, content: T) -> String {
        let color_code = format!("\x1b[38;2;{};{};{}m", self.red, self.green, self.blue);
        let reset_code = "\x1b[0m";
        format!("{}{}{}", color_code, content.to_string(), reset_code)
    }
}
