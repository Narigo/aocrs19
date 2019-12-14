use crate::intcode_compute::{computer_1202, Amplifier};
use std::collections::VecDeque;
use std::fs;

pub fn program_1202_01() -> i32 {
  let filename = "./src/aoc02/input.txt";
  let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
  computer_1202(&contents, true, &mut VecDeque::new()).state[0]
}

pub fn program_1202_02() -> i32 {
  let filename = "./src/aoc02/input.txt";
  let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
  let input = contents
    .split(",")
    .map(|n| n.parse::<i32>().unwrap())
    .collect::<Vec<i32>>();
  for noun in 0..100 {
    for verb in 0..100 {
      let mut result = input.clone();
      result[1] = noun;
      result[2] = verb;
      let mut amplifier = Amplifier {
        program: result
          .iter()
          .map(|x| format!("{}", x))
          .collect::<Vec<String>>()
          .join(","),
        phase_setting: 0,
        input_value: VecDeque::new(),
        output_value: None,
      };
      amplifier.interprete(&mut result, 0, &mut VecDeque::new());
      if result[0] == 19690720 {
        return 100 * noun + verb;
      }
    }
  }
  panic!("should have found something!");
}
