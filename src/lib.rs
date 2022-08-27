//! ![tests](https://github.com/yds12/mexe/actions/workflows/unit.yml/badge.svg)
//!
//! **m**athematical **ex**pression **e**valuator.
//!
//! ## How to Use
//!
//!     use mexe::eval;
//!
//!     fn main() {
//!         let forty_six = eval("(5 * 8) + 6").unwrap();
//!         let two = eval("1 + 1").unwrap();
//!         println!("{} & {}", forty_six, two);
//!
//!         assert_eq!(forty_six, 46.0);
//!         assert_eq!(two, 2.0);
//!     }
//!
//! Note: the above `assert_eq`s work, but for float comparison in general use a
//! crate such as `float-cmp`.
//!
//! ## Why?
//!
//! If you need to evaluate simple arithmetic expressions, this crate offers a fast
//! and lightweight solution.
//!
//! In our [current benchmarks](https://github.com/yds12/mexe/actions/workflows/bench.yml),
//! it's about 4-10x faster than `meval` and about 2x
//! faster than `fasteval`. Note that those crates do much more than `mexe`. Our focus
//! on a very small problem makes it easier for us to ship a fast and lean library.
//!
//! ## Includes
//!
//! - sum
//! - subtraction
//! - multiplication
//! - division
//! - integers
//! - floats
//! - parentheses
//! - arbitrary whitespace
//!
//! ## Goals
//!
//! - Minimal
//! - Fast: O(n)
//! - No allocations if possible
//! - We can assume the input is ASCII, and throw an error otherwise
//! - Thoroughly tested
//! - Maybe try to make it no-std
//!
//! ## Grammar
//!
//! See readme.
//!
//! Grammar idea adapted from [this post](https://stackoverflow.com/a/23845375).
//!
//! Our first implementation uses an LL(1) parser.
//!
//! ## Similar Projects
//!
//! - [meval](https://crates.io/crates/meval)
//! - [fasteval](https://crates.io/crates/fasteval)
//! - [pmalmgren/rust-calculator](https://github.com/pmalmgren/rust-calculator)
//! - [adriaN/simple_rust_parser](https://github.com/adrianN/simple_rust_parser)
//!
//! ## Links
//!
//! * Documentation: [docs.rs](https://docs.rs/mexe/latest)
//! * Crate: [crates.io](https://crates.io/crates/mexe)
//! * Repository: [Github](https://github.com/yds12/mexe)

mod consts;
mod lex;
mod parser;

use consts::*;

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

#[derive(Clone, Copy, Debug, PartialEq)]
enum Operator {
    Plus = PLUS as isize,
    Minus = MINUS as isize,
    Mul = ASTERISK as isize,
    Div = SLASH as isize,
}

impl std::fmt::Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Operator::Plus => write!(f, "+"),
            Operator::Minus => write!(f, "-"),
            Operator::Mul => write!(f, "*"),
            Operator::Div => write!(f, "/"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Token {
    LPar,
    RPar,
    Number(f64),
    Op(Operator),
    EOI, // end of input
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Token::LPar => write!(f, "("),
            Token::RPar => write!(f, ")"),
            Token::Number(n) => write!(f, "{}", n),
            Token::Op(op) => write!(f, "{}", op),
            Token::EOI => write!(f, "EOI"),
        }
    }
}

/// Evaluates a numeric expression.
///
/// The expression can contain integers, floats, sums, subtractions,
/// multiplications, divisions and can use parentheses. Whitespace is ignored.
///
/// `T`: type of the expression. Usually a `&str` or a `String`.
///
/// ```
/// # fn main() -> Result<(), mexe::MexeError> {
/// let x = mexe::eval("2 * (1 + 1)")?;
/// # Ok(())
/// # }
/// ```
///
/// # Errors
///
/// This function will return a [`MexeError`] if the input is not a valid
/// arithmetic expression.
pub fn eval<T>(expression: T) -> Result<f64>
where
    T: AsRef<str>,
{
    let tokens = lex::get_tokens(expression.as_ref())?;
    parser::parse_and_evaluate(tokens)
}

