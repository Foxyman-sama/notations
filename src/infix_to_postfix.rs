#[path = "parser.rs"]
mod parser;

pub use parser::Parser;

pub struct InfixToPostfixParser {
  result: String,
  stack: Vec<char>,
  can_add_operand: bool,
}

impl Parser for InfixToPostfixParser {
  fn parse(&mut self, expr: &str) -> Result<String, String> {
    self.check_expression(expr)?;
    self.main_parse(expr)?;
    self.extract_remaining_operators()?;
    Ok(std::mem::take(&mut self.result))
  }
}

impl InfixToPostfixParser {
  pub fn new() -> InfixToPostfixParser {
    InfixToPostfixParser {
      stack: Vec::new(),
      result: String::new(),
      can_add_operand: false,
    }
  }

  fn check_expression(&self, expr: &str) -> Result<(), String> {
    if expr.is_empty() {
      Err(String::from("Expression is empty."))
    } else {
      Ok(())
    }
  }

  fn main_parse(&mut self, expr: &str) -> Result<(), String> {
    for ch in expr.chars() {
      match ch {
        ' ' => (),
        '0'..='9' | 'a'..='z' | 'A'..='Z' => self.parse_as_operand(ch),
        _ => self.parse_other(ch)?,
      }
    }

    Ok(())
  }

  fn parse_as_operand(&mut self, digit: char) {
    if self.result.is_empty() == false && self.can_add_operand == false {
      self.result.push(' ');
    }
    self.result.push(digit);
    self.can_add_operand = true;
  }

  fn parse_other(&mut self, ch: char) -> Result<(), String> {
    self.can_add_operand = false;
    match ch {
      '(' => self.parse_as_open_bracket(),
      ')' => self.parse_as_closed_bracket()?,
      _ => self.parse_as_operator(ch),
    }

    Ok(())
  }

  fn parse_as_open_bracket(&mut self) {
    self.stack.push('(');
  }

  fn parse_as_closed_bracket(&mut self) -> Result<(), String> {
    while self.stack.is_empty() == false && self.stack.last().unwrap().ne(&'(') {
      self.insert_space_and_pop_from_stack();
    }

    if self.stack.is_empty() {
      return Err(String::from("Missing left bracket."));
    }

    self.stack.pop();
    Ok(())
  }

  fn parse_as_operator(&mut self, op: char) {
    while self.stack.is_empty() == false && self.precedence(op) {
      self.insert_space_and_pop_from_stack();
    }
    self.stack.push(op);
  }
  fn extract_remaining_operators(&mut self) -> Result<(), String> {
    while self.stack.is_empty() == false && self.stack.last().unwrap().ne(&'(') {
      self.insert_space_and_pop_from_stack();
    }

    if self.stack.is_empty() == false {
      self.stack.clear();
      return Err(String::from("Missing right bracket."));
    }

    Ok(())
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
