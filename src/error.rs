use std::{io, process};

use colored::Colorize;

use crate::parse;

pub type HarmoniconResult<T> = Result<T, HarmoniconError>;

#[derive(thiserror::Error, Debug)]
pub enum HarmoniconError {
    #[error("{0}")]
    SyntaxError(Box<pest::error::Error<parse::Rule>>),

    #[error("Expected '{0}', found '{1}'")]
    TypeError(&'static str, &'static str),

    #[error("Could not find block with name '{0}'")]
    UnknownBlock(String),

    #[error("Unknown property '{0}' for block type '{1}'")]
    UnknownProperty(String, &'static str),

    #[error("{0}")]
    IO(#[from] io::Error),

    #[error("{0}")]
    FSNotify(#[from] notify::Error),
}

impl From<pest::error::Error<parse::Rule>> for HarmoniconError {
    fn from(value: pest::error::Error<parse::Rule>) -> Self {
        HarmoniconError::SyntaxError(Box::new(value))
    }
}

impl HarmoniconError {
    pub fn warn(&self) {
        eprintln!("{} {}", "Warning:".yellow(), self);
    }

    pub fn resolve(&self) -> ! {
        eprintln!("{} {}", "Error:".red(), self);
        process::exit(1);
    }
}


pub fn resolve<T>(result: HarmoniconResult<T>) -> T {
    match result {
        Ok(t) => t,
        Err(e) => e.resolve(),
    }
}