/// Evaluates a numeric expression assuming it is just one operation between
/// two numbers, without parentheses. Whitespace is ignored.
///
/// `T`: type of the expression. Usually a `&str` or a `String`.
///
/// ```
/// # fn main() -> Result<(), mexe::MexeError> {
/// let x = mexe::eval_binary("2 * 7")?;
/// # Ok(())
/// # }
/// ```
///
/// # Errors
///
/// This function will return a [`MexeError`] if the input is not a valid
/// binary arithmetic expression.
pub fn eval_binary<T>(expression: T) -> Result<f64>
where
    T: AsRef<str>,
{
    let tokens = lex::get_tokens(expression.as_ref())?;

    if tokens.len() != 4 || tokens[3] != Token::EOI {
        return Err(MexeError::InvalidBinaryExpression);
    }

    let lhs = match tokens.get(0).unwrap() {
        Token::Number(n) => n,
        _ => return Err(MexeError::MissingOperand),
    };

    let rhs = match tokens.get(2).unwrap() {
        Token::Number(n) => n,
        _ => return Err(MexeError::MissingOperand),
    };

    match tokens.get(1).unwrap() {
        Token::Op(Operator::Plus) => Ok(lhs + rhs),
        Token::Op(Operator::Minus) => Ok(lhs - rhs),
        Token::Op(Operator::Mul) => Ok(lhs * rhs),
        Token::Op(Operator::Div) => Ok(lhs / rhs),
        _ => Err(MexeError::MissingOperator),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! float_eq {
        ($op1:expr, $op2:expr) => {
            assert!(float_cmp::approx_eq!(f64, $op1, $op2));
        };
    }

    #[test]
    fn eval_does_not_panic_with_empty_input() {
        let _val = eval("");

        let empty: &[u8] = &[];
        let _val = eval(std::str::from_utf8(empty).unwrap());
    }

    #[test]
    fn eval_does_not_panic_with_bad_input() {
        let exprs = ["(1"];

        for expr in exprs.iter() {
            let _val = eval(expr);
        }
    }

    #[test]
    fn test_eval() {
        float_eq!(1.0, eval("1").unwrap());
        float_eq!(-1.0, eval("-1").unwrap());
        float_eq!(1.0, eval("(1)").unwrap());
        float_eq!(1.0, eval("((1))").unwrap());
        float_eq!(-1.0, eval("-(1)").unwrap());

        float_eq!(2.0, eval("1 + 1").unwrap());
        float_eq!(0.0, eval("1 - 1").unwrap());
        float_eq!(1.1, eval("(1+1.1) - 1").unwrap());
        float_eq!(1.1, eval("(1+(1.1)) - 1").unwrap());
        float_eq!(1.1, eval("(1+(1.1 + 0)) - 1").unwrap());
        float_eq!(3.0, eval("(1+(1.0 + 0) + 2) - 1").unwrap());
        float_eq!(2.0, eval("(((1))) + ((((1))))").unwrap());

        float_eq!(21.0, eval("(1 + (4 * 5))").unwrap());
        float_eq!(10.5, eval("(1 + (4 * 5)) / 2").unwrap());
        float_eq!(18.0, eval("1 + (4 * 5) - 9 / 3").unwrap());
        float_eq!(8.4, eval("(1 + (4 * 5)) / 2 - 3 * 0.7").unwrap());
        float_eq!(9.9, eval("(1 + ((4 * 5) + (3))) / 2 - 3 * 0.7").unwrap());
        float_eq!(0.45, eval("0.15 + 0.15 + 0.15").unwrap());
    }

    #[test]
    fn test_eval_failures() {
        let exprs = ["(((1", "((1", "(1", "1)))", "1))", "1)"];

        for expr in exprs.iter() {
            if let Ok(_) = eval(expr) {
                panic!("{} should not be parsed", expr);
            }
        }
    }

    #[test]
    fn test_eval_binary() {
        float_eq!(3.0, eval_binary("1 + 2").unwrap());
        float_eq!(10.0, eval_binary("2*5").unwrap());
        float_eq!(1.1, eval_binary("5.5/5").unwrap());
        float_eq!(10.5, eval_binary(" 5.5  + 5 ").unwrap());
    }

    #[test]
    fn correct_errors_are_returned() {
        assert_eq!(eval("1++"), Err(MexeError::UnexpectedToken("+".to_owned())));
    }
}
