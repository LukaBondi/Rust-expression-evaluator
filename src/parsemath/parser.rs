/// This program reads tokens returned by Tokenizer and converts them into AST.
// Standard lib
use std::fmt;

// Internal modules
use super::ast::Node;
use super::token::{OperPrec, Token};
use super::tokenizer::Tokenizer;

//Structs and constants

// Parser struct
pub struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
    current_token: Token,
}

// Public methods of Parser

impl<'a> Parser<'a> {
    // Create a new instance of Parser
    pub fn new(expr: &'a str) -> Result<Self, ParseError> {
        let mut lexer = Tokenizer::new(expr);
        let cur_token = match lexer.next() {
            Some(token) => token,
            None => return Err(ParseError::InvalidOperator("Invalid character".into())),
        };
        Ok(Parser {
            tokenizer: lexer,
            current_token: cur_token,
        })
    }

    // Take an arithmetic expression as input and return an AST
    pub fn parse(&mut self) -> Result<Node, ParseError> {
        let ast = self.generate_ast(OperPrec::DefaultZero);
        match ast {
            Ok(res) => Ok(res),
            Err(error) => Err(error),
        }
    }
}

// Private methods of Parser

impl<'a> Parser<'a> {
    // Retrieve the next token from arithmetic expression and set it to current_token field in Parser struct
    fn get_next_token(&mut self) -> Result<(), ParseError> {
        let next_token = match self.tokenizer.next() {
            Some(token) => token,
            None => return Err(ParseError::InvalidOperator("Invalid character".into())),
        };
        self.current_token = next_token;
        Ok(())
    }

    // Main workhorse method that is called recursively
    fn generate_ast(&mut self, oper_prec: OperPrec) -> Result<Node, ParseError> {
        let mut left_expr = self.parse_number()?;

        while oper_prec < self.current_token.get_oper_prec() {
            if self.current_token == Token::EOF {
                break;
            }
            let right_expr = self.convert_token_to_node(left_expr.clone())?;
            left_expr = right_expr;
        }
        Ok(left_expr)
    }

    // Construct AST node for numbers, taking into account negative prefixes while handling parenthesis
    fn parse_number(&mut self) -> Result<Node, ParseError> {
        let token = self.current_token.clone();
        match token {
            Token::Subtract => {
                self.get_next_token()?;
                let expr = self.generate_ast(OperPrec::Negative)?;
                Ok(Node::Negative(Box::new(expr)))
            }
            Token::Num(i) => {
                self.get_next_token()?;
                if self.current_token == Token::LeftParen {
                    let right_expr = self.parse_number()?;
                    return Ok(Node::Multiply(Box::new(Node::Number(i)), Box::new(right_expr)));
                }
                Ok(Node::Number(i))
            }
            Token::LeftParen => {
                self.get_next_token()?;
      
                let expr = self.generate_ast(OperPrec::DefaultZero)?;
                if let Err(e) = self.check_paren(Token::RightParen) {
                    return Err(e)
                }

                // Check if there is another follow-up expression for multiplication
                if self.current_token == Token::Subtract {
                    return Ok(expr);
                }
                match self.parse_number() {
                    Ok(right_expr) => Ok(Node::Multiply(Box::new(expr), Box::new(right_expr))),
                    Err(_) => Ok(expr),
                }
            }
            _ => Err(ParseError::UnableToParse("Unable to parse".to_string())),
        }
    }

    // Check for balancing parenthesis
    fn check_paren(&mut self, expected: Token) -> Result<(), ParseError> {
        if expected == self.current_token {
            self.get_next_token()?;
            Ok(())
        } else {
            Err(ParseError::InvalidOperator(format!(
                "Expected {:?}, got {:?}",
                expected, self.current_token
            )))
        }
    }

