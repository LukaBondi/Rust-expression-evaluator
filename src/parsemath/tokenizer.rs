/// This module reads characters in arithmetic expression and converts them to tokens.
/// The allowed tokens are defined in ast module.
// Standard lib
use std::iter::Peekable;
use std::str::Chars;

//Other internal modules
use super::token::Token;

// Other structs

// Tokenizer struct contains a Peekable iterator on the arithmetic expression
pub struct Tokenizer<'a> {
    expr: Peekable<Chars<'a>>,
}

// Constructs a new instance of Tokenizer
impl<'a> Tokenizer<'a> {
    pub fn new(new_expr: &'a str) -> Self {
        Tokenizer {
            expr: new_expr.chars().peekable(),
        }
    }
}

// Implement Iterator trait for Tokenizer struct.
// With this, we can use next() method on tokenizer to retrieve the next token from arithmetic expression
impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        while let Some(&c) = self.expr.peek() {
            if c.is_ascii_whitespace() {
                self.expr.next();
            } else {
                break;
            }
        }
        
        let next_char = self.expr.next();
        match next_char {
            Some('0'..='9') => {
                let mut buffer = String::from(next_char.unwrap());
                let mut has_decimal = false;
                while let Some(&c) = self.expr.peek() {
                    match c {
                        '0'..='9'=> {
                            buffer.push(self.expr.next().unwrap());
                        },
                        '.' => {
                            if has_decimal { 
                                return None;
                            } else {
                                has_decimal = true;
                                buffer.push(self.expr.next().unwrap());
                            }
                        },
                        c if c.is_ascii_whitespace() => { self.expr.next(); },
                        _ => break,
                     }
                }
                buffer.parse::<f64>().map(Token::Num).ok()
            },
            Some('&') => Some(Token::And),
            Some('|') => Some(Token::Or),
            Some('+') => Some(Token::Add),
            Some('-') => Some(Token::Subtract),
            Some('*') => Some(Token::Multiply),
            Some('/') => Some(Token::Divide),
            Some('^') => Some(Token::Caret),
            Some('(') => Some(Token::LeftParen),
            Some(')') => Some(Token::RightParen),
            None => Some(Token::EOF),
            Some(_) => None,
        }
    }
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_positive_integer() {
        let mut tokenizer = Tokenizer::new("34");
        assert_eq!(tokenizer.next().unwrap(), Token::Num(34.0))
    }
    #[test]
    fn test_decimal_number() {
        let mut tokenizer = Tokenizer::new("34.5");
        assert_eq!(tokenizer.next().unwrap(), Token::Num(34.5))
    }
    #[test]
    fn test_invalid_char() {
        let mut tokenizer = Tokenizer::new("#$%");
        assert_eq!(tokenizer.next(), None);
    }

    /* ~~~~~~~~~~~~ Additional test cases ~~~~~~~~~~~~ */
    #[test]
    fn test_incorrect_decimal() {
        let mut tokenizer = Tokenizer::new("12.3.4");
        assert_eq!(tokenizer.next(), None);
    }

    #[test]
    fn test_space_in_number() {
        let mut tokenizer = Tokenizer::new("1 2 . 3 4");
        assert_eq!(tokenizer.next().unwrap(), Token::Num(12.34))
    }

    #[test]
    fn test_other_whitespaces() {
        let mut tokenizer = Tokenizer::new("1\t.\n2\r\n3");
        assert_eq!(tokenizer.next().unwrap(), Token::Num(1.23))
    }

    #[test]
    fn test_end_of_file() {
        let mut tokenizer = Tokenizer::new("");
        assert_eq!(tokenizer.next().unwrap(), Token::EOF)
    }

    #[test]
    fn test_leading_whitespace() {
        let mut tokenizer = Tokenizer::new("  \n\r\t  34.56");
        assert_eq!(tokenizer.next().unwrap(), Token::Num(34.56))
    }

    #[test]
    fn test_other_sign() {
        let mut tokenizer = Tokenizer::new("&|+-*/^()");
        assert_eq!(tokenizer.next().unwrap(), Token::And);
        assert_eq!(tokenizer.next().unwrap(), Token::Or);
        assert_eq!(tokenizer.next().unwrap(), Token::Add);
        assert_eq!(tokenizer.next().unwrap(), Token::Subtract);
        assert_eq!(tokenizer.next().unwrap(), Token::Multiply);
        assert_eq!(tokenizer.next().unwrap(), Token::Divide);
        assert_eq!(tokenizer.next().unwrap(), Token::Caret);
        assert_eq!(tokenizer.next().unwrap(), Token::LeftParen);
        assert_eq!(tokenizer.next().unwrap(), Token::RightParen);
    }

}
