pub struct Result {
  pub state: Vec<i32>,
  pub output: i32,
}

pub fn computer_1202(contents: &String, fix_data: bool, input_value: Option<i32>) -> Result {
  let input = contents
    .split(",")
    .map(|n| n.parse::<i32>().unwrap())
    .collect::<Vec<i32>>();
  let mut result = input.clone();
  if fix_data {
    result[1] = 12;
    result[2] = 2;
  }
  let mut current_input = input_value.clone();
  interprete(&mut result, 0, &mut current_input);
  Result {
    state: result,
    output: current_input.unwrap_or(0),
  }
}

pub fn interprete(result: &mut Vec<i32>, index: usize, current_input: &mut Option<i32>) {
  use Command::*;
  use Parameter::*;
  let opcode = result[index];
  let command = code_to_command(opcode);
  let get_by_offset = |mode: Parameter, offset: i32| match mode {
    Position => result[(index as i32 + offset) as usize] as usize,
    Immediate => (index as i32 + offset) as usize,
  };
  match command {
    Add(mode_a, mode_b, mode_position) => {
      let a = get_by_offset(mode_a, 1);
      let b = get_by_offset(mode_b, 2);
      let position = get_by_offset(mode_position, 3);
      let output = result[a] + result[b];
      result[position] = output;
      interprete(result, index + 4, current_input);
    }
    Multiply(mode_a, mode_b, mode_position) => {
      let a = get_by_offset(mode_a, 1);
      let b = get_by_offset(mode_b, 2);
      let position = get_by_offset(mode_position, 3);
      let output = result[a] * result[b];
      result[position] = output;
      interprete(result, index + 4, current_input);
    }
    Input(mode) => {
      let position = get_by_offset(mode, 1);
      result[position] = current_input.unwrap();
      interprete(result, index + 2, &mut None);
    }
    Output(mode) => {
      let position = get_by_offset(mode, 1);
      let next_input = result[position];
      interprete(result, index + 2, &mut Some(next_input));
    }
    JumpIfTrue(mode_a, mode_b) => {
      let test_non_zero = get_by_offset(mode_a, 1);
      let next_index = get_by_offset(mode_b, 2);
      if result[test_non_zero] != 0 {
        interprete(result, next_index, current_input);
      } else {
        interprete(result, index + 3, current_input);
      }
    }
    JumpIfFalse(mode_a, mode_b) => {
      let test_zero = get_by_offset(mode_a, 1);
      let next_index = get_by_offset(mode_b, 2);
      if result[test_zero] == 0 {
        interprete(result, next_index, current_input);
      } else {
        interprete(result, index + 3, current_input);
      }
    }
    LessThan(mode_a, mode_b, mode_position) => {
      let a = get_by_offset(mode_a, 1);
      let b = get_by_offset(mode_b, 2);
      let position = get_by_offset(mode_position, 3);
      if result[a] < result[b] {
        result[position] = 1;
      } else {
        result[position] = 0;
      }
    }
    Equals(mode_a, mode_b, mode_position) => {
      let a = get_by_offset(mode_a, 1);
      let b = get_by_offset(mode_b, 2);
      let position = get_by_offset(mode_position, 3);
      if result[a] == result[b] {
        result[position] = 1;
      } else {
        result[position] = 0;
      }
    }
    End => match current_input {
      Some(diagnostic_code) => println!("{}", diagnostic_code),
      _ => {}
    },
  }
}

enum Parameter {
  Immediate,
  Position,
}

enum Command {
  Add(Parameter, Parameter, Parameter),
  Multiply(Parameter, Parameter, Parameter),
  Input(Parameter),
  Output(Parameter),
  JumpIfTrue(Parameter, Parameter),
  JumpIfFalse(Parameter, Parameter),
  LessThan(Parameter, Parameter, Parameter),
  Equals(Parameter, Parameter, Parameter),
  End,
}

fn code_to_command(opcode: i32) -> Command {
  use Command::*;
  use Parameter::*;

  let operation = opcode % 100;
  let retrieve_param = |n: u32| {
    let mode = (opcode / (10 as i32).pow(n + 1)) % 10;
    if mode == 0 {
      Position
    } else {
      Immediate
    }
  };
  match operation {
    1 => Add(retrieve_param(1), retrieve_param(2), retrieve_param(3)),
    2 => Multiply(retrieve_param(1), retrieve_param(2), retrieve_param(3)),
    3 => Input(retrieve_param(1)),
    4 => Output(retrieve_param(1)),
    5 => JumpIfTrue(retrieve_param(1), retrieve_param(2)),
    6 => JumpIfFalse(retrieve_param(1), retrieve_param(2)),
    7 => LessThan(retrieve_param(1), retrieve_param(2), retrieve_param(3)),
    8 => Equals(retrieve_param(1), retrieve_param(2), retrieve_param(3)),
    99 => End,
    _ => panic!("nothing found! Something's off!"),
  }
}

