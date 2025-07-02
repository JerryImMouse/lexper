// Well, error module. Of course, i can use thiserror to setup it without boilerplate
// But I don't want to use any dependencies in this project, so here is the boilerplate code :)

use std::{error, num::ParseFloatError};

use crate::lexer::OperatorType;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    ParseNumError(ParseFloatError),
    InvalidUnaryOperator(OperatorType),
    UnexpectedValue {
        found: Option<String>,
        expected: String,
    },
    UndefinedIdentifier(String),
    UnknownChar {
        row: usize,
        col: usize,
        char: char,
    },
    Other(String),
}

impl Error {
    pub fn invalid_unary_op(op: &OperatorType) -> Self {
        Self::InvalidUnaryOperator(op.to_owned())
    }

    pub fn unexpected_value(expected: &str, found: Option<&str>) -> Self {
        Self::UnexpectedValue {
            found: found.map(|x| x.to_string()),
            expected: expected.to_string(),
        }
    }

    pub fn unknown_char(row: usize, col: usize, char: char) -> Self {
        Self::UnknownChar { row, col, char }
    }

    pub fn undefined(ident: String) -> Self {
        Self::UndefinedIdentifier(ident)
    }

    pub fn other(s: String) -> Self {
        Self::Other(s)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IoError(e) => write!(f, "{}", e),
            Self::InvalidUnaryOperator(op) => {
                write!(f, "Invalid unary operator usage. Operator: {:?}", op)
            }
            Self::UnexpectedValue { found, expected } => {
                if let Some(found) = found {
                    write!(f, "Expected: '{}', but got: '{}'", expected, found)
                } else {
                    write!(f, "Expected: '{}', but nothing found", expected)
                }
            }
            Self::ParseNumError(e) => write!(f, "{}", e),
            Self::UndefinedIdentifier(i) => write!(f, "Undefined identifier met: {}", i),
            Self::UnknownChar { row, col, char } => write!(
                f,
                "Unknown character met at: Row: {}, Column: {}, Character: {}",
                row, col, char
            ),
            Self::Other(s) => write!(f, "{s}"),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::IoError(e) => Some(e),
            _ => None,
        }
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        self.source()
    }
}

// Froms
impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}

impl From<ParseFloatError> for Error {
    fn from(value: ParseFloatError) -> Self {
        Self::ParseNumError(value)
    }
}
