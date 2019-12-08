pub struct ComputationResult {
  pub state: Vec<i32>,
  pub output: i32,
}

pub fn computer_1202(
  program: &String,
  fix_data: bool,
  input_values: Vec<i32>,
) -> ComputationResult {
  let input = program
    .split(",")
    .map(|n| n.parse::<i32>().unwrap())
    .collect::<Vec<i32>>();
  let mut result = input.clone();
  if fix_data {
    result[1] = 12;
    result[2] = 2;
  }
  let output = interprete(&mut result, 0, &mut input_values.clone());
  ComputationResult {
    state: result,
    output: output.unwrap_or(0),
  }
}

pub fn interprete(result: &mut Vec<i32>, index: usize, input_values: &mut Vec<i32>) -> Option<i32> {
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
      interprete(result, index + 4, input_values)
    }
    Multiply(mode_a, mode_b, mode_position) => {
      let a = get_by_offset(mode_a, 1);
      let b = get_by_offset(mode_b, 2);
      let position = get_by_offset(mode_position, 3);
      let output = result[a] * result[b];
      result[position] = output;
      interprete(result, index + 4, input_values)
    }
    Input(mode) => {
      let position = get_by_offset(mode, 1);
      result[position] = input_values
        .pop()
        .expect("Should have an input value left!");
      interprete(result, index + 2, input_values)
    }
    Output(mode) => {
      let position = get_by_offset(mode, 1);
      let next_input = result[position];
      input_values.push(next_input);
      interprete(result, index + 2, input_values)
    }
    JumpIfTrue(mode_a, mode_b) => {
      let test_non_zero = get_by_offset(mode_a, 1);
      let next_index = get_by_offset(mode_b, 2);
      if result[test_non_zero] != 0 {
        interprete(result, result[next_index] as usize, input_values)
      } else {
        interprete(result, index + 3, input_values)
      }
    }
    JumpIfFalse(mode_a, mode_b) => {
      let test_zero = get_by_offset(mode_a, 1);
      let next_index = get_by_offset(mode_b, 2);
      if result[test_zero] == 0 {
        interprete(result, result[next_index] as usize, input_values)
      } else {
        interprete(result, index + 3, input_values)
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
      interprete(result, index + 4, input_values)
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
      interprete(result, index + 4, input_values)
    }
    End => input_values.first().cloned(),
  }
}

#[derive(Debug)]
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
    _ => {
      println!("Looked at opcode {}", opcode);
      panic!("nothing found! Something's off!")
    }
  }
}

#[cfg(test)]
mod test {
  use super::computer_1202;

  #[test]
  fn equal_to_8() {
    let input = "3,9,8,9,10,9,4,9,99,-1,8";
    let eq_eight = vec![8];
    let not_eight_1 = vec![9];
    let not_eight_2 = vec![5];
    let not_eight_3 = vec![10];
    let not_eight_4 = vec![0];

    assert_eq!(1, computer_1202(&input.to_owned(), false, eq_eight).output);
    assert_eq!(
      0,
      computer_1202(&input.to_owned(), false, not_eight_1).output
    );
    assert_eq!(
      0,
      computer_1202(&input.to_owned(), false, not_eight_2).output
    );
    assert_eq!(
      0,
      computer_1202(&input.to_owned(), false, not_eight_3).output
    );
    assert_eq!(
      0,
      computer_1202(&input.to_owned(), false, not_eight_4).output
    );
  }
  #[test]
  fn larger_jump_example() {
    let input = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
    let lt_eight = vec![7];
    let eq_eight = vec![8];
    let gt_eight = vec![9];
    assert_eq!(
      999,
      computer_1202(&input.to_owned(), false, lt_eight).output
    );
    assert_eq!(
      1000,
      computer_1202(&input.to_owned(), false, eq_eight).output
    );
    assert_eq!(
      1001,
      computer_1202(&input.to_owned(), false, gt_eight).output
    );
  }
  #[test]
  fn jump_test_position_mode() {
    let input = "3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9";
    let input_parameter = vec![0];
    let expected_output = 0;
    assert_eq!(
      expected_output,
      computer_1202(&input.to_owned(), false, input_parameter).output
    );

    let input_parameter = vec![1];
    let expected_output = 1;
    assert_eq!(
      expected_output,
      computer_1202(&input.to_owned(), false, input_parameter).output
    )
  }

  #[test]
  fn jump_test_immediate_mode() {
    let input = "3,3,1105,-1,9,1101,0,0,12,4,12,99,1";
    let input_parameter = vec![0];
    let expected_output = 0;
    assert_eq!(
      expected_output,
      computer_1202(&input.to_owned(), false, input_parameter).output
    );

    let input_parameter = vec![1];
    let expected_output = 1;
    assert_eq!(
      expected_output,
      computer_1202(&input.to_owned(), false, input_parameter).output
    )
  }

