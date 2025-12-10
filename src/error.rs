use std::fmt;

#[derive(Debug, Clone)]
pub enum TogError {
    LexError(String, usize, usize), // message, line, column
    ParseError(String, usize, usize),
    RuntimeError(String, Option<usize>), // message, optional line number
    IoError(String),
    TypeError(String, Option<usize>), // message, optional line number
}

impl fmt::Display for TogError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TogError::LexError(msg, line, col) => {
                write!(f, "Lexer Error at line {}:{}: {}", line, col, msg)
            }
            TogError::ParseError(msg, line, col) => {
                if *line == 0 && *col == 0 {
                    write!(f, "Parse Error: {}", msg)
                } else {
                    write!(f, "Parse Error at line {}:{}: {}", line, col, msg)
                }
            }
            TogError::RuntimeError(msg, line) => {
                if let Some(ln) = line {
                    write!(f, "Runtime Error at line {}: {}", ln, msg)
                } else {
                    write!(f, "Runtime Error: {}", msg)
                }
            }
            TogError::IoError(msg) => {
                write!(f, "IO Error: {}", msg)
            }
            TogError::TypeError(msg, line) => {
                if let Some(ln) = line {
                    write!(f, "Type Error at line {}: {}", ln, msg)
                } else {
                    write!(f, "Type Error: {}", msg)
                }
            }
        }
    }
}

impl std::error::Error for TogError {}

