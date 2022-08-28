/// Represents any errors that may occur in this library
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum MexeError {
    /// Invalid character at the specified index
    InvalidCharacter(usize),

    /// Unexpected character (u8) at index
    UnexpectedCharacter(u8, usize),

    /// Binary expression should be: number operator number
    InvalidBinaryExpression,
    MissingOperand,
    MissingOperator,
    /// Unexpected token
    UnexpectedToken(String),
    InternalParserError,
    UnexpectEndOfInput,
}

impl std::error::Error for MexeError {}

impl std::fmt::Display for MexeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            MexeError::InvalidCharacter(index) => write!(f, "Invalid character at index {}", index),
            MexeError::UnexpectedCharacter(_character, index) => {
                write!(f, "Unexpected character at index {}", index)
            }
            MexeError::InvalidBinaryExpression => write!(f, "Invalid binary expression"),
            MexeError::MissingOperand => write!(f, "Missing operand"),
            MexeError::MissingOperator => write!(f, "Missing operator"),
            MexeError::UnexpectedToken(token) => write!(f, "Unexpected token: `{}`", token),
            MexeError::InternalParserError => write!(f, "Internal parser error"),
            MexeError::UnexpectEndOfInput => write!(f, "Unexpected end of input"),
        }
    }
}

/// Represents the result of any fallible operation in this library
pub type Result<T> = std::result::Result<T, MexeError>;
