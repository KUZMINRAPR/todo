use std::fmt::Display;
use std::ops::Deref;
use ratatui::text::{Line, Text};

#[derive(Clone,Eq, PartialEq)]
pub struct Task{
    pub text: String,
    pub status: bool
}

impl Task{
    pub fn new(text: String) -> Self{
        Self{
            text,
            status: false
        }
    }
}

impl Display for Task{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text)
    }
}

impl From<&Task> for Text<'_> {
    fn from(value: &Task) -> Self {
        Self::from(format!("{} {}", value.text, value.status))
    }
}
impl Task {
    pub fn is_done(&mut self) {
        self.status = true
    }
}