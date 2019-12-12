use crate::intcode_compute::computer_1202;
use std::fs;
use std::collections::VecDeque;

pub fn program_1205_01() -> i32 {
  let filename = "./src/aoc05/input.txt";
  let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
  let result = computer_1202(&contents, false, &mut VecDeque::from(vec![1]));
  result.output
}

pub fn program_1205_02() -> i32 {
  let filename = "./src/aoc05/input.txt";
  let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
  let result = computer_1202(&contents, false, &mut VecDeque::from(vec![5]));
  result.output
}
