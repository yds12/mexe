use crate::{MexeError, Operator, Result, Token};

pub(crate) fn parse_and_evaluate(input: Vec<Token>) -> Result<f64> {
    match ll_parse_expr(&input[..]) {
        Ok((Some(val), input)) => {
            if !is_over(input) {
                // finished parsing but there's something left
                Err(MexeError::UnexpectedToken(input[0].to_string()))
            } else {
                Ok(val)
            }
        }
        // if value is `None` the parse should have failed earlier (should never happen)
        Ok((None, _)) => Err(MexeError::InternalParserError),
        Err(err) => Err(err),
    }
}

fn is_over(input: &[Token]) -> bool {
    input.len() == 1 && input[0] == Token::EOI
}

// E  -> T E'
fn ll_parse_expr(input: &[Token]) -> Result<(Option<f64>, &[Token])> {
    match input[0] {
        Token::LPar | Token::Number(_) | Token::Op(Operator::Minus) => {
            let (val, input) = ll_parse_term(input)?;
            ll_parse_addexpr(val.unwrap(), input)
        }
        token => Err(MexeError::UnexpectedToken(token.to_string())),
    }
}

// E' -> + T E'
// E' -> - T E'
// E' -> ε
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
        _ => Ok((Some(val), input)),
    }
}

// T  -> F T'
fn ll_parse_term(input: &[Token]) -> Result<(Option<f64>, &[Token])> {
    match input[0] {
        Token::LPar | Token::Number(_) | Token::Op(Operator::Minus) => {
            let (val, input) = ll_parse_factor(input)?;

            ll_parse_multerm(val.unwrap(), input)
        }
        token => Err(MexeError::UnexpectedToken(token.to_string())),
    }
}

// T' -> * F T'
// T' -> / F T'
// T' -> ε
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
        _ => Ok((Some(val), input)),
    }
}

// F  -> ( E )
// F  -> n
// F  -> - ( E )
// F  -> - n
fn ll_parse_factor(input: &[Token]) -> Result<(Option<f64>, &[Token])> {
    match (&input[0], input.get(1)) {
        (Token::Op(Operator::Minus), Some(Token::LPar)) => match ll_parse_expr(&input[2..]) {
            Ok((Some(val), input)) => ll_consume_rpar(-val, input),
            err => err,
        },
        (Token::Op(Operator::Minus), Some(Token::Number(n))) => Ok((Some(-*n), &input[2..])),
        (Token::LPar, _) => match ll_parse_expr(&input[1..]) {
            Ok((Some(val), input)) => ll_consume_rpar(val, input),
            err => err,
        },
        (Token::Number(n), _) => Ok((Some(*n), &input[1..])),
        (token, _) => Err(MexeError::UnexpectedToken(token.to_string())),
    }
}

fn ll_consume_rpar(val: f64, input: &[Token]) -> Result<(Option<f64>, &[Token])> {
    match input.get(0) {
        Some(Token::RPar) => Ok((Some(val), &input[1..])),
        None => Err(MexeError::UnexpectEndOfInput),
        Some(t) => Err(MexeError::UnexpectedToken(t.to_string())),
    }
}
