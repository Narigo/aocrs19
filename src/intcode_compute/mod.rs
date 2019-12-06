pub fn computer_1202(contents: &String, fix_data: bool) -> Vec<usize> {
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

pub fn interprete(result: &mut Vec<usize>, index: usize) {
  use Command::*;
  use Parameter::*;
  let opcode = result[index];
  let command = code_to_command(opcode);
  let get_by_offset = |mode: Parameter, offset: usize| match mode {
    Position => result[index + offset],
    Immediate => index + offset,
  };
  match command {
    Add(mode_a, mode_b, mode_position) => {
      let a = get_by_offset(mode_a, 1);
      let b = get_by_offset(mode_b, 2);
      let position = get_by_offset(mode_position, 3);
      let output = result[a] + result[b];
      result[position] = output;
      interprete(result, index + 4);
    }
    Multiply(mode_a, mode_b, mode_position) => {
      let a = get_by_offset(mode_a, 1);
      let b = get_by_offset(mode_b, 2);
      let position = get_by_offset(mode_position, 3);
      let output = result[a] * result[b];
      result[position] = output;
      interprete(result, index + 4);
    }
    End => {}
  }
}

enum Parameter {
  Immediate,
  Position,
}

enum Command {
  Add(Parameter, Parameter, Parameter),
  Multiply(Parameter, Parameter, Parameter),
  End,
}

fn code_to_command(opcode: usize) -> Command {
  use Command::*;
  use Parameter::*;

  let operation = opcode % 100;
  let retrieve_param = |n: u32| {
    let mode = (opcode / (10 as usize).pow(n + 1)) % 10;
    if mode == 0 {
      Position
    } else {
      Immediate
    }
  };
  match operation {
    1 => Add(retrieve_param(1), retrieve_param(2), retrieve_param(3)),
    2 => Multiply(retrieve_param(1), retrieve_param(2), retrieve_param(3)),
    99 => End,
    _ => panic!("nothing found! Something's off!"),
  }
}

#[cfg(test)]
mod test {
  use super::computer_1202;

  #[test]
  fn can_add_with_immediate_mode() {
    let input = "1101,2,3,3,99";
    let expected = "[1101, 2, 3, 5, 99]";
    assert_eq!(
      expected,
      format!("{:?}", computer_1202(&input.to_owned(), false))
    )
  }

  #[test]
  fn can_multiply_with_immediate_mode() {
    let input = "1002,4,3,4,33";
    let expected = "[1002, 4, 3, 4, 99]";
    assert_eq!(
      expected,
      format!("{:?}", computer_1202(&input.to_owned(), false))
    )
  }

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
