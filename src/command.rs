use clap::Parser;

use crate::{lox::Lox, mode::RunningMode, Analylize};

#[derive(Parser)]
#[command(name = "rlox")]
#[command(author = "xiuxiubiu <xiuxiubiu@gmail.com>")]
#[command(version = "0.0.1")]
#[command(about = "Implementation of lox language interpreter using Rust.")]
#[command(next_line_help = false)]
#[command(ignore_errors = false)]
#[command(override_usage = "rlox [script path]")]
pub struct Args {
    #[arg(allow_hyphen_values = false)]
    #[arg(hide_default_value = true)]
    #[arg(hide = true)]
    path: Option<String>,
}

pub fn init() -> Args {
    Args::parse()
}

impl Args {
    pub fn run(self) {
        let mode = match self.path {
            Some(path) => RunningMode::File(path),
            None => RunningMode::Prompt,
        };

        Lox::new(mode).main().analylize();
    }
}
