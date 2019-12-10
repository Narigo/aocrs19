extern crate itertools;
use itertools::Itertools;

use crate::intcode_compute::computer_1202;
use std::collections::VecDeque;
use std::fs;

pub fn program_1207_01() -> i32 {
  let filename = "./src/aoc07/input.txt";
  let amplifier_program =
    fs::read_to_string(filename).expect("Something went wrong reading the file");
  find_maximum(&amplifier_program)
}

fn find_maximum(amplifier_program: &String) -> i32 {
  let permutations = (0..5).permutations(5);
  let mut maximum_output = std::i32::MIN;
  for permutation in permutations {
    let mut input_output_value = 0;
    for phase_setting in permutation.iter() {
      let amplifier = Amplifier {
        program: amplifier_program.clone(),
        phase_setting: *phase_setting as i32,
      };
      input_output_value = amplifier.calculate_output(input_output_value);
    }
    if maximum_output < input_output_value {
      maximum_output = input_output_value;
    }
  }
  maximum_output
}

#[derive(Clone, Debug)]
struct Amplifier {
  program: String,
  phase_setting: i32,
}

impl Amplifier {
  fn calculate_output(&self, input_value: i32) -> i32 {
    let result = computer_1202(
      &self.program,
      false,
      VecDeque::from(vec![input_value, self.phase_setting]),
    );
    result.output
  }
}

#[cfg(test)]
mod test {
  use super::*;

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
  fn two_amplifiers() {
    let program = "3,16,3,15,2,15,16,15,1,15,16,17,4,17,99,-1,-1,-1".to_owned();
    let amplifier1 = Amplifier {
      program: program.to_owned(),
      phase_setting: 0,
    };
    let amplifier2 = Amplifier {
      program: program.to_owned(),
      phase_setting: 1,
    };
    let result_1_2 = amplifier2.calculate_output(amplifier1.calculate_output(2));
    assert_eq!(1, result_1_2);
    let result_2_1 = amplifier1.calculate_output(amplifier2.calculate_output(2));
    assert_eq!(0, result_2_1);
  }

  #[test]
  fn multiple_input_values() {
    let amplifier = Amplifier {
      program: "3,11,3,12,1,11,12,13,4,13,99,-1,-1,-1".to_owned(),
      phase_setting: 1,
    };
    let result = amplifier.calculate_output(2);
    assert_eq!(3, result);
  }
}
