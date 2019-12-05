pub fn find_amount_of_possible_passwords() -> u32 {
  let min = 284639;
  let max = 748759;
  let mut amount = 0;
  for num in min..max {
    if meets_criteria_part1(num) {
      amount = amount + 1;
    }
  }
  amount
}

pub fn find_possible_passwords() -> u32 {
  let min = 284639;
  let max = 748759;
  let mut amount = 0;
  for num in min..max {
    if meets_criteria_part2(num) {
      amount = amount + 1;
    }
  }
  amount
}

fn meets_criteria_part1(number: u32) -> bool {
  let as_string = format!("{}", number);
  has_two_adjacent_digits(&as_string) && digits_never_decrease(&as_string)
}

fn meets_criteria_part2(number: u32) -> bool {
  let as_string = format!("{}", number);
  has_two_adjacent_digits_in_no_larger_group(&as_string) && digits_never_decrease(&as_string)
}

fn has_two_adjacent_digits(number: &str) -> bool {
  if number.len() < 2 {
    return false;
  }

  let mut number_iter = number.chars();
  let mut last_digit = number_iter.next().unwrap();
  for digit in number_iter {
    if last_digit == digit {
      return true;
    }
    last_digit = digit;
  }
  return false;
}

fn has_two_adjacent_digits_in_no_larger_group(number: &str) -> bool {
  if number.len() < 2 {
    return false;
  }

  let mut number_iter = number.chars();
  let mut last_digit = number_iter.next().unwrap();
  let mut current_group_length = 1;
  for digit in number_iter {
    if last_digit == digit {
      current_group_length = current_group_length + 1;
    } else if current_group_length == 2 {
      return true;
    } else {
      current_group_length = 1;
    }
    last_digit = digit;
  }
  return current_group_length == 2;
}

fn digits_never_decrease(number: &str) -> bool {
  if number.len() < 2 {
    return true;
  }

  let mut number_iter = number.chars();
  let mut last_digit = number_iter.next().unwrap();
  for digit in number_iter {
    if last_digit > digit {
      return false;
    }
    last_digit = digit;
  }
  return true;
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn examples_part1() {
    assert!(meets_criteria_part1(111111));
    assert!(!meets_criteria_part1(223450));
    assert!(!meets_criteria_part1(123789));
  }

  #[test]
  fn examples_part2() {
    assert!(meets_criteria_part2(112233));
    assert!(meets_criteria_part2(111122));
    assert!(!meets_criteria_part2(111111));
    assert!(!meets_criteria_part2(123444));
    assert!(!meets_criteria_part2(223450));
    assert!(!meets_criteria_part2(123789));
  }

  #[test]
  fn larger_group() {
    assert!(has_two_adjacent_digits_in_no_larger_group("111122"));
    assert!(has_two_adjacent_digits_in_no_larger_group("1121"));
    assert!(!has_two_adjacent_digits_in_no_larger_group(""));
    assert!(!has_two_adjacent_digits_in_no_larger_group("1"));
    assert!(!has_two_adjacent_digits_in_no_larger_group("12"));
    assert!(!has_two_adjacent_digits_in_no_larger_group("111"));
    assert!(!has_two_adjacent_digits_in_no_larger_group("111111"));
  }

  #[test]
  fn decreasing_digits() {
    assert!(digits_never_decrease("11"));
    assert!(digits_never_decrease("111111"));
    assert!(digits_never_decrease("123345"));
    assert!(!digits_never_decrease("21"));
    assert!(!digits_never_decrease("125456"));
    assert!(!digits_never_decrease("121212"));
  }

  #[test]
  fn two_adjacent_digits() {
    assert!(has_two_adjacent_digits("11"));
    assert!(has_two_adjacent_digits("111111"));
    assert!(has_two_adjacent_digits("123345"));
    assert!(!has_two_adjacent_digits("1"));
    assert!(!has_two_adjacent_digits("123456"));
    assert!(!has_two_adjacent_digits("121212"));
  }
}
