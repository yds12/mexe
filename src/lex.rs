use crate::{consts::*, MexeError, Operator, Result, Token};

enum LexerState {
    Normal,
    ReadingNumber(usize),
    ReadingDecimals(usize),
}

pub(crate) fn get_tokens(expression: &str) -> Result<Vec<Token>> {
    let chars = expression.as_bytes();
    let mut tokens = Vec::with_capacity(chars.len() / 2 + 1);
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
            N0 | N1 | N2 | N3 | N4 | N5 | N6 | N7 | N8 | N9 => {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn does_not_panic_with_good_input() {
        let exprs = [
            "1+1",
            "1.1+1",
            "1.1+1.2",
            "183.387+(2*2.3)",
            "(2.3 + 1) - ((2.55 - 91381.832) / (83767.3 * 22))",
        ];

        for expr in exprs.iter() {
            let _tokens = get_tokens(expr);
        }
    }

    #[test]
    fn does_not_panic_with_bad_input() {
        let exprs = [
            "",
            "1+1+",
            "1.1.1+1",
            "1.1+1.",
            "183.+(2*2.3)",
            "(2.3 ++ 1)",
        ];

        for expr in exprs.iter() {
            let _tokens = get_tokens(expr);
        }
    }
}
