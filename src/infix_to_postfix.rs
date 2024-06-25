pub fn convert(expr: &str) -> String {
  let mut result = String::new();
  let mut stack: Vec<char> = Vec::new();
  for ch in expr.chars() {
    if ch.is_ascii_digit() {
      result.push(ch);
    } else if ch == '(' {
      stack.push(ch);
    } else if is_operator(ch) {
      result = format!("{}{} ", result, process_operators(ch, &mut stack));
    } else if ch == ')' {
      result = format!("{}{}", result, extract_until_open_bracket(&mut stack));
      stack.pop();
    }
  }

  format!("{}{}", result, extract_until_closed_bracket(&mut stack))
}

fn process_operators(right_op: char, stack: &mut Vec<char>) -> String {
  let mut result = String::new();

  if stack.is_empty() == false {
    let left_op = *stack.last().unwrap();
    while stack.is_empty() == false && is_operator(left_op) && precedence(left_op, right_op) {
      result = format!(" {}{}", result, left_op);
    }
  }
  stack.push(right_op);

  result
}

fn is_operator(ch: char) -> bool {
  match ch {
    '*' | '/' | '+' | '-' | '^' | '%' => true,
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

fn extract_until_open_bracket(stack: &mut Vec<char>) -> String {
  extract_while_brackets(stack, '(')
}

fn extract_until_closed_bracket(stack: &mut Vec<char>) -> String {
  extract_while_brackets(stack, ')')
}

fn extract_while_brackets(stack: &mut Vec<char>, type_of_brackets: char) -> String {
  let mut result = String::new();

  while stack.is_empty() == false && *stack.last().unwrap() != type_of_brackets {
    result = format!("{} {}", result, stack.pop().unwrap());
  }

  result
}
