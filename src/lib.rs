pub mod command;
pub mod lox;
pub mod mode;
pub mod scanner;
pub mod token;

use std::{
    fmt::Display,
    io::{stdout, Write},
    process,
};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub trait Analylize {
    fn analylize(&self) -> !;
}

impl<T> Analylize for Result<T> {
    fn analylize(&self) -> ! {
        stdout().lock().flush().unwrap();
        match self {
            Ok(_) => process::exit(0),
            Err(_) => process::exit(1),
        }
    }
}

pub trait PrintResult {
    fn print(&self) -> &Self;
}

impl<T> PrintResult for Result<T>
where
    T: Display,
{
    fn print(&self) -> &Self {
        match self {
            Ok(display) => print!("{}", display),
            Err(err) => eprint!("{}", err.to_string()),
        }
        self
    }
}
