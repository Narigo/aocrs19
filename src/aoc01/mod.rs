use std::fs;

pub fn sum_fuel_01() -> i64 {
  let filename = "./src/aoc01/input.txt";
  let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
  let inputs = contents.lines().map(|num| num.parse::<i64>().unwrap());
  let sum = sum_fuel_requirements_01(inputs);
  sum
}

pub fn sum_fuel_02() -> i64 {
  let filename = "./src/aoc01/input.txt";
  let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
  let inputs = contents.lines().map(|num| num.parse::<i64>().unwrap());
  let sum = sum_fuel_requirements_02(inputs);
  sum
}

fn sum_fuel_requirements_01(inputs: impl Iterator<Item = i64>) -> i64 {
  let sum = inputs.fold(0, |sum, mass| sum + fuel_requirement_of_mass(mass));
  sum
}

fn sum_fuel_requirements_02(inputs: impl Iterator<Item = i64>) -> i64 {
  let sum = inputs.fold(0, |sum, mass| {
    sum + fuel_requirement_of_mass_with_fuel(mass)
  });
  sum
}

fn fuel_requirement_of_mass_with_fuel(mass: i64) -> i64 {
  let fuel_for_mass = fuel_requirement_of_mass(mass);
  if fuel_for_mass > 0 {
    fuel_for_mass + fuel_requirement_of_mass_with_fuel(fuel_for_mass)
  } else {
    0
  }
}

fn fuel_requirement_of_mass(mass: i64) -> i64 {
  mass / 3 - 2
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn sums_fuel_requirements() {
    let expected_result = 3363033;
    assert_eq!(expected_result, sum_fuel_01())
  }

  #[test]
  fn sums_fuel_requirements_harder() {
    let expected_result = 5041680;
    assert_eq!(expected_result, sum_fuel_02())
  }

  #[test]
  fn complete_fuel_for_12_is_2() {
    let mass = 12;
    assert_eq!(2, fuel_requirement_of_mass_with_fuel(mass));
  }
  #[test]
  fn complete_fuel_for_14_is_2() {
    let mass = 14;
    assert_eq!(2, fuel_requirement_of_mass_with_fuel(mass));
  }
  #[test]
  fn complete_fuel_for_1969_is_966() {
    let mass = 1969;
    assert_eq!(966, fuel_requirement_of_mass_with_fuel(mass));
  }
  #[test]
  fn complete_fuel_for_100756_is_50346() {
    let mass = 100756;
    assert_eq!(50346, fuel_requirement_of_mass_with_fuel(mass));
  }
  #[test]
  fn fuel_for_12_is_2() {
    let mass = 12;
    assert_eq!(2, fuel_requirement_of_mass(mass));
  }
  #[test]
  fn fuel_for_14_is_2() {
    let mass = 14;
    assert_eq!(2, fuel_requirement_of_mass(mass));
  }
  #[test]
  fn fuel_for_1969_is_654() {
    let mass = 1969;
    assert_eq!(654, fuel_requirement_of_mass(mass));
  }
  #[test]
  fn fuel_for_100756_is_33583() {
    let mass = 100756;
    assert_eq!(33583, fuel_requirement_of_mass(mass));
  }
}
