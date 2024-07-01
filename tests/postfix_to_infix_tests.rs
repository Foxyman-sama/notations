#[path = "../src/postfix_to_infix.rs"]
mod postfix_to_infix;

use postfix_to_infix::*;

#[test]
fn when_user_forgot_about_expression() {
  let mut parser = PostfixToInfixParser::new();

  let result = parser.parse("");

  assert!(result.is_err());
}

#[test]
fn complex_expression() {
  let mut parser = PostfixToInfixParser::new();

  let result = parser.parse("3213 555555 111 * 122222222 15 - 109 ^ / +").unwrap();

  assert_eq!(result, "(3213 + ((555555 * 111) / ((122222222 - 15) ^ 109)))");
}
