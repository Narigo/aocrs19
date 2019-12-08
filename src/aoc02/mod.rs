use crate::intcode_compute::{computer_1202, interprete};
use std::fs;

pub fn program_1202_01() -> i32 {
  let filename = "./src/aoc02/input.txt";
  let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
  computer_1202(&contents, true, vec![]).state[0]
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
      interprete(&mut result, 0, &mut vec![]);
      if result[0] == 19690720 {
        return 100 * noun + verb;
      }
    }
  }
  panic!("should have found something!");
}
