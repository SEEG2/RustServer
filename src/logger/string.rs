use crate::logger::logger;
use crate::logger::logger::Highlight::{ERROR, INFO, SUCCESS, WARNING};

pub trait Logger {
    fn info(&self) -> String;

    fn success(&self) -> String;

    fn warning(&self) -> String;

    fn error(&self) -> String;
}

impl Logger for &str {
    fn info(&self) -> String {
        println!("{}", logger::log(&self, INFO));
        self.to_string()
    }

    fn success(&self) -> String {
        println!("{}", logger::log(self, SUCCESS));
        self.to_string()
    }

    fn warning(&self) -> String {
        println!("{}", logger::log(self, WARNING));
        self.to_string()
    }

    fn error(&self) -> String {
        println!("{}", logger::log(self, ERROR));
        self.to_string()
    }
}

impl Logger for String {
    fn info(&self) -> String {
        println!("{}", logger::log(&self, INFO));
        self.to_string()
    }

    fn success(&self) -> String {
        println!("{}", logger::log(self, SUCCESS));
        self.to_string()
    }

    fn warning(&self) -> String {
        println!("{}", logger::log(self, WARNING));
        self.to_string()
    }

    fn error(&self) -> String {
        println!("{}", logger::log(self, ERROR));
        self.to_string()
    }
}