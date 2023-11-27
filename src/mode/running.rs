use super::Mode;
use crate::Result;
use std::{
    fs,
    io::{stdin, stdout, BufRead, BufReader, Read, Write},
    path::Path,
};

pub enum RunningMode<P>
where
    P: AsRef<Path>,
{
    File(P),
    Prompt,
}

impl<P> Mode for RunningMode<P>
where
    P: AsRef<Path>,
{
    fn run<F>(&self, f: F) -> Result<()>
    where
        F: Fn() -> Result<()>,
    {
        match self {
            RunningMode::File(_) => f(),
            RunningMode::Prompt => {
                loop {
                    print!("> ");
                    stdout().lock().flush()?;

                    f()?;
                }

                #[allow(unreachable_code)]
                Ok(())
            }
        }
    }

    fn reader(&self) -> Result<Box<dyn BufRead>> {
        match self {
            RunningMode::File(path) => {
                let file = fs::File::open(path)?;
                Ok(Box::new(BufReader::new(file)))
            }
            RunningMode::Prompt => Ok(Box::new(stdin().lock())),
        }
    }

    fn read<R>(&self) -> fn(&mut R, &mut String) -> std::io::Result<usize>
    where
        R: BufRead,
    {
        match self {
            RunningMode::File(_) => Read::read_to_string,
            RunningMode::Prompt => BufRead::read_line,
        }
    }
}
