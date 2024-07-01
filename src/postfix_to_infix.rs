#[path = "parser.rs"]
mod parser;

pub use parser::Parser;

pub struct PostfixToInfixParser {
  stack: Vec<String>,
}

impl Parser for PostfixToInfixParser {
  fn parse(&mut self, expr: &str) -> Result<String, String> {
    self.check_expression(expr)?;
    self.try_parse(expr)?;
    self.check_stack()
  }
}

impl PostfixToInfixParser {
  pub fn new() -> PostfixToInfixParser {
    PostfixToInfixParser { stack: Vec::new() }
  }

  fn check_expression(&self, expr: &str) -> Result<(), String> {
    if expr.is_empty() {
      Err(String::from("Expression is empty."))
    } else {
      Ok(())
    }
  }

  fn try_parse(&mut self, expr: &str) -> Result<(), String> {
    for substr in expr.split_whitespace() {
      if self.is_operand(substr) {
        self.parse_as_operand(substr);
      } else {
        self.parse_as_operator(substr)?;
      }
    }
    Ok(())
  }

  fn is_operand(&mut self, str: &str) -> bool {
    match str.chars().last() {
      Some(ch) => ch.is_digit(10),
      None => false,
    }
  }

  fn parse_as_operand(&mut self, str: &str) {
    self.stack.push(str.to_string());
  }

  fn parse_as_operator(&mut self, str: &str) -> Result<(), String> {
    let operand_2 = self.pop_operand()?;
    let operand_1 = self.pop_operand()?;
    let expr = format!("({} {} {})", operand_1, str, operand_2);
    self.stack.push(expr);
    Ok(())
  }

  fn pop_operand(&mut self) -> Result<String, String> {
    if self.stack.is_empty() {
      Err(String::from("Wrong number of operands."))
    } else {
      Ok(self.stack.pop().unwrap())
    }
  }

  fn check_stack(&mut self) -> Result<String, String> {
    if self.stack.len() > 1 {
      Err(String::from("Wrong number of operators."))
    } else {
      Ok(self.stack.pop().unwrap())
    }
  }
}
