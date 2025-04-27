/// This contains enum for list of Tokens, and handles Operator precedence rules.

// List of valid tokens that can be constructed from arithmetic expression by Tokenizer

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    And,        // &
    Or,         // |
    Add,        // +
    Subtract,   // -
    Multiply,   // *
    Divide,     // /
    Caret,      // ^
    LeftParen,  // (
    RightParen, // )
    Num(f64),   // 12.34
    EOF,
}

// Order of operators as per operator precedence rules (low to high)

#[derive(Debug, PartialEq, PartialOrd)]
/// Defines all the OperPrec levels, from lowest to highest.
pub enum OperPrec {
    DefaultZero,
    AndOr,
    AddSub,
    MulDiv,
    Power,
    Negative,
}

// This contains methods to retrieve operator precedence for a given arithmetic operator

impl Token {
    pub fn get_oper_prec(&self) -> OperPrec {
        use self::OperPrec::*;
        use self::Token::*;
        match *self {
            And | Or => AndOr,
            Add | Subtract => AddSub,
            Multiply | Divide => MulDiv,
            Caret => Power,

            _ => DefaultZero,
        }
    }
}
