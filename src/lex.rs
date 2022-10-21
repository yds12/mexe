use crate::{MexeError, Operator, Result, Token};

enum LexerState {
    Normal,
    ReadingUTF8MD,
    ReadingNumber(usize),
    ReadingDecimals(usize),
}

pub(crate) fn get_tokens(expression: &str) -> Result<Vec<Token>> {
    let chars = expression.as_bytes();
    let mut tokens = Vec::with_capacity(chars.len() / 2 + 2); // heuristic
    let mut state = LexerState::Normal;

    for i in 0..chars.len() {
        let (in_number, token) = match chars[i] {
            b' ' => (false, None),
            b'(' => (false, Some(Token::LPar)),
            b')' => (false, Some(Token::RPar)),
            b'*' => (false, Some(Token::Op(Operator::Mul))),
            b'+' => (false, Some(Token::Op(Operator::Add))),
            b'-' => (false, Some(Token::Op(Operator::Sub))),
            b'/' => (false, Some(Token::Op(Operator::Div))),

            b'0'..=b'9' => {
                state = match state {
                    LexerState::Normal => LexerState::ReadingNumber(i),
                    LexerState::ReadingUTF8MD |
                    LexerState::ReadingNumber(_) | LexerState::ReadingDecimals(_) => state,
                };

                (true, None)
            }

            b'.' => {
                state = match state {
                    LexerState::ReadingUTF8MD |
                    LexerState::Normal | LexerState::ReadingDecimals(_) => {
                        return Err(MexeError::UnexpectedCharacter(b'.', i))
                    }
                    LexerState::ReadingNumber(n) => LexerState::ReadingDecimals(n),
                };

                (true, None)
            }

            195/* 0xc3*/ => {
                //eprintln!("{:?}, {:?}", chars, state);
                state = LexerState::ReadingUTF8MD;
                (false, None)
            }

            151 | 183/* 0x97 | 0xb7*/ if matches!(state, LexerState::ReadingUTF8MD) => {
                state = LexerState::Normal;    // '×', '÷'
                (false, Some(Token::Op(if chars[i] == 151 {
                    Operator::Mul } else { Operator::Div })))
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

            if matches!(state, LexerState::ReadingUTF8MD) { continue }
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
            "1÷1×1",
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
