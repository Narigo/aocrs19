use std::collections::VecDeque;

pub struct ComputationResult {
  pub state: Vec<i64>,
  pub output: VecDeque<i64>,
}

pub fn computer_1202(
  program: &String,
  fix_data: bool,
  input_values: &mut VecDeque<i64>,
) -> ComputationResult {
  let input = program
    .split(",")
    .map(|n| n.parse::<i64>().unwrap())
    .collect::<Vec<i64>>();
  let mut result = input.clone();
  if fix_data {
    result[1] = 12;
    result[2] = 2;
  }
  let program = result
    .iter()
    .map(|x| format!("{}", x))
    .collect::<Vec<String>>()
    .join(",");
  let mut amplifier = Amplifier::new(program, None);
  amplifier.input_value = input_values.clone();

  let output = amplifier.interprete();
  ComputationResult {
    state: amplifier.program,
    output,
  }
}

#[derive(Clone, Debug)]
pub struct Amplifier {
  pub phase_setting: Option<i64>,
  pub program: Vec<i64>,
  pub index: usize,
  pub input_value: VecDeque<i64>,
  offset_base: i64,
  pub output_value: VecDeque<i64>,
}

impl Amplifier {
  pub fn new(program: String, phase_setting: Option<i64>) -> Self {
    Amplifier {
      phase_setting,
      program: program
        .split(",")
        .map(|n| n.parse::<i64>().unwrap())
        .collect::<Vec<i64>>(),
      index: 0,
      input_value: VecDeque::from(phase_setting.into_iter().collect::<Vec<i64>>()),
      offset_base: 0,
      output_value: VecDeque::new(),
    }
  }
  fn get_data_at(self: &Amplifier, index: usize) -> i64 {
    if self.program.len() <= index {
      0
    } else {
      self.program[index]
    }
  }
  fn store_data_at(self: &mut Amplifier, index: usize, value: i64) {
    if self.program.len() <= index {
      self.program.resize(index + 1, 0);
    }
    self.program[index] = value;
  }
  pub fn interprete(self: &mut Amplifier) -> VecDeque<i64> {
    use Command::*;
    use Parameter::*;
    let debug = false;
    let opcode = self.program[self.index];
    let command = code_to_command(opcode);
    let get_by_offset = |mode: &Parameter, offset: i64| match mode {
      Position => self.get_data_at((self.index as i64 + offset) as usize) as usize,
      Immediate => (self.index as i64 + offset) as usize,
      Relative => (self.offset_base + offset) as usize,
    };
    match command {
      Add(mode_a, mode_b, mode_position) => {
        let a = get_by_offset(&mode_a, 1);
        let b = get_by_offset(&mode_b, 2);
        let position = get_by_offset(&mode_position, 3);
        let output = self.get_data_at(a) + self.get_data_at(b);
        if debug {
          println!(
            "[AMP {:?}] ({:4}) {:5?} {:4} {:4} {:4} |      ADD {} + {} (={}) => {:4}",
            self.phase_setting,
            self.index,
            opcode,
            self.get_data_at(self.index + 1),
            self.get_data_at(self.index + 2),
            self.get_data_at(self.index + 3),
            self.get_data_at(a),
            self.get_data_at(b),
            output,
            position
          );
        }
        self.store_data_at(position, output);
        self.index = self.index + 4;
        self.interprete()
      }
      Multiply(mode_a, mode_b, mode_position) => {
        let a = get_by_offset(&mode_a, 1);
        let b = get_by_offset(&mode_b, 2);
        let position = get_by_offset(&mode_position, 3);
        let output = self.get_data_at(a) * self.get_data_at(b);
        if debug {
          println!(
            "[AMP {:?}] ({:4}) {:5?} {:4} {:4} {:4} | MULTIPLY {} * {} (={}) => {:4}",
            self.phase_setting,
            self.index,
            opcode,
            self.get_data_at(self.index + 1),
            self.get_data_at(self.index + 2),
            self.get_data_at(self.index + 3),
            self.get_data_at(a),
            self.get_data_at(b),
            output,
            position,
          );
        }
        self.store_data_at(position, output);
        self.index = self.index + 4;
        self.interprete()
      }
      Input(mode) => {
        let position = get_by_offset(&mode, 1);
        let input = self.input_value.pop_back();
        if debug {
          println!(
            "[AMP {:?}] ({:4}) {:5?} {:4} {:4} {:4} |    INPUT {:4} => {:?}",
            self.phase_setting,
            self.index,
            opcode,
            self.get_data_at(self.index + 1),
            " ",
            " ",
            position,
            input
          );
        }
        if input.is_some() {
          self.store_data_at(position, input.expect("Should have an input value left!"));
          self.index = self.index + 2;
          self.interprete()
        } else {
          self.output_value
        }
      }
      Output(mode) => {
        let position = get_by_offset(&mode, 1);
        let output_value = self.get_data_at(position);
        self.output_value.push_back(output_value);
        println!("OUT: {}", output_value);
        if debug {
          println!(
            "[AMP {:?}] ({:4}) {:5?} {:4} {:4} {:4} |   OUTPUT {:4} => {:?}",
            self.phase_setting,
            self.index,
            opcode,
            self.get_data_at(self.index + 1),
            " ",
            " ",
            position,
            output_value
          );
        }
        self.index = self.index + 2;
        self.interprete()
      }
      JumpIfTrue(mode_a, mode_b) => {
        let test_non_zero = get_by_offset(&mode_a, 1);
        let next_index = get_by_offset(&mode_b, 2);
        let next_position = if self.get_data_at(test_non_zero) != 0 {
          self.get_data_at(next_index) as usize
        } else {
          self.index + 3
        };
        if debug {
          println!(
            "[AMP {:?}] ({:4}) {:5?} {:4} {:4} {:4} |  JMPTRUE {:4} {:4} => {:?}",
            self.phase_setting,
            self.index,
            opcode,
            self.get_data_at(self.index + 1),
            self.get_data_at(self.index + 2),
            self.get_data_at(self.index + 3),
            test_non_zero,
            next_index,
            next_position
          );
        }
        self.index = next_position;
        self.interprete()
      }
      JumpIfFalse(mode_a, mode_b) => {
        let test_zero = get_by_offset(&mode_a, 1);
        let next_index = get_by_offset(&mode_b, 2);
        let next_position = if self.get_data_at(test_zero) == 0 {
          self.get_data_at(next_index) as usize
        } else {
          self.index + 3
        };
        if debug {
          println!(
            "[AMP {:?}] ({:4}) {:5?} {:4} {:4} {:4} | JMPFALSE {:4} {:4} => {:?}",
            self.phase_setting,
            self.index,
            opcode,
            self.get_data_at(self.index + 1),
            self.get_data_at(self.index + 2),
            self.get_data_at(self.index + 3),
            test_zero,
            next_index,
            next_position
          );
        }
        self.index = next_position;
        self.interprete()
      }
      LessThan(mode_a, mode_b, mode_position) => {
        let a = get_by_offset(&mode_a, 1);
        let b = get_by_offset(&mode_b, 2);
        let position = get_by_offset(&mode_position, 3);
        let value_to_store = if self.get_data_at(a) < self.get_data_at(b) {
          1
        } else {
          0
        };
        if debug {
          println!(
            "[AMP {:?}] ({:4}) {:5?} {:4} {:4} {:4} | LESSTHAN {:4} {:4} => {:?}",
            self.phase_setting,
            self.index,
            opcode,
            self.get_data_at(self.index + 1),
            self.get_data_at(self.index + 2),
            self.get_data_at(self.index + 3),
            a,
            b,
            value_to_store
          );
        }
        self.store_data_at(position, value_to_store);
        self.index = self.index + 4;
        self.interprete()
      }
      Equals(mode_a, mode_b, mode_position) => {
        let a = get_by_offset(&mode_a, 1);
        let b = get_by_offset(&mode_b, 2);
        let position = get_by_offset(&mode_position, 3);
        let value_to_store = if self.get_data_at(a) == self.get_data_at(b) {
          1
        } else {
          0
        };
        if debug {
          println!(
            "[AMP {:?}] ({:4}) {:5?} {:4} {:4} {:4} |   EQUALS {:4} {:4} => {:?}",
            self.phase_setting,
            self.index,
            opcode,
            self.get_data_at(self.index + 1),
            self.get_data_at(self.index + 2),
            self.get_data_at(self.index + 3),
            a,
            b,
            value_to_store
          );
        }
        self.store_data_at(position, value_to_store);
        self.index = self.index + 4;
        self.interprete()
      }
      RelativeBaseOffset(mode) => {
        let offset_base = get_by_offset(&mode, 1);
        self.offset_base = offset_base as i64;
        self.index = self.index + 2;
        self.interprete()
      }
      End => {
        if debug {
          println!(
            "[AMP {:?}] ({:4}) {:5?} {:4} {:4} {:4} |      END",
            self.phase_setting, self.index, opcode, " ", " ", " ",
          );
        }
        self.output_value
      }
    }
  }
  pub fn calculate_output(&mut self, input_value: i64) -> VecDeque<i64> {
    self.input_value.push_front(input_value);
    self.interprete()
  }
}

