use expression_eval::parsemath::ast;
use expression_eval::parsemath::parser::Parser;

#[test]
fn test_expr1() {
    let mut parser = Parser::new("(3.5 + 4.5) * 2 ^ ((7 & 3) | 1)").unwrap();
    let ast = parser.parse().unwrap();
    assert_eq!(ast::eval(ast).unwrap(), 64.0);
}

#[test]
fn test_expr2() {
    let mut parser = Parser::new("(-2 + 3.5) * ((4 ^ 2) - (5.0 / (1 + 1)))").unwrap();
    let ast = parser.parse().unwrap();
    assert_eq!(ast::eval(ast).unwrap(), 20.25);
}

#[test]
fn test_expr3() {
    let mut parser = Parser::new("(((10 / 2) + 3.0) ^ 2) & (15 ^ (8 | 2))").unwrap();
    let ast = parser.parse().unwrap();
    assert_eq!(ast::eval(ast).unwrap(), 64.0);
}


#[test]
fn test_expr4() {
    let mut parser = Parser::new("((((5 + 3.0) ^ 2) - 1) / (4 - 1)) + ((12 & 7) | (3 ^ 1))").unwrap();
    let ast = parser.parse().unwrap();
    assert_eq!(ast::eval(ast).unwrap(), 28.0);
}

#[test]
fn test_expr5() {
    let mut parser = Parser::new("(8.5 & 3) + 2").unwrap();
    let ast = parser.parse().unwrap();
    assert!(ast::eval(ast).is_err());
}

#[test]
fn test_expr6() {
    let mut parser = Parser::new("(4 + 1) * (2 / (3 - 3))").unwrap();
    let ast = parser.parse().unwrap();
    assert!(ast::eval(ast).is_err());
}

#[test]
fn test_expr7() {
    let mut parser = Parser::new("((4 + 1) * ((2 / (3 - 3))").unwrap();
    assert!(parser.parse().is_err());
}

#[test]
fn test_expr8() {
    let mut parser = Parser::new("(6 + 8) $% 45 #* 90").unwrap();
    assert!(parser.parse().is_err());
}