#[cfg(test)]
mod test {
  use super::computer_1202;

  #[test]
  fn can_add_negative_numbers() {
    let input = "1101,100,-1,4,0";
    let expected = "[1101, 100, -1, 4, 99]";
    assert_eq!(
      expected,
      format!("{:?}", computer_1202(&input.to_owned(), false, None).state)
    )
  }

  #[test]
  fn input_parameter() {
    let input = "3,0,4,0,99";
    let expected = "[123, 0, 4, 0, 99]";
    assert_eq!(
      expected,
      format!(
        "{:?}",
        computer_1202(&input.to_owned(), false, Some(123)).state
      )
    )
  }
  #[test]
  fn can_add_with_immediate_mode() {
    let input = "1101,2,3,3,99";
    let expected = "[1101, 2, 3, 5, 99]";
    assert_eq!(
      expected,
      format!("{:?}", computer_1202(&input.to_owned(), false, None).state)
    )
  }

  #[test]
  fn can_multiply_with_immediate_mode() {
    let input = "1002,4,3,4,33";
    let expected = "[1002, 4, 3, 4, 99]";
    assert_eq!(
      expected,
      format!("{:?}", computer_1202(&input.to_owned(), false, None).state)
    )
  }

  #[test]
  fn adds_with_opcode_1() {
    let input = "1,2,3,3,99";
    let expected = "[1, 2, 3, 6, 99]";
    assert_eq!(
      expected,
      format!("{:?}", computer_1202(&input.to_owned(), false, None).state)
    )
  }

  #[test]
  fn multiplies_with_opcode_2() {
    let input = "2,2,3,3,99";
    let expected = "[2, 2, 3, 9, 99]";
    assert_eq!(
      expected,
      format!("{:?}", computer_1202(&input.to_owned(), false, None).state)
    )
  }

  #[test]
  fn works_with_multiple_opcodes() {
    let input = "1,2,3,3,2,2,3,7,99";
    let expected = "[1, 2, 3, 6, 2, 2, 3, 18, 99]";
    assert_eq!(
      expected,
      format!("{:?}", computer_1202(&input.to_owned(), false, None).state)
    )
  }

  #[test]
  fn can_change_an_opcode_before_running() {
    let input = "1,1,1,4,1,3,3,7,99";
    let expected = "[1, 1, 1, 4, 2, 3, 3, 16, 99]";
    assert_eq!(
      expected,
      format!("{:?}", computer_1202(&input.to_owned(), false, None).state)
    )
  }

  #[test]
  fn works_for_example_01() {
    let input = "1,0,0,0,99";
    let expected = "[2, 0, 0, 0, 99]";
    assert_eq!(
      expected,
      format!("{:?}", computer_1202(&input.to_owned(), false, None).state)
    )
  }

  #[test]
  fn works_for_example_02() {
    let input = "2,3,0,3,99";
    let expected = "[2, 3, 0, 6, 99]";
    assert_eq!(
      expected,
      format!("{:?}", computer_1202(&input.to_owned(), false, None).state)
    )
  }

  #[test]
  fn works_for_example_03() {
    let input = "2,4,4,5,99,0";
    let expected = "[2, 4, 4, 5, 99, 9801]";
    assert_eq!(
      expected,
      format!("{:?}", computer_1202(&input.to_owned(), false, None).state)
    )
  }

  #[test]
  fn works_for_example_04() {
    let input = "1,1,1,4,99,5,6,0,99";
    let expected = "[30, 1, 1, 4, 2, 5, 6, 0, 99]";
    assert_eq!(
      expected,
      format!("{:?}", computer_1202(&input.to_owned(), false, None).state)
    )
  }

  #[test]
  fn can_override_end_of_program() {
    let input = "1,1,1,4,99,9,10,8,0,3,33";
    let expected = "[1, 1, 1, 4, 2, 9, 10, 8, 99, 3, 33]";
    assert_eq!(
      expected,
      format!("{:?}", computer_1202(&input.to_owned(), false, None).state)
    )
  }

  #[test]
  fn can_fix_input_before_running() {
    let input = "1,1,1,3,99,1,0,1,0,1,0,1,0,1,0,1,0";
    let expected = "[1, 12, 2, 2, 99, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0]";
    assert_eq!(
      expected,
      format!("{:?}", computer_1202(&input.to_owned(), true, None).state)
    )
  }
}
