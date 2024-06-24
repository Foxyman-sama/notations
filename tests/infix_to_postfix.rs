#[cfg(test)]
mod infix_to_postfix {
  use super::*;

  struct InfixToPostfix {}

  impl InfixToPostfix {
    fn convert(expr: &str) -> String {
      let mut result = String::new();
      let mut stack: Vec<char> = Vec::new();
      for ch in expr.chars() {
        if ch.is_ascii_whitespace() {
          continue;
        } else if ch.is_ascii_digit() || ch == '.' {
          result.push(ch);
        } else if ch == '(' {
          stack.push(ch);
        } else if Self::is_operator(ch) {
          let right_op = ch;
          while stack.is_empty() == false
            && Self::is_operator(*stack.last().unwrap())
            && Self::precedence(*stack.last().unwrap(), right_op)
          {
            result.push(' ');
            result.push(ch)
          }

          result.push(' ');
          stack.push(right_op);
        } else if ch == ')' {
          while stack.is_empty() == false && *stack.last().unwrap() != '(' {
            result.push(' ');
            result.push(stack.pop().unwrap());
          }

          stack.pop();
          result.push(' ');
        }
      }

      while stack.is_empty() == false && '(' != *stack.last().unwrap() {
        result.push(' ');
        result.push(stack.pop().unwrap());
      }

      result
    }

    fn is_operator(ch: char) -> bool {
      match ch {
        '+' => true,
        '-' => true,
        '*' => true,
        '/' => true,
        '^' => true,
        '%' => true,
        _ => false,
      }
    }

    fn precedence(left_op: char, right_op: char) -> bool {
      if left_op == '^' {
        true
      } else if right_op == '^' {
        false
      } else if left_op == '*' || left_op == '/' || left_op == '%' {
        true
      } else if right_op == '*' || right_op == '/' || right_op == '%' {
        false
      } else {
        true
      }
    }
  }

  #[test]
  fn three_plus_five() {
    assert_eq!(InfixToPostfix::convert("3 + 5"), "3 5 +");
  }
}
