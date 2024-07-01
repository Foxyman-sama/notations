#[path = "parser.rs"]
mod parser;

pub use parser::Parser;

pub struct PostfixToInfixParser {
  stack: Vec<String>,
}

impl Parser for PostfixToInfixParser {
  fn parse(&mut self, expr: &str) -> Result<String, String> {
    self.check_expression(expr)?;
    self.main_parse(expr);
    Ok(self.stack.pop().unwrap())
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