  #[test]
  fn can_add_negative_numbers() {
    let input = "1101,100,-1,4,0";
    let expected = "[1101, 100, -1, 4, 99]";
    assert_eq!(
      expected,
      format!(
        "{:?}",
        computer_1202(&input.to_owned(), false, vec![]).state
      )
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
        computer_1202(&input.to_owned(), false, vec![123]).state
      )
    )
  }
  #[test]
  fn can_add_with_immediate_mode() {
    let input = "1101,2,3,3,99";
    let expected = "[1101, 2, 3, 5, 99]";
    assert_eq!(
      expected,
      format!(
        "{:?}",
        computer_1202(&input.to_owned(), false, vec![]).state
      )
    )
  }

  #[test]
  fn can_multiply_with_immediate_mode() {
    let input = "1002,4,3,4,33";
    let expected = "[1002, 4, 3, 4, 99]";
    assert_eq!(
      expected,
      format!(
        "{:?}",
        computer_1202(&input.to_owned(), false, vec![]).state
      )
    )
  }

  #[test]
  fn adds_with_opcode_1() {
    let input = "1,2,3,3,99";
    let expected = "[1, 2, 3, 6, 99]";
    assert_eq!(
      expected,
      format!(
        "{:?}",
        computer_1202(&input.to_owned(), false, vec![]).state
      )
    )
  }

  #[test]
  fn multiplies_with_opcode_2() {
    let input = "2,2,3,3,99";
    let expected = "[2, 2, 3, 9, 99]";
    assert_eq!(
      expected,
      format!(
        "{:?}",
        computer_1202(&input.to_owned(), false, vec![]).state
      )
    )
  }

  #[test]
  fn works_with_multiple_opcodes() {
    let input = "1,2,3,3,2,2,3,7,99";
    let expected = "[1, 2, 3, 6, 2, 2, 3, 18, 99]";
    assert_eq!(
      expected,
      format!(
        "{:?}",
        computer_1202(&input.to_owned(), false, vec![]).state
      )
    )
  }

  #[test]
  fn can_change_an_opcode_before_running() {
    let input = "1,1,1,4,1,3,3,7,99";
    let expected = "[1, 1, 1, 4, 2, 3, 3, 16, 99]";
    assert_eq!(
      expected,
      format!(
        "{:?}",
        computer_1202(&input.to_owned(), false, vec![]).state
      )
    )
  }

  #[test]
  fn works_for_example_01() {
    let input = "1,0,0,0,99";
    let expected = "[2, 0, 0, 0, 99]";
    assert_eq!(
      expected,
      format!(
        "{:?}",
        computer_1202(&input.to_owned(), false, vec![]).state
      )
    )
  }

  #[test]
  fn works_for_example_02() {
    let input = "2,3,0,3,99";
    let expected = "[2, 3, 0, 6, 99]";
    assert_eq!(
      expected,
      format!(
        "{:?}",
        computer_1202(&input.to_owned(), false, vec![]).state
      )
    )
  }

  #[test]
  fn works_for_example_03() {
    let input = "2,4,4,5,99,0";
    let expected = "[2, 4, 4, 5, 99, 9801]";
    assert_eq!(
      expected,
      format!(
        "{:?}",
        computer_1202(&input.to_owned(), false, vec![]).state
      )
    )
  }

  #[test]
  fn works_for_example_04() {
    let input = "1,1,1,4,99,5,6,0,99";
    let expected = "[30, 1, 1, 4, 2, 5, 6, 0, 99]";
    assert_eq!(
      expected,
      format!(
        "{:?}",
        computer_1202(&input.to_owned(), false, vec![]).state
      )
    )
  }

  #[test]
  fn can_override_end_of_program() {
    let input = "1,1,1,4,99,9,10,8,0,3,33";
    let expected = "[1, 1, 1, 4, 2, 9, 10, 8, 99, 3, 33]";
    assert_eq!(
      expected,
      format!(
        "{:?}",
        computer_1202(&input.to_owned(), false, vec![]).state
      )
    )
  }

  #[test]
  fn can_fix_input_before_running() {
    let input = "1,1,1,3,99,1,0,1,0,1,0,1,0,1,0,1,0";
    let expected = "[1, 12, 2, 2, 99, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0]";
    assert_eq!(
      expected,
      format!("{:?}", computer_1202(&input.to_owned(), true, vec![]).state)
    )
  }
}
