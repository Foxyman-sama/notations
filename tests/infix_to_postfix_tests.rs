#[path = "../src/infix_to_postfix.rs"]
mod infix_to_postfix;

use infix_to_postfix::*;

#[test]
fn when_user_forgot_about_expression() {
  let mut parser = InfixToPostfixParser::new();

  let result = parser.parse("");

  assert!(result.is_err());
}

#[test]
fn when_user_forgot_about_spaces() {
  let mut parser = InfixToPostfixParser::new();

  let result = parser.parse("3+4*2/(1-5)^2").unwrap();

  assert_eq!(result, "3 4 2 * 1 5 - 2 ^ / +");
}

#[test]
fn complex_expression() {
  let mut parser = InfixToPostfixParser::new();

  let result = parser.parse("3213 + 555555 * 111 / (122222222 - 15)^109").unwrap();

  assert_eq!(result, "3213 555555 111 * 122222222 15 - 109 ^ / +");
}

#[test]
fn error_when_left_bracket_is_missing() {
  let mut parser = InfixToPostfixParser::new();

  let result = parser.parse("1-1)");

  assert!(result.is_err());
}

#[test]
fn error_when_right_bracket_is_missing() {
  let mut parser = InfixToPostfixParser::new();

  let result = parser.parse("(1-1");

  assert!(result.is_err());
}

#[test]
fn complex_expression_with_letters() {
  let mut parser = InfixToPostfixParser::new();

  let result = parser.parse("a + b * cf / (a - b)^c").unwrap();

  assert_eq!(result, "a b cf * a b - c ^ / +");
}
