use crate::intcode_compute::computer_1202;
use std::collections::VecDeque;
use std::fs;

pub fn program_1205_01() -> i64 {
  let filename = "./src/aoc05/input.txt";
  let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
  let result = computer_1202(&contents, false, &mut VecDeque::from(vec![1]));
  *result.output.back().expect("should have an output value")
}

pub fn program_1205_02() -> i64 {
  let filename = "./src/aoc05/input.txt";
  let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
  let result = computer_1202(&contents, false, &mut VecDeque::from(vec![5]));
  *result.output.back().expect("should have an output value")
}
