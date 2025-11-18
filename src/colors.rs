// ANSI color codes for terminal output
pub struct Color;

impl Color {
    pub const RESET: &'static str = "\x1b[0m";
    pub const RED: &'static str = "\x1b[31m";
    pub const GREEN: &'static str = "\x1b[32m";
    pub const YELLOW: &'static str = "\x1b[33m";
    pub const BLUE: &'static str = "\x1b[34m";
    pub const MAGENTA: &'static str = "\x1b[35m";
    pub const CYAN: &'static str = "\x1b[36m";
    pub const BOLD: &'static str = "\x1b[1m";
    pub const DIM: &'static str = "\x1b[2m";
}

pub fn success(msg: &str) -> String {
    format!("{}✓{} {}", Color::GREEN, Color::RESET, msg)
}

pub fn error(msg: &str) -> String {
    format!("{}✗{} {}", Color::RED, Color::RESET, msg)
}

pub fn warning(msg: &str) -> String {
    format!("{}⚠{} {}", Color::YELLOW, Color::RESET, msg)
}

pub fn info(msg: &str) -> String {
    format!("{}ℹ{} {}", Color::BLUE, Color::RESET, msg)
}

pub fn highlight(msg: &str) -> String {
    format!("{}{}{}", Color::CYAN, msg, Color::RESET)
}

pub fn bold(msg: &str) -> String {
    format!("{}{}{}", Color::BOLD, msg, Color::RESET)
}

pub fn dim(msg: &str) -> String {
    format!("{}{}{}", Color::DIM, msg, Color::RESET)
}
