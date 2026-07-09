#[derive(Debug)]
enum Token {
    Num(i32),
    Add,
    Sub,
    Mul,
    Div,
    Pow(u32),
}

fn next_token<'a, I>(tokens: &mut I) -> Option<Token>
where
    I: Iterator<Item = &'a str>,
{
    let t = tokens.next()?;

    match t {
        "plus" => Some(Token::Add),
        "minus" => Some(Token::Sub),
        "multiplied" => {
            tokens.next()?;
            Some(Token::Mul)
        }
        "divided" => {
            tokens.next()?;
            Some(Token::Div)
        }
        "raised" => {
            tokens.next()?;
            tokens.next()?;

            let pow = tokens.next()?;
            let pow = &pow[..pow.len().saturating_sub(2)];
            let rhs: u32 = pow.parse().ok()?;

            tokens.next()?;

            Some(Token::Pow(rhs))
        }
        _ => {
            let mut t = t;

            if t.ends_with('?') {
                t = &t[..t.len() - 1];
            }
            t.parse::<i32>().ok().map(Token::Num)
        }
    }
}

pub fn answer(command: &str) -> Option<i32> {
    let mut tokens = command.split_ascii_whitespace();

    tokens.next();
    tokens.next();

    let mut res = match next_token(&mut tokens)? {
        Token::Num(v) => v,
        _ => return None,
    };

    while let Some(token) = next_token(&mut tokens) {
        match token {
            Token::Add => {
                if let Token::Num(rhs) = next_token(&mut tokens)? {
                    res += rhs;
                } else {
                    return None;
                }
            }
            Token::Sub => {
                if let Token::Num(rhs) = next_token(&mut tokens)? {
                    res -= rhs;
                } else {
                    return None;
                }
            }
            Token::Mul => {
                if let Token::Num(rhs) = next_token(&mut tokens)? {
                    res *= rhs;
                } else {
                    return None;
                }
            }
            Token::Div => {
                if let Token::Num(rhs) = next_token(&mut tokens)? {
                    if rhs == 0 {
                        return None;
                    }
                    res /= rhs;
                } else {
                    return None;
                }
            }
            Token::Pow(rhs) => {
                res = res.pow(rhs);
            }

            Token::Num(_) => return None,
        }
    }

    Some(res)
}

fn main() {
    let input = "What is -3 plus 7 multiplied by -2?";
    dbg!(answer(input));
}

