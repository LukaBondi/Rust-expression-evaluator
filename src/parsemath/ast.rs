/// This program contains list of valid AST nodes that can be constructed and also evaluates an AST to compute a value
// Standard lib
use std::error;

//structs

// List of allowed AST nodes that can be constructed by Parser
// Tokens can be arithmetic operators or a Number
#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    // WARNING: Bitwise And and Or operation only works on integer value
    And(Box<Node>, Box<Node>),
    Or(Box<Node>, Box<Node>),

    Add(Box<Node>, Box<Node>),
    Subtract(Box<Node>, Box<Node>),
    Multiply(Box<Node>, Box<Node>),
    Divide(Box<Node>, Box<Node>),
    Caret(Box<Node>, Box<Node>),
    Negative(Box<Node>),
    Number(f64),
}

// Given an AST, calculate the numeric value.
pub fn eval(expr: Node) -> Result<f64, Box<dyn error::Error>> {
    use self::Node::*;
    match expr {
        Number(i) => Ok(i),
        Negative(expr) => Ok(-eval(*expr)?),
        Add(expr1, expr2) => Ok(eval(*expr1)? + eval(*expr2)?),
        Subtract(expr1, expr2) => Ok(eval(*expr1)? - eval(*expr2)?),
        Multiply(expr1, expr2) => Ok(eval(*expr1)? * eval(*expr2)?),
        Divide(expr1, expr2) => {
            let denom = eval(*expr2)?;
            if denom.abs() < f64::EPSILON {
                Err(("Division by zero").into())
            } else {
                Ok(eval(*expr1)? / denom)
            }
        },
        Caret(expr1, expr2) => {
            let base_exp = eval(*expr1)?;
            let pow_exp = eval(*expr2)?;

            if base_exp == 0.0 && pow_exp < 0.0 {
                return Err("0^negative is undefined".into());
            } 
            
            if base_exp < 0.0 && (pow_exp.fract().abs() > f64::EPSILON) {
                return Err("Negative base with fractional exponent".into());
            }  

            let res = base_exp.powf(pow_exp);
            if res.is_infinite() {
                return Err("Overflow in exponentiation".into());
            }

            Ok(res)
        },
        And(expr1, expr2) => {
            let left_exp = eval(*expr1)?;
            let right_exp = eval(*expr2)?;
            if left_exp.fract() == 0.0 && right_exp.fract() == 0.0 
                && left_exp >= (i64::MIN as f64)
                && right_exp >= (i64::MIN as f64)
                && left_exp <= (i64::MAX as f64) 
                && right_exp <= (i64::MAX as f64) 
            {
                Ok((left_exp as i64 & right_exp as i64) as f64)
            }
            else {
                Err("Cannot perform bitwise AND with the following expressions".into())
            }
        },
        Or(expr1, expr2) => {
            let left_exp = eval(*expr1)?;
            let right_exp = eval(*expr2)?;
            if left_exp.fract() == 0.0 && right_exp.fract() == 0.0 
                && left_exp >= (i64::MIN as f64)
                && right_exp >= (i64::MIN as f64)
                && left_exp <= (i64::MAX as f64) 
                && right_exp <= (i64::MAX as f64) 
            {
                Ok((left_exp as i64 | right_exp as i64) as f64)
            } else {
                Err("Cannot perform bitwise OR with the following expressions".into())
            }
        },
    }
}

