use crate::parse;

pub type HarmoniconResult<T> = Result<T, HarmoniconError>;

#[derive(thiserror::Error, Debug)]
pub enum HarmoniconError {
    #[error("Syntax Error: {0}")]
    SyntaxError(Box<pest::error::Error<parse::Rule>>),

    #[error("Type Error: Expected '{0}', found '{1}'")]
    TypeError(&'static str, &'static str),

    #[error("Unknown Block Error: Could not find block with name '{0}'")]
    UnknownBlock(String),

    #[error("Unknown Property Error: Unknown property '{0}' for block type '{1}'")]
    UnknownProperty(String, &'static str),
}

impl From<pest::error::Error<parse::Rule>> for HarmoniconError {
    fn from(value: pest::error::Error<parse::Rule>) -> Self {
        HarmoniconError::SyntaxError(Box::new(value))
    }
}


