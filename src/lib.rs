// ASCII character codes
const SPACE: u8 = 32;
const LPAR: u8 = 40;
const RPAR: u8 = 41;
const ASTERISK: u8 = 42;
const PLUS: u8 = 43;
const MINUS: u8 = 45;
const PERIOD: u8 = 46;
const SLASH: u8 = 47;
const N0: u8 = 48;
const N1: u8 = 49;
const N2: u8 = 50;
const N3: u8 = 51;
const N4: u8 = 52;
const N5: u8 = 53;
const N6: u8 = 54;
const N7: u8 = 55;
const N8: u8 = 56;
const N9: u8 = 57;

/// Represents any errors that may occur in this library
#[derive(Debug)]
pub enum MexeError {
    /// Invalid character at the specified index
    InvalidCharacter(usize),

    /// Unexpected character (u8) at index
    UnexpectedCharacter(u8, usize),

    /// Binary expression should be: number operator number
    InvalidBinaryExpression,
    MissingOperand,
    MissingOperator,
    UnexpectedToken,
    InternalParserError,
}

/// Represents the result of any fallible operation in this library
pub type Result<T> = std::result::Result<T, MexeError>;

#[derive(Debug, PartialEq)]
enum Operator {
    Plus = PLUS as isize,
    Minus = MINUS as isize,
    Mul = ASTERISK as isize,
    Div = SLASH as isize,
}

#[derive(Debug, PartialEq)]
enum Token {
    LPar,
    RPar,
    Number(f64),
    Op(Operator),
    EOI, // end of input
}

fn get_tokens(expression: &str) -> Result<Vec<Token>> {
    let chars = expression.as_bytes();
    let mut tokens = Vec::with_capacity(chars.len() / 2 + 1);

    enum LexerState {
        Normal,
        ReadingNumber(usize),
        ReadingDecimals(usize),
    }

    let mut state = LexerState::Normal;

    for i in 0..chars.len() {
        let (in_number, token) = match chars[i] {
            SPACE => (false, None),
            LPAR => (false, Some(Token::LPar)),
            RPAR => (false, Some(Token::RPar)),
            ASTERISK => (false, Some(Token::Op(Operator::Mul))),
            PLUS => (false, Some(Token::Op(Operator::Plus))),
            MINUS => (false, Some(Token::Op(Operator::Minus))),
            SLASH => (false, Some(Token::Op(Operator::Div))),
            num @ (N0 | N1 | N2 | N3 | N4 | N5 | N6 | N7 | N8 | N9) => {
                state = match state {
                    LexerState::Normal => LexerState::ReadingNumber(i),
                    LexerState::ReadingNumber(_) | LexerState::ReadingDecimals(_) => state,
                };

                (true, None)
            }
            PERIOD => {
                state = match state {
                    LexerState::Normal | LexerState::ReadingDecimals(_) => {
                        return Err(MexeError::UnexpectedCharacter(PERIOD, i))
                    }
                    LexerState::ReadingNumber(n) => LexerState::ReadingDecimals(n),
                };

                (true, None)
            }
            _ => return Err(MexeError::InvalidCharacter(i)),
        };

        if !in_number {
            match state {
                LexerState::ReadingNumber(n) | LexerState::ReadingDecimals(n) => {
                    let number = std::str::from_utf8(&chars[n..i]).unwrap(); // infallible
                    let number = number
                        .parse::<f64>()
                        .unwrap_or_else(|_| panic!("input:|{}|", number)); // infallible

                    tokens.push(Token::Number(number));
                }
                _ => (),
            }

            state = LexerState::Normal;
        }

        if let Some(token) = token {
            tokens.push(token);
        }
    }

    match state {
        LexerState::ReadingNumber(n) | LexerState::ReadingDecimals(n) => {
            let number = std::str::from_utf8(&chars[n..]).unwrap(); // infallible
            let number = number
                .parse::<f64>()
                .unwrap_or_else(|_| panic!("input:|{}|", number)); // infallible

            tokens.push(Token::Number(number));
        }
        _ => (),
    }

    tokens.push(Token::EOI);
    Ok(tokens)
}

/// Evaluates a numeric expression.
///
/// The expression can contain integers, floats, sums, subtractions,
/// multiplications, divisions and can use parentheses. Whitespace is ignored.
pub fn eval(expression: &str) -> Result<f64> {
    let mut tokens = get_tokens(expression)?;
    parse_and_evaluate(tokens)
}

