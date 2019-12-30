use crate::intcode_compute::computer_1202;
use std::collections::VecDeque;
use std::fs;

pub fn boost_01() -> VecDeque<i64> {
  let filename = "./src/aoc09/input.txt";
  let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
  let result = computer_1202(&contents, false, &mut VecDeque::from(vec![1]));
  result.output
}
