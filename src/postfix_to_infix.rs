#[path = "parser.rs"]
mod parser;

use std::fmt::format;

use parser::Parser;
// https://www.geeksforgeeks.org/postfix-to-infix/
struct PostfixToInfix {
  stack: Vec<String>,
}

impl Parser for PostfixToInfix {
  fn parse(&mut self, expr: &str) -> Result<String, String> {
    self.check_expression(expr)?;
    self.main_parse(expr);
    Ok(self.stack.pop().unwrap())
  }
}

impl PostfixToInfix {
  fn new() -> PostfixToInfix {
    PostfixToInfix { stack: Vec::new() }
  }

  fn check_expression(&self, expr: &str) -> Result<(), String> {
    if expr.is_empty() {
      Err(String::from("Expression is empty."))
    } else {
      Ok(())
    }
  }

  fn main_parse(&mut self, expr: &str) {
    for substr in expr.split_whitespace() {
      if is_operand(substr) {
        self.parse_as_operand(substr);
      } else {
        self.parse_as_operator(substr);
      }
    }
  }

  fn parse_as_operand(&mut self, str: &str) {
    self.stack.push(str.to_string());
  }

  fn parse_as_operator(&mut self, str: &str) {
    let operand_2 = self.stack.pop().unwrap();
    let operand_1 = self.stack.pop().unwrap();
    let expr = format!("({} {} {})", operand_1, str, operand_2);
    self.stack.push(expr);
  }
}

fn is_operand(str: &str) -> bool {
  match str.chars().last() {
    Some(ch) => ch.is_digit(10),
    None => false,
  }
}

#[cfg(test)]
mod postfix_to_infix_tests {
  use super::*;

  #[test]
  fn when_user_forgot_about_expression() {
    let mut parser = PostfixToInfix::new();

    let result = parser.parse("");

    assert!(result.is_err());
  }

  #[test]
  fn complex_expression() {
    let mut parser = PostfixToInfix::new();

    let result = parser.parse("3213 555555 111 * 122222222 15 - 109 ^ / +").unwrap();

    assert_eq!(result, "(3213 + ((555555 * 111) / ((122222222 - 15) ^ 109)))");
  }
}
