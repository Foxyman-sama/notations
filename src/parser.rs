pub trait Parser {
  fn parse(&mut self, expr: &str) -> String;

  fn parse_as_whitespace(&mut self);

  fn parse_as_digit(&mut self, ch: char);

  fn parse_as_open_bracket(&mut self);

  fn parse_as_closed_bracket(&mut self);

  fn parse_as_operator(&mut self, ch: char);
}
