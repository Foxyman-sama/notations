pub fn convert(expr: &str) -> String {
  let mut result = String::new();
  let mut stack = Vec::new();

  for ch in expr.chars() {
    if ch.is_whitespace() {
      continue;
    }

    if ch.is_digit(10) {
      if result.is_empty() == false {
        result.push(' ');
      }
      result.push(ch);
    } else if ch == '(' {
      stack.push(ch);
    } else if ch == ')' {
      while stack.last().unwrap().ne(&'(') {
        push_into_string_space_and_from_stack(&mut result, &mut stack)
      }
      stack.pop();
    } else {
      while stack.is_empty() == false && precedence(ch) <= precedence(*stack.last().unwrap()) {
        push_into_string_space_and_from_stack(&mut result, &mut stack)
      }
      stack.push(ch);
    }
  }

  while stack.is_empty() == false {
    push_into_string_space_and_from_stack(&mut result, &mut stack)
  }

  result
}

fn push_into_string_space_and_from_stack(str: &mut String, stack: &mut Vec<char>) {
  str.push(' ');
  str.push(stack.pop().unwrap());
}

fn precedence(op: char) -> i32 {
  match op {
    '+' | '-' => 1,
    '*' | '/' => 2,
    '^' => 3,
    _ => 0,
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn three_plus_five() {
    assert_eq!(convert("3 + 5"), "3 5 +");
  }

  #[test]
  fn three_plus_five_multiply_five_with_brackets() {
    assert_eq!(convert("(3 + 5) * 7"), "3 5 + 7 *");
  }

  #[test]
  fn three_plus_five_multiply_five_without_brackets() {
    assert_eq!(convert("3 + 5 * 7"), "3 5 7 * +");
  }

  #[test]
  fn complex_expression() {
    assert_eq!(convert("3 + 4 * 2 / (1 - 5)^2"), "3 4 2 * 1 5 - 2 ^ / +");
  }

  #[test]
  fn simple_two_digit_expression() {
    assert_eq!(convert("34 + 57"), "34 57 +");
  }
}
