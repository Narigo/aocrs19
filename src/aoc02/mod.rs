use std::fs;

pub fn program_1202_01() -> usize {
  let filename = "./src/aoc02/input.txt";
  let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
  computer_1202(&contents, true)[0]
}

pub fn program_1202_02() -> usize {
  let filename = "./src/aoc02/input.txt";
  let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
  let input = contents
    .split(",")
    .map(|n| n.parse::<usize>().unwrap())
    .collect::<Vec<usize>>();
  for noun in 0..100 {
    for verb in 0..100 {
      let mut result = input.clone();
      result[1] = noun;
      result[2] = verb;
      interprete(&mut result, 0);
      if result[0] == 19690720 {
        return 100 * noun + verb;
      }
    }
  }
  panic!("should have found something!");
}

fn computer_1202(contents: &String, fix_data: bool) -> Vec<usize> {
  let input = contents
    .split(",")
    .map(|n| n.parse::<usize>().unwrap())
    .collect::<Vec<usize>>();
  let mut result = input.clone();
  if fix_data {
    result[1] = 12;
    result[2] = 2;
  }
  interprete(&mut result, 0);
  result
}

fn interprete(result: &mut Vec<usize>, index: usize) {
  let opcode = result[index];
  match opcode {
    1 => {
      let a = result[result[index + 1]];
      let b = result[result[index + 2]];
      let position = result[index + 3];
      let output = a + b;
      result[position] = output;
      interprete(result, index + 4);
    }
    2 => {
      let a = result[result[index + 1]];
      let b = result[result[index + 2]];
      let position = result[index + 3];
      let output = a * b;
      result[position] = output;
      interprete(result, index + 4);
    }
    99 => {}
    _ => {
      println!("nothing found! Something's off!");
    }
  }
}

#[cfg(test)]
mod test {
  use super::computer_1202;

  #[test]
  fn adds_with_opcode_1() {
    let input = "1,2,3,3,99";
    let expected = "[1, 2, 3, 6, 99]";
    assert_eq!(
      expected,
      format!("{:?}", computer_1202(&input.to_owned(), false))
    )
  }

  #[test]
  fn multiplies_with_opcode_2() {
    let input = "2,2,3,3,99";
    let expected = "[2, 2, 3, 9, 99]";
    assert_eq!(
      expected,
      format!("{:?}", computer_1202(&input.to_owned(), false))
    )
  }

  #[test]
  fn works_with_multiple_opcodes() {
    let input = "1,2,3,3,2,2,3,7,99";
    let expected = "[1, 2, 3, 6, 2, 2, 3, 18, 99]";
    assert_eq!(
      expected,
      format!("{:?}", computer_1202(&input.to_owned(), false))
    )
  }

  #[test]
  fn can_change_an_opcode_before_running() {
    let input = "1,1,1,4,1,3,3,7,99";
    let expected = "[1, 1, 1, 4, 2, 3, 3, 16, 99]";
    assert_eq!(
      expected,
      format!("{:?}", computer_1202(&input.to_owned(), false))
    )
  }

  #[test]
  fn works_for_example_01() {
    let input = "1,0,0,0,99";
    let expected = "[2, 0, 0, 0, 99]";
    assert_eq!(
      expected,
      format!("{:?}", computer_1202(&input.to_owned(), false))
    )
  }

  #[test]
  fn works_for_example_02() {
    let input = "2,3,0,3,99";
    let expected = "[2, 3, 0, 6, 99]";
    assert_eq!(
      expected,
      format!("{:?}", computer_1202(&input.to_owned(), false))
    )
  }

  #[test]
  fn works_for_example_03() {
    let input = "2,4,4,5,99,0";
    let expected = "[2, 4, 4, 5, 99, 9801]";
    assert_eq!(
      expected,
      format!("{:?}", computer_1202(&input.to_owned(), false))
    )
  }

  #[test]
  fn works_for_example_04() {
    let input = "1,1,1,4,99,5,6,0,99";
    let expected = "[30, 1, 1, 4, 2, 5, 6, 0, 99]";
    assert_eq!(
      expected,
      format!("{:?}", computer_1202(&input.to_owned(), false))
    )
  }

  #[test]
  fn can_override_end_of_program() {
    let input = "1,1,1,4,99,9,10,8,0,3,33";
    let expected = "[1, 1, 1, 4, 2, 9, 10, 8, 99, 3, 33]";
    assert_eq!(
      expected,
      format!("{:?}", computer_1202(&input.to_owned(), false))
    )
  }

  #[test]
  fn can_fix_input_before_running() {
    let input = "1,1,1,3,99,1,0,1,0,1,0,1,0,1,0,1,0";
    let expected = "[1, 12, 2, 2, 99, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0]";
    assert_eq!(
      expected,
      format!("{:?}", computer_1202(&input.to_owned(), true))
    )
  }
}