/// Evaluates a numeric expression assuming it is just one operation between
/// two numbers, without parentheses. Whitespace is ignored.
pub fn eval_binary(expression: &str) -> Result<f64> {
    let mut tokens = get_tokens(expression)?;

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

// PARSER

type ParserReturn = Result<(Option<f64>, usize)>;

fn parse_and_evaluate(input: Vec<Token>) -> Result<f64> {
    return ll_parse_expr(&input[..]).map(|val| val.0.unwrap());
}

fn ll_parse_expr(input: &[Token]) -> Result<(Option<f64>, &[Token])> {
    match input[0] {
        Token::LPar | Token::Number(_) | Token::Op(Operator::Minus) => {
            let (val, input) = ll_parse_term(input)?;
            ll_parse_addexpr(val.unwrap(), input)
        }
        _ => Err(MexeError::UnexpectedToken), // TODO: must include token
    }
}

fn ll_parse_addexpr(val: f64, input: &[Token]) -> Result<(Option<f64>, &[Token])> {
    match &input[0] {
        t @ (Token::Op(Operator::Plus) | Token::Op(Operator::Minus)) => {
            let (val2, input) = ll_parse_term(&input[1..])?;

            let val = match t {
                Token::Op(Operator::Plus) => val + val2.unwrap(),
                Token::Op(Operator::Minus) => val - val2.unwrap(),
                _ => unreachable!(),
            };

            ll_parse_addexpr(val, input)
        }
        Token::RPar => Ok((Some(val), input)),
        Token::Op(_) | Token::EOI => Ok((Some(val), input)),
        _ => Err(MexeError::UnexpectedToken),
    }
}

fn ll_parse_term(input: &[Token]) -> Result<(Option<f64>, &[Token])> {
    match input[0] {
        Token::LPar | Token::Number(_) | Token::Op(Operator::Minus) => {
            let (val, input) = ll_parse_factor(input)?;

            ll_parse_multerm(val.unwrap(), input)
        }
        _ => Err(MexeError::UnexpectedToken), // TODO: must include token
    }
}

fn ll_parse_multerm(val: f64, input: &[Token]) -> Result<(Option<f64>, &[Token])> {
    match &input[0] {
        t @ (Token::Op(Operator::Mul) | Token::Op(Operator::Div)) => {
            let (val2, input) = ll_parse_factor(&input[1..])?;

            let val = match t {
                Token::Op(Operator::Mul) => val * val2.unwrap(),
                Token::Op(Operator::Div) => val / val2.unwrap(),
                _ => unreachable!(),
            };

            ll_parse_multerm(val, input)
        }
        Token::RPar => Ok((Some(val), input)),
        Token::Op(_) | Token::EOI => Ok((Some(val), input)),
        _ => Err(MexeError::UnexpectedToken),
    }
}

fn ll_parse_factor(input: &[Token]) -> Result<(Option<f64>, &[Token])> {
    match (&input[0], input.get(1)) {
        (Token::Op(Operator::Minus), Some(Token::LPar)) => match ll_parse_expr(&input[2..]) {
            Ok((Some(val), input)) => Ok((Some(-val), input)),
            err => err,
        },
        (Token::Op(Operator::Minus), Some(Token::Number(n))) => Ok((Some(-*n), &input[2..])),
        (Token::LPar, _) => match ll_parse_expr(&input[1..]) {
            Ok((Some(val), input)) => Ok((Some(val), &input[1..])),
            err => err,
        },
        (Token::Number(n), _) => Ok((Some(*n), &input[1..])),
        _ => Err(MexeError::UnexpectedToken),
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
    fn get_tokens_does_not_panic_with_good_input() {
        let exprs = [
            "1+1",
            "1.1+1",
            "1.1+1.2",
            "183.387+(2*2.3)",
            "(2.3 + 1) - ((2.55 - 91381.832) / (83767.3 * 22))",
        ];

        for expr in exprs.iter() {
            let tokens = get_tokens(expr);
        }
    }

    #[test]
    fn get_tokens_does_not_panic_with_bad_input() {
        let exprs = ["1+1+", "1.1.1+1", "1.1+1.", "183.+(2*2.3)", "(2.3 ++ 1)"];

        for expr in exprs.iter() {
            let tokens = get_tokens(expr);
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

        float_eq!(21.0, eval("(1 + (4 * 5))").unwrap());
        float_eq!(10.5, eval("(1 + (4 * 5)) / 2").unwrap());
        float_eq!(18.0, eval("1 + (4 * 5) - 9 / 3").unwrap());
        float_eq!(8.4, eval("(1 + (4 * 5)) / 2 - 3 * 0.7").unwrap());
        float_eq!(9.9, eval("(1 + ((4 * 5) + (3))) / 2 - 3 * 0.7").unwrap());
        float_eq!(0.45, eval("0.15 + 0.15 + 0.15").unwrap());
    }

    #[test]
    fn test_eval_binary() {
        float_eq!(3.0, eval_binary("1 + 2").unwrap());
        float_eq!(10.0, eval_binary("2*5").unwrap());
        float_eq!(1.1, eval_binary("5.5/5").unwrap());
        float_eq!(10.5, eval_binary(" 5.5  + 5 ").unwrap());
    }
}
