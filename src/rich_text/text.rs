pub enum Color {
    RED,
    GREEN,
    BLUE,
    BLACK,
    WHITE,
    YELLOW,
    MAGENTA,
    CYAN
}

pub enum Format {
    BOLD,
    ITALIC,
    DIM,
    Underline
}

const RESET: &str = "\x1b[0m";
pub fn color_text(text: &str, color: Color) -> String {
    format!("{}{}{}", get_string_for_color(color), text, RESET)
}

pub fn format_text(text: &str, format: Format) -> String {
    format!("{}{}{}", get_string_for_format(format), text, RESET)
}
fn get_string_for_color(color: Color) -> &'static str {
    match color {
        Color::RED => "\x1b[31m",
        Color::GREEN => "\x1b[32m",
        Color::BLUE => "\x1b[34m",
        Color::BLACK => "\x1b[30m",
        Color::WHITE => "\x1b[37m",
        Color::YELLOW => "\x1b[33m",
        Color::MAGENTA => "\x1b[35m",
        Color::CYAN => "\x1b[36m"
    }
}

fn get_string_for_format(format: Format) -> &'static str {
    match format {
        Format::BOLD => "\x1b[1m",
        Format::DIM => "\x1b[2m",
        Format::ITALIC => "\x1b[3m",
        Format::Underline => "\x1b[4m"
    }
}