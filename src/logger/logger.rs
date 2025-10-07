use std::collections::LinkedList;
use std::f32::INFINITY;
use std::sync::Mutex;
use lazy_static::lazy_static;
use crate::rich_text::colorize::Colorize;

#[derive(Copy, Clone)]
pub enum Highlight {
    INFO,
    SUCCESS,
    WARNING,
    ERROR
}

lazy_static! {
    static ref LOG_ENTRIES: Mutex<LinkedList<Entry>> = Mutex::new(LinkedList::new());
}
const MAX_ENTRIES: i32 = -1;

pub fn log(raw: &str, highlight: Highlight) -> String {
    let mut entries = LOG_ENTRIES.lock().unwrap();
    if MAX_ENTRIES != -1 && entries.len() >= MAX_ENTRIES as usize {
         entries.pop_back();
    }
    entries.push_front(Entry::new(raw.to_string(), highlight));

    match highlight {
        Highlight::INFO => raw.to_string(),
        Highlight::SUCCESS => raw.green(),
        Highlight::WARNING => raw.yellow(),
        Highlight::ERROR => raw.red()
    }
}

pub struct Entry {
    text: String,
    highlight: Highlight,
}

impl Entry {
    pub fn new(text: String, highlight: Highlight) -> Self {
        Entry { text, highlight }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn highlight(&self) -> Highlight {
        self.highlight
    }
}