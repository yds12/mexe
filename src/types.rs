use crate::consts::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum Operator {
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
pub(crate) enum Token {
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
