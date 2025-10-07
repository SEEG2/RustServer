use crate::rich_text::text::{color_text, Color};

pub trait Colorize {
    fn red(&self) -> String;
    fn green(&self) -> String;
    fn blue(&self) -> String;
    fn black(&self) -> String;
    fn white(&self) -> String;
    fn yellow(&self) -> String;
    fn magenta(&self) -> String;
    fn cyan(&self) -> String;
}

impl Colorize for &str {
    fn red(&self) -> String {
        color_text(self, Color::RED)
    }
    fn green(&self) -> String {
        color_text(self, Color::GREEN)
    }

    fn blue(&self) -> String {
        color_text(self, Color::BLUE)
    }

    fn black(&self) -> String {
        color_text(self, Color::BLACK)
    }

    fn white(&self) -> String {
        color_text(self, Color::WHITE)
    }

    fn yellow(&self) -> String {
        color_text(self, Color::YELLOW)
    }

    fn magenta(&self) -> String {
        color_text(self, Color::MAGENTA)
    }

    fn cyan(&self) -> String {
        color_text(self, Color::CYAN)
    }
}

impl Colorize for String {
    fn red(&self) -> String {
        color_text(self, Color::RED)
    }
    fn green(&self) -> String {
        color_text(self, Color::GREEN)
    }

    fn blue(&self) -> String {
        color_text(self, Color::BLUE)
    }

    fn black(&self) -> String {
        color_text(self, Color::BLACK)
    }

    fn white(&self) -> String {
        color_text(self, Color::WHITE)
    }

    fn yellow(&self) -> String {
        color_text(self, Color::YELLOW)
    }

    fn magenta(&self) -> String {
        color_text(self, Color::MAGENTA)
    }

    fn cyan(&self) -> String {
        color_text(self, Color::CYAN)
    }
}