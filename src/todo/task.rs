use std::fmt::Display;
use std::ops::Deref;
type Status = bool;
#[derive(Clone)]
pub struct Task{
    pub text: String,
    pub status: Status
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

impl Task {
    pub fn is_done(&mut self) {
        self.status = true
    }
}