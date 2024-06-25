use notations::infix_to_postfix::convert;

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
