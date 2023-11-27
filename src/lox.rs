use crate::{mode::Mode, PrintResult, Result};
use std::io::{stdout, Write};

pub struct Lox<M> {
    mode: M,
}

impl<M> Lox<M>
where
    M: Mode,
{
    pub fn new(mode: M) -> Lox<M> {
        Lox { mode }
    }

    pub fn main(&mut self) -> Result<()> {
        self.mode.run(|| {
            let mut buffer = String::new();
            let mut r = self.mode.reader()?;

            self.mode.read()(&mut r, &mut buffer)?;

            self.run(buffer).print();
            stdout().lock().flush()?;

            Ok(())
        })
    }

    fn run(&self, s: String) -> Result<String> {
        stdout().lock().flush()?;
        Ok(s)
    }
}
