use crate::rich_text::text::{format_text, Format};

pub trait Formatize {
    fn bold(&self) -> String;
    fn italic(&self) -> String;
    fn underline(&self) -> String;
    fn dime(&self) -> String;
}

impl Formatize for &str {
    fn bold(&self) -> String {
        format_text(self, Format::BOLD)
    }

    fn italic(&self) -> String {
        format_text(self, Format::ITALIC)
    }

    fn underline(&self) -> String {
        format_text(self, Format::UNDERLINE)
    }

    fn dime(&self) -> String {
        format_text(self, Format::DIM)
    }
}

impl Formatize for String {
    fn bold(&self) -> String {
        format_text(self, Format::BOLD)
    }

    fn italic(&self) -> String {
        format_text(self, Format::ITALIC)
    }

    fn underline(&self) -> String {
        format_text(self, Format::UNDERLINE)
    }

    fn dime(&self) -> String {
        format_text(self, Format::DIM)
    }
}