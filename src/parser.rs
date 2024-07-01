pub trait Parser {
  fn parse(&mut self, expr: &str) -> Result<String, String>;
}
