extern crate itertools;
use itertools::Itertools;

use crate::intcode_compute::Amplifier;
use std::fs;

pub fn program_1207_01() -> i64 {
  let filename = "./src/aoc07/input.txt";
  let amplifier_program =
    fs::read_to_string(filename).expect("Something went wrong reading the file");
  find_maximum(&amplifier_program)
}

pub fn program_1207_02() -> i64 {
  let filename = "./src/aoc07/input.txt";
  let amplifier_program =
    fs::read_to_string(filename).expect("Something went wrong reading the file");
  find_maximum_with_feedback(&amplifier_program)
}

fn find_maximum(amplifier_program: &String) -> i64 {
  let permutations = (0..5).permutations(5);
  let mut maximum_output = std::i64::MIN;
  for permutation in permutations {
    let mut input_output_value = 0;
    for phase_setting in permutation.iter() {
      let mut amplifier = Amplifier::new(amplifier_program.clone(), Some(*phase_setting as i64));
      input_output_value = *amplifier
        .calculate_output(input_output_value)
        .back()
        .expect("should have an output value");
    }
    if maximum_output < input_output_value {
      maximum_output = input_output_value;
    }
  }
  maximum_output
}

fn find_maximum_with_feedback(amplifier_program: &String) -> i64 {
  let permutations = (5..10).permutations(5);
  let mut maximum_output = std::i64::MIN;
  for permutation in permutations {
    let max_of_permutation =
      find_maximum_with_feedback_of_permutation(amplifier_program, permutation);

    if maximum_output < max_of_permutation {
      maximum_output = max_of_permutation;
    }
  }
  maximum_output
}

fn find_maximum_with_feedback_of_permutation(
  amplifier_program: &String,
  permutation: Vec<i64>,
) -> i64 {
  let mut max_of_permutation = std::i64::MIN;
  let mut last_input_output_value = std::i64::MIN;
  let mut input_output_value = 0;

  let mut amplifiers = Vec::new();
  for phase_setting in permutation.iter() {
    let amplifier = Amplifier::new(amplifier_program.clone(), Some(*phase_setting as i64));
    amplifiers.push(amplifier);
  }

  while last_input_output_value < input_output_value {
    last_input_output_value = input_output_value;
    for amplifier in amplifiers.iter_mut() {
      input_output_value = *amplifier
        .calculate_output(input_output_value)
        .back()
        .expect("should have an output value");
    }
    if max_of_permutation < input_output_value {
      max_of_permutation = input_output_value;
    }
  }
  max_of_permutation
}

#[cfg(test)]
pub mod test {
  use super::*;

  #[test]
  fn example_amplifier_1_feedback_looped() {
    let program =
      "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5";
    let expected = 139629729;
    let result =
      find_maximum_with_feedback_of_permutation(&program.to_owned(), vec![9, 8, 7, 6, 5]);
    assert_eq!(expected, result);
  }

  #[test]
  pub fn example_amplifier_2_feedback_looped() {
    let program =
      "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10";
    let expected = 18216;
    let result =
      find_maximum_with_feedback_of_permutation(&program.to_owned(), vec![9, 7, 8, 5, 6]);
    assert_eq!(expected, result);
  }

  #[test]
  fn example_amplifier_1() {
    let program = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";
    let expected = 43210;
    let result = find_maximum(&program.to_owned());
    assert_eq!(expected, result);
  }

  #[test]
  fn example_amplifier_2() {
    let program = "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0";
    let expected = 54321;
    let result = find_maximum(&program.to_owned());
    assert_eq!(expected, result);
  }

  #[test]
  fn example_amplifier_3() {
    let program = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0";
    let expected = 65210;
    let result = find_maximum(&program.to_owned());
    assert_eq!(expected, result);
  }

  #[test]
  fn two_amplifiers_1_to_2() {
    let program = "3,16,3,15,2,15,16,15,1,15,16,17,4,17,99,-1,-1,-1".to_owned();
    let mut amplifier1 = Amplifier::new(program.to_owned(), Some(0));
    let mut amplifier2 = Amplifier::new(program.to_owned(), Some(1));
    let result_1_2 = amplifier2.calculate_output(
      amplifier1
        .calculate_output(2)
        .expect("should have an output value"),
    );
    assert_eq!(1, result_1_2.expect("should have an output value"));
  }

  #[test]
  fn two_amplifiers_2_to_1() {
    let program = "3,16,3,15,2,15,16,15,1,15,16,17,4,17,99,-1,-1,-1".to_owned();
    let mut amplifier1 = Amplifier::new(program.to_owned(), Some(0));
    let mut amplifier2 = Amplifier::new(program.to_owned(), Some(1));
    let result_2_1 = amplifier1.calculate_output(
      amplifier2
        .calculate_output(2)
        .expect("should have an output value"),
    );
    assert_eq!(0, result_2_1.expect("should have an output value"));
  }

  #[test]
  fn multiple_input_values() {
    let mut amplifier = Amplifier::new("3,11,3,12,1,11,12,13,4,13,99,-1,-1,-1".to_owned(), Some(1));
    let result = amplifier.calculate_output(2);
    assert_eq!(3, result.expect("should have an output value"));
  }
}
