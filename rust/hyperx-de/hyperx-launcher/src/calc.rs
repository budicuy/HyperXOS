/// Simple, safe recursive descent math evaluator for Spotlight quick calculation
pub fn evaluate_math(input: &str) -> Option<f64> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return None;
    }

    // Only attempt math evaluation if there are operators or digits
    let has_digit = trimmed.chars().any(|c| c.is_ascii_digit());
    let has_math_char = trimmed.chars().any(|c| matches!(c, '+' | '-' | '*' | '/' | '^' | '%'));

    if !has_digit || !has_math_char {
        return None;
    }

    let tokens = tokenize(trimmed)?;
    let mut parser = Parser { tokens, pos: 0 };
    let res = parser.parse_expr()?;
    if parser.pos == parser.tokens.len() {
        Some(res)
    } else {
        None
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Token {
    Number(f64),
    Plus,
    Minus,
    Mul,
    Div,
    Mod,
    Pow,
    LParen,
    RParen,
}

fn tokenize(s: &str) -> Option<Vec<Token>> {
    let mut tokens = Vec::new();
    let mut chars = s.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            ' ' | '\t' => {
                chars.next();
            }
            '+' => {
                tokens.push(Token::Plus);
                chars.next();
            }
            '-' => {
                tokens.push(Token::Minus);
                chars.next();
            }
            '*' => {
                tokens.push(Token::Mul);
                chars.next();
            }
            '/' => {
                tokens.push(Token::Div);
                chars.next();
            }
            '%' => {
                tokens.push(Token::Mod);
                chars.next();
            }
            '^' => {
                tokens.push(Token::Pow);
                chars.next();
            }
            '(' => {
                tokens.push(Token::LParen);
                chars.next();
            }
            ')' => {
                tokens.push(Token::RParen);
                chars.next();
            }
            '0'..='9' | '.' => {
                let mut num_str = String::new();
                while let Some(&nc) = chars.peek() {
                    if nc.is_ascii_digit() || nc == '.' {
                        num_str.push(nc);
                        chars.next();
                    } else {
                        break;
                    }
                }
                let val: f64 = num_str.parse().ok()?;
                tokens.push(Token::Number(val));
            }
            _ => return None,
        }
    }

    Some(tokens)
}

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn next_token(&mut self) -> Option<Token> {
        if self.pos < self.tokens.len() {
            let t = self.tokens[self.pos].clone();
            self.pos += 1;
            Some(t)
        } else {
            None
        }
    }

    fn parse_expr(&mut self) -> Option<f64> {
        let mut left = self.parse_term()?;

        while let Some(tok) = self.peek() {
            match tok {
                Token::Plus => {
                    self.next_token();
                    let right = self.parse_term()?;
                    left += right;
                }
                Token::Minus => {
                    self.next_token();
                    let right = self.parse_term()?;
                    left -= right;
                }
                _ => break,
            }
        }

        Some(left)
    }

    fn parse_term(&mut self) -> Option<f64> {
        let mut left = self.parse_factor()?;

        while let Some(tok) = self.peek() {
            match tok {
                Token::Mul => {
                    self.next_token();
                    let right = self.parse_factor()?;
                    left *= right;
                }
                Token::Div => {
                    self.next_token();
                    let right = self.parse_factor()?;
                    if right == 0.0 {
                        return None;
                    }
                    left /= right;
                }
                Token::Mod => {
                    self.next_token();
                    let right = self.parse_factor()?;
                    if right == 0.0 {
                        return None;
                    }
                    left %= right;
                }
                _ => break,
            }
        }

        Some(left)
    }

    fn parse_factor(&mut self) -> Option<f64> {
        let mut base = self.parse_primary()?;

        if let Some(Token::Pow) = self.peek() {
            self.next_token();
            let exponent = self.parse_factor()?;
            base = base.powf(exponent);
        }

        Some(base)
    }

    fn parse_primary(&mut self) -> Option<f64> {
        match self.next_token()? {
            Token::Number(n) => Some(n),
            Token::Plus => self.parse_primary(),
            Token::Minus => Some(-self.parse_primary()?),
            Token::LParen => {
                let expr = self.parse_expr()?;
                if let Some(Token::RParen) = self.next_token() {
                    Some(expr)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spotlight_eval() {
        assert_eq!(evaluate_math("2 + 2"), Some(4.0));
        assert_eq!(evaluate_math("10 * 5 + 2"), Some(52.0));
        assert_eq!(evaluate_math("100 / 4"), Some(25.0));
        assert_eq!(evaluate_math("2 ^ 8"), Some(256.0));
        assert_eq!(evaluate_math("(10 + 5) * 2"), Some(30.0));
        assert_eq!(evaluate_math("firefox"), None);
        assert_eq!(evaluate_math(""), None);
    }
}
