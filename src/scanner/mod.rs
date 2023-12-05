pub mod scanner;

use crate::Result;
pub use scanner::*;
use std::fmt::Display;

#[derive(Debug)]
pub struct ScanError {
    line: usize,
    message: String,
}

impl ScanError {
    pub fn new(line: usize, message: String) -> ScanError {
        ScanError { line, message }
    }
}

impl Display for ScanError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} line. {}", self.line, self.message)
    }
}

impl std::error::Error for ScanError {}

pub trait Scan {
    fn is_at_end(&self) -> bool;

    fn peek(&self) -> Option<char>;

    fn accumulate(&mut self);

    fn advance(&mut self) -> Option<char> {
        let char = self.peek();
        if char.is_some() {
            self.accumulate();
        }
        char
    }

    fn peek_or_advance<F>(&mut self, f: F) -> Option<bool>
    where
        F: Fn(char) -> bool,
    {
        match self.peek() {
            Some(next_char) if f(next_char) => {
                self.accumulate();
                Some(true)
            }
            Some(_) => Some(false),
            None => None,
        }
    }

    fn advance_until<F>(&mut self, f: F) -> Option<char>
    where
        F: Fn(char) -> bool,
    {
        while let Some(char) = self.advance() {
            if f(char) {
                return Some(char);
            }
        }
        None
    }

    fn advance_except<F>(&mut self, f: F) -> Option<char>
    where
        F: Fn(char) -> bool,
    {
        while let Some(char) = self.peek() {
            if f(char) {
                return Some(char);
            }
            self.accumulate();
        }
        None
    }

    fn scan(&mut self) -> Result<()>;
}