//Unit tests
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_expr1() {
        use crate::parsemath::parser::Parser;

        let ast = Parser::new("1+2-3").unwrap().parse().unwrap();
        let value = eval(ast).unwrap();
        assert_eq!(value, 0.0);
    }
    #[test]
    fn test_expr2() {
        use crate::parsemath::parser::Parser;

        let ast = Parser::new("3+2-1*5/4").unwrap().parse().unwrap();
        let value = eval(ast).unwrap();
        assert_eq!(value, 3.75);
    }
    #[test]
    fn test_expr3() {
        use crate::parsemath::parser::Parser;

        let ast = Parser::new("3+3 | 4").unwrap().parse().unwrap();
        let value = eval(ast).unwrap();
        assert_eq!(value, 6.0);
    }

    /* ~~~~~~~~~~~~ Additional test cases ~~~~~~~~~~~~ */
    #[test]
    fn test_negative() {
        use crate::parsemath::parser::Parser;

        let mut ast = Parser::new("-5").unwrap().parse().unwrap();
        let mut value = eval(ast).unwrap();
        assert_eq!(value, -5.0);

        ast = Parser::new("--3").unwrap().parse().unwrap();
        value = eval(ast).unwrap();
        assert_eq!(value, 3.0);
    }

    #[test]
    fn test_addition() {
        use crate::parsemath::parser::Parser;

        // Addition 1
        let mut ast = Parser::new("3+4").unwrap().parse().unwrap();
        let mut value = eval(ast).unwrap();
        assert_eq!(value, 7.0);

        // Addition 2
        ast = Parser::new("-5+6").unwrap().parse().unwrap();
        value = eval(ast).unwrap();
        assert_eq!(value, 1.0);

        // Addition 3
        ast = Parser::new("-8+-1").unwrap().parse().unwrap();
        value = eval(ast).unwrap();
        assert_eq!(value, -9.0);
    }

    #[test]
    fn test_subtraction() {
        use crate::parsemath::parser::Parser;

        // Subtraction 1
        let mut ast = Parser::new("4-3").unwrap().parse().unwrap();
        let mut value = eval(ast).unwrap();
        assert_eq!(value, 1.0);

        // Subtraction 2
        ast = Parser::new("3-6").unwrap().parse().unwrap();
        value = eval(ast).unwrap();
        assert_eq!(value, -3.0);

        // Subtraction 3
        ast = Parser::new("-2-7").unwrap().parse().unwrap();
        value = eval(ast).unwrap();
        assert_eq!(value, -9.0);

        // Subtraction 4
        ast = Parser::new("-5--10").unwrap().parse().unwrap();
        value = eval(ast).unwrap();
        assert_eq!(value, 5.0);
    }

    #[test]
    fn test_multiplication() {
        use crate::parsemath::parser::Parser;

        // Subtraction 1
        let mut ast = Parser::new("4*3").unwrap().parse().unwrap();
        let mut value = eval(ast).unwrap();
        assert_eq!(value, 12.0);

        // Subtraction 2
        ast = Parser::new("-5*3").unwrap().parse().unwrap();
        value = eval(ast).unwrap();
        assert_eq!(value, -15.0);

        // Subtraction 3
        ast = Parser::new("-6*-4").unwrap().parse().unwrap();
        value = eval(ast).unwrap();
        assert_eq!(value, 24.0);

        // Subtraction 4
        ast = Parser::new("3*-7").unwrap().parse().unwrap();
        value = eval(ast).unwrap();
        assert_eq!(value, -21.0);
    }

    #[test]
    fn test_division() {
        use crate::parsemath::parser::Parser;

        // Division 1
        let mut ast = Parser::new("6/2").unwrap().parse().unwrap();
        let mut value = eval(ast).unwrap();
        assert_eq!(value, 3.0);

        // Division 2
        ast = Parser::new("-9/2").unwrap().parse().unwrap();
        value = eval(ast).unwrap();
        assert_eq!(value, -4.5);

        // Division 3
        ast = Parser::new("10/-2").unwrap().parse().unwrap();
        value = eval(ast).unwrap();
        assert_eq!(value, -5.0);

        // Division 4
        ast = Parser::new("-25/-4").unwrap().parse().unwrap();
        value = eval(ast).unwrap();
        assert_eq!(value, 6.25);

        ast = Parser::new("1/0").unwrap().parse().unwrap();
        let error = eval(ast);
        assert!(error.is_err());
    }

    #[test]
    fn test_caret() {
        use crate::parsemath::parser::Parser;

        // Exponent 1
        let mut ast = Parser::new("2^3").unwrap().parse().unwrap();
        let mut value = eval(ast).unwrap();
        assert_eq!(value, 8.0);
    
        // Exponent 2
        ast = Parser::new("4^0.5").unwrap().parse().unwrap();
        value = eval(ast).unwrap();
        assert_eq!(value, 2.0);
        
        // Exponent 3
        ast = Parser::new("-2^3").unwrap().parse().unwrap();
        value = eval(ast).unwrap();
        assert_eq!(value, -8.0);

        // Exponent 4
        ast = Parser::new("0^-1").unwrap().parse().unwrap();
        let mut error = eval(ast);
        assert!(error.is_err());

        // Exponent 5
        ast = Parser::new("-4^0.5").unwrap().parse().unwrap();
        error = eval(ast);
        assert!(error.is_err());

        // Exponent 6
        ast = Parser::new("100000000^100").unwrap().parse().unwrap();
        error = eval(ast);
        assert!(error.is_err());
        
    }

    #[test]
    fn test_and() {
        use crate::parsemath::parser::Parser;

        // Bitwise And 1
        let mut ast = Parser::new("6&2").unwrap().parse().unwrap();
        let value = eval(ast).unwrap();
        assert_eq!(value, 2.0);
        
        // Bitwise And 2
        ast = Parser::new("6.5&2").unwrap().parse().unwrap();
        let error = eval(ast);
        assert!(error.is_err());
    }

    #[test]
    fn test_or() {
        use crate::parsemath::parser::Parser;

        // Bitwise Or 1
        let mut ast = Parser::new("6|2").unwrap().parse().unwrap();
        let mut value = eval(ast).unwrap();
        assert_eq!(value, 6.0);
        
        // Bitwise Or 2
        ast = Parser::new("-1|2").unwrap().parse().unwrap();
        value = eval(ast).unwrap();
        assert_eq!(value, -1.0);

        // Bitwise Or 2
        ast = Parser::new("6.5&2").unwrap().parse().unwrap();
        let error = eval(ast);
        assert!(error.is_err());
    }
}