#[derive(Debug)]
enum Parameter {
  Immediate,
  Position,
  Relative,
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
  RelativeBaseOffset(Parameter),
  End,
}

fn code_to_command(opcode: i64) -> Command {
  use Command::*;
  use Parameter::*;

  let operation = opcode % 100;
  let retrieve_param = |n: u32| {
    let mode = (opcode / (10 as i64).pow(n + 1)) % 10;
    if mode == 0 {
      Position
    } else if mode == 1 {
      Immediate
    } else {
      Relative
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
    9 => RelativeBaseOffset(retrieve_param(1)),
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
  use std::collections::VecDeque;

  #[test]
  fn relative_base_offset_bigger_memory() {
    let input = "104,1125899906842624,99";
    let expected_output = Some(1125899906842624);
    assert_eq!(
      expected_output,
      computer_1202(&input.to_owned(), false, &mut VecDeque::new()).output
    );
  }

  #[test]
  fn relative_base_offset_16_digit_number() {
    let input = "1102,34915192,34915192,7,4,7,99,0";
    let result = computer_1202(&input.to_owned(), false, &mut VecDeque::new());
    assert_eq!(
      16,
      format!("{}", result.output.expect("expected an output value")).len()
    );
  }
  #[test]
  fn relative_base_offset_quine() {
    let input = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
    let result = computer_1202(&input.to_owned(), false, &mut VecDeque::new());
    assert_eq!(204, result.output.expect("should have an output"));
    assert_eq!(true, false);
  }

  #[test]
  fn equal_to_8() {
    let input = "3,9,8,9,10,9,4,9,99,-1,8";
    let mut eq_eight = VecDeque::from(vec![8]);
    let mut not_eight_1 = VecDeque::from(vec![9]);
    let mut not_eight_2 = VecDeque::from(vec![5]);
    let mut not_eight_3 = VecDeque::from(vec![10]);
    let mut not_eight_4 = VecDeque::from(vec![0]);

    assert_eq!(
      1,
      computer_1202(&input.to_owned(), false, &mut eq_eight)
        .output
        .expect("should have an output value")
    );
    assert_eq!(
      0,
      computer_1202(&input.to_owned(), false, &mut not_eight_1)
        .output
        .expect("should have an output value")
    );
    assert_eq!(
      0,
      computer_1202(&input.to_owned(), false, &mut not_eight_2)
        .output
        .expect("should have an output value")
    );
    assert_eq!(
      0,
      computer_1202(&input.to_owned(), false, &mut not_eight_3)
        .output
        .expect("should have an output value")
    );
    assert_eq!(
      0,
      computer_1202(&input.to_owned(), false, &mut not_eight_4)
        .output
        .expect("should have an output value")
    );
  }
  #[test]
  fn larger_jump_example() {
    let input = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
    let mut lt_eight = VecDeque::from(vec![7]);
    let mut eq_eight = VecDeque::from(vec![8]);
    let mut gt_eight = VecDeque::from(vec![9]);
    assert_eq!(
      999,
      computer_1202(&input.to_owned(), false, &mut lt_eight)
        .output
        .expect("should have an output value")
    );
    assert_eq!(
      1000,
      computer_1202(&input.to_owned(), false, &mut eq_eight)
        .output
        .expect("should have an output value")
    );
    assert_eq!(
      1001,
      computer_1202(&input.to_owned(), false, &mut gt_eight)
        .output
        .expect("should have an output value")
    );
  }
  #[test]
  fn jump_test_position_mode() {
    let input = "3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9";
    let mut input_parameter = VecDeque::from(vec![0]);
    let expected_output = 0;
    assert_eq!(
      expected_output,
      computer_1202(&input.to_owned(), false, &mut input_parameter)
        .output
        .expect("should have an output value")
    );

    let mut input_parameter = VecDeque::from(vec![1]);
    let expected_output = 1;
    assert_eq!(
      expected_output,
      computer_1202(&input.to_owned(), false, &mut input_parameter)
        .output
        .expect("should have an output value")
    )
  }

  #[test]
  fn jump_test_immediate_mode() {
    let input = "3,3,1105,-1,9,1101,0,0,12,4,12,99,1";
    let mut input_parameter = VecDeque::from(vec![0]);
    let expected_output = 0;
    assert_eq!(
      expected_output,
      computer_1202(&input.to_owned(), false, &mut input_parameter)
        .output
        .expect("should have an output value")
    );

    let mut input_parameter = VecDeque::from(vec![1]);
    let expected_output = 1;
    assert_eq!(
      expected_output,
      computer_1202(&input.to_owned(), false, &mut input_parameter)
        .output
        .expect("should have an output value")
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
        computer_1202(&input.to_owned(), false, &mut VecDeque::new()).state
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
        computer_1202(&input.to_owned(), false, &mut VecDeque::from(vec![123])).state
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
        computer_1202(&input.to_owned(), false, &mut VecDeque::new()).state
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
        computer_1202(&input.to_owned(), false, &mut VecDeque::new()).state
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
        computer_1202(&input.to_owned(), false, &mut VecDeque::new()).state
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
        computer_1202(&input.to_owned(), false, &mut VecDeque::new()).state
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
        computer_1202(&input.to_owned(), false, &mut VecDeque::new()).state
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
        computer_1202(&input.to_owned(), false, &mut VecDeque::new()).state
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
        computer_1202(&input.to_owned(), false, &mut VecDeque::new()).state
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
        computer_1202(&input.to_owned(), false, &mut VecDeque::new()).state
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
        computer_1202(&input.to_owned(), false, &mut VecDeque::new()).state
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
        computer_1202(&input.to_owned(), false, &mut VecDeque::new()).state
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
        computer_1202(&input.to_owned(), false, &mut VecDeque::new()).state
      )
    )
  }

  #[test]
  fn can_fix_input_before_running() {
    let input = "1,1,1,3,99,1,0,1,0,1,0,1,0,1,0,1,0";
    let expected = "[1, 12, 2, 2, 99, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0]";
    assert_eq!(
      expected,
      format!(
        "{:?}",
        computer_1202(&input.to_owned(), true, &mut VecDeque::new()).state
      )
    )
  }
}