    // Construct Operator AST nodes
    fn convert_token_to_node(&mut self, left_expr: Node) -> Result<Node, ParseError> {
        match self.current_token {
            Token::Add => {
                self.get_next_token()?; // Get right-side expression
                let right_expr = self.generate_ast(OperPrec::AddSub)?;
                Ok(Node::Add(Box::new(left_expr), Box::new(right_expr)))
            },
            Token::Subtract => {
                self.get_next_token()?; // Get right-side expression
                let right_expr = self.generate_ast(OperPrec::AddSub)?;
                Ok(Node::Subtract(Box::new(left_expr), Box::new(right_expr)))
            },
            Token::Multiply => {
                self.get_next_token()?; // Get right-side expression
                let right_expr = self.generate_ast(OperPrec::MulDiv)?;
                Ok(Node::Multiply(Box::new(left_expr), Box::new(right_expr)))
            },
            Token::Divide => {
                self.get_next_token()?; // Get right-side expression
                let right_expr = self.generate_ast(OperPrec::MulDiv)?;
                Ok(Node::Divide(Box::new(left_expr), Box::new(right_expr)))
            },
            Token::Caret => {
                self.get_next_token()?; // Get right-side expression
                let right_expr = self.generate_ast(OperPrec::Power)?;
                Ok(Node::Caret(Box::new(left_expr), Box::new(right_expr)))
            },
            Token::And => {
                self.get_next_token()?; // Get right-side expression
                let right_expr = self.generate_ast(OperPrec::AndOr)?;
                Ok(Node::And(Box::new(left_expr), Box::new(right_expr)))
            }
            Token::Or => {
                self.get_next_token()?; // Get right-side expression
                let right_expr = self.generate_ast(OperPrec::AndOr)?;
                Ok(Node::Or(Box::new(left_expr), Box::new(right_expr)))
            }
            _ => Err(ParseError::InvalidOperator(format!(
                "Please enter valid operator {:?}",
                self.current_token
            ))),
        }
    }
}

// Custom error handler for Parser
#[derive(Debug)]
pub enum ParseError {
    UnableToParse(String),
    InvalidOperator(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            ParseError::UnableToParse(e) => write!(f, "Error in evaluating {}", e),
            ParseError::InvalidOperator(e) => write!(f, "Error in evaluating {}", e),
        }
    }
}

// Handle error thrown from AST module

impl From<Box<dyn std::error::Error>> for ParseError {
    fn from(_evalerr: Box<dyn std::error::Error>) -> Self {
        ParseError::UnableToParse("Unable to parse".into())
    }
}

// Unit tests

#[cfg(test)]
mod tests {
    // use std::ops::Neg;

    use super::*;
    use crate::parsemath::ast::Node::*;
    #[test]
    fn test_addition() {
        let mut parser = Parser::new("1+2").unwrap();
        let expected = Add(Box::new(Number(1.0)), Box::new(Number(2.0)));
        assert_eq!(parser.parse().unwrap(), expected);
    }

    #[test]
    fn test_bitwise_or() {
        let mut parser = Parser::new("6|2").unwrap();
        let expected = Or(Box::new(Number(6.0)), Box::new(Number(2.0)));
        assert_eq!(parser.parse().unwrap(), expected);
    }

    /* ~~~~~~~~~~~~ Additional test cases ~~~~~~~~~~~~ */
    #[test]
    fn test_parenthesis_ast() {
        let mut parser = Parser::new("(1 + 2) (3 / 4)").unwrap();
        let expected = Multiply(
            Box::new(Add(
                Box::new(Number(1.0)), 
                Box::new(Number(2.0)))
            ), 
            Box::new(Divide(
                Box::new(Number(3.0)), 
                Box::new(Number(4.0)))
            )
        );
        assert_eq!(parser.parse().unwrap(), expected);
    }

    #[test]
    fn test_parenthesis_multiplication() {
        let mut parser = Parser::new("(-5)4").unwrap();
        let mut expected = Multiply(
            Box::new(Negative(Box::new(Number(5.0)))), 
            Box::new(Number(4.0))
        );
        assert_eq!(parser.parse().unwrap(), expected);

        parser = Parser::new("-5(4)").unwrap();
        expected = Negative(Box::new(Multiply(
            Box::new(Number(5.0)), 
            Box::new(Number(4.0))
        )));
        assert_eq!(parser.parse().unwrap(), expected);
    }

    #[test]
    fn test_bitwise_parsing() {
        let mut parser = Parser::new("6 & 3 | 1").unwrap();
        let expected = Or(
            Box::new(And(
                Box::new(Number(6.0)), 
                Box::new(Number(3.0))
            )),
            Box::new(Number(1.0))
        );
        assert_eq!(parser.parse().unwrap(), expected);
    }

    #[test]
    fn test_unary_negative() {
        let mut parser = Parser::new("-3 + 5").unwrap();
        let expected = Add(
            Box::new(Negative(Box::new(Number(3.0)))), 
            Box::new(Number(5.0))
        );
        assert_eq!(parser.parse().unwrap(), expected);
    }

    #[test]
    fn test_errors() {
        let mut parser = Parser::new("(1+2").unwrap();
        assert!(parser.parse().is_err());

        parser = Parser::new("8 @ 6").unwrap();
        assert!(parser.parse().is_err());
    }
}
