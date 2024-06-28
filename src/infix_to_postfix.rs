#[path = "parser.rs"]
mod parser;

struct InfixToPostfixParser {
  result: String,
  stack: Vec<char>,
  can_add_num: bool,
}

impl parser::Parser for InfixToPostfixParser {
  fn parse(&mut self, expr: &str) -> String {
    self.main_parse(expr);
    self.extract_remaining_operators();
    self.result.clone()
  }

  fn parse_as_whitespace(&mut self) {}

  fn parse_as_digit(&mut self, digit: char) {
    if self.result.is_empty() == false && self.can_add_num == false {
      self.result.push(' ');
    }
    self.result.push(digit);
    self.can_add_num = true;
  }

  fn parse_as_open_bracket(&mut self) {
    self.can_add_num = false;
    self.stack.push('(');
  }

  fn parse_as_closed_bracket(&mut self) {
    self.can_add_num = false;
    while self.stack.last().unwrap().ne(&'(') {
      self.insert_space_and_pop_from_stack();
    }
    self.stack.pop();
  }

  fn parse_as_operator(&mut self, op: char) {
    self.can_add_num = false;
    while self.stack.is_empty() == false && self.precedence(op) {
      self.insert_space_and_pop_from_stack();
    }
    self.stack.push(op);
  }
}

impl InfixToPostfixParser {
  fn new() -> InfixToPostfixParser {
    InfixToPostfixParser {
      stack: Vec::new(),
      result: String::new(),
      can_add_num: false,
    }
  }

  fn main_parse(&mut self, expr: &str) {
    use parser::Parser;

    for ch in expr.chars() {
      match ch {
        ' ' => self.parse_as_whitespace(),
        '0'..='9' => self.parse_as_digit(ch),
        '(' => self.parse_as_open_bracket(),
        ')' => self.parse_as_closed_bracket(),
        _ => self.parse_as_operator(ch),
      }
    }
  }

  fn extract_remaining_operators(&mut self) {
    while self.stack.is_empty() == false {
      self.insert_space_and_pop_from_stack();
    }
  }

  fn insert_space_and_pop_from_stack(&mut self) {
    self.result.push(' ');
    self.result.push(self.stack.pop().unwrap());
  }

  fn precedence(&mut self, left_op: char) -> bool {
    let left_precedence = Self::precedence_for_one_operator(left_op);
    let right_precedence = Self::precedence_for_one_operator(*self.stack.last().unwrap());
    left_precedence <= right_precedence
  }

  fn precedence_for_one_operator(op: char) -> i32 {
    match op {
      '+' | '-' => 1,
      '*' | '/' => 2,
      '^' => 3,
      _ => 0,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use parser::Parser;

  #[test]
  fn when_user_forgot_about_expression() {
    let mut parser = InfixToPostfixParser::new();

    let result = parser.parse("");

    assert_eq!(result, "");
  }

  #[test]
  fn when_user_forgot_about_spaces() {
    let mut parser = InfixToPostfixParser::new();

    let result = parser.parse("3+4*2/(1-5)^2");

    assert_eq!(result, "3 4 2 * 1 5 - 2 ^ / +");
  }

  #[test]
  fn complex_expression() {
    let mut parser = InfixToPostfixParser::new();

    let result = parser.parse("3213 + 555555 * 111 / (122222222 - 15)^109");

    assert_eq!(result, "3213 555555 111 * 122222222 15 - 109 ^ / +");
  }
}
