#[cfg(test)]
mod infix_to_postfix {
  use super::*;

  fn infix_to_postfix(expr: &str) -> String {
    let mut result = String::new();
    for ch in expr.chars() {
      if ch.is_ascii_digit() {
        result.push(ch);
      }
    }

    result
  }

  #[test]
  fn three_plus_five() {
    assert_eq!(infix_to_postfix("3 + 5"), "3 5 +");
  }
}
