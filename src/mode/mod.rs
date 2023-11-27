pub mod running;

pub use running::*;
use std::io::BufRead;

use crate::Result;

pub trait Mode {
    fn run<F>(&self, f: F) -> Result<()>
    where
        F: Fn() -> Result<()>;

    fn reader(&self) -> Result<Box<dyn BufRead>>;

    fn read<R>(&self) -> fn(&mut R, &mut String) -> std::io::Result<usize>
    where
        R: BufRead;
}
