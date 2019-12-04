use std::fs;

pub fn compute_crossing() -> i64 {
  let filename = "./src/aoc03/input.txt";
  let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
  let mut lines = contents.lines();
  let a = lines.next().unwrap();
  let b = lines.next().unwrap();
  compute_nearest_crossing(&a.to_owned(), &b.to_owned())
}

pub fn find_lowest_amount_of_steps() -> i64 {
  let filename = "./src/aoc03/input.txt";
  let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
  let mut lines = contents.lines();
  let a = lines.next().unwrap();
  let b = lines.next().unwrap();
  let cable_a = command_string_to_cable(&a.to_owned());
  let cable_b = command_string_to_cable(&b.to_owned());
  find_lowest_steps(cable_a, cable_b)
}

fn compute_nearest_crossing(first: &String, second: &String) -> i64 {
  let start = (0, 0);
  let cable_a = first
    .split(",")
    .into_iter()
    .map(|string_command| into_command(&string_command.to_owned()))
    .fold(vec![start], |mut cable, command| wire(&mut cable, command));
  let cable_b = second
    .split(",")
    .into_iter()
    .map(|string_command| into_command(&string_command.to_owned()))
    .fold(vec![start], |mut cable, command| wire(&mut cable, command));
  let crossings = retrieve_crossings(cable_a, cable_b);
  find_shortest_distance(start, crossings)
}

fn wire(wire: &mut Vec<Coords>, next: Command) -> Vec<Coords> {
  let start_coords = wire.last().unwrap();
  let mut next_line = into_line(*start_coords, next);
  wire.append(&mut next_line);
  wire.to_vec()
}

fn into_line(last: Coords, command: Command) -> Vec<Coords> {
  let mut v = Vec::new();
  match (last, command) {
    ((x, y), Command::Down(n)) => {
      for i in 1..n + 1 {
        v.push((x, y - i))
      }
    }
    ((x, y), Command::Left(n)) => {
      for i in 1..n + 1 {
        v.push((x - i, y))
      }
    }
    ((x, y), Command::Right(n)) => {
      for i in 1..n + 1 {
        v.push((x + i, y))
      }
    }
    ((x, y), Command::Up(n)) => {
      for i in 1..n + 1 {
        v.push((x, y + i))
      }
    }
  }
  v
}

type Coords = (i64, i64);

type Cable = Vec<Coords>;

type CoordsList = Vec<Coords>;

#[derive(Debug, PartialEq)]
enum Command {
  Down(i64),
  Left(i64),
  Right(i64),
  Up(i64),
}

fn into_command(command: &String) -> Command {
  match command.chars().nth(0) {
    Some('D') => Command::Down(command[1..].parse::<i64>().unwrap()),
    Some('L') => Command::Left(command[1..].parse::<i64>().unwrap()),
    Some('R') => Command::Right(command[1..].parse::<i64>().unwrap()),
    Some('U') => Command::Up(command[1..].parse::<i64>().unwrap()),
    _ => panic!("should not happen!"),
  }
}

fn retrieve_crossings(first: Cable, second: Cable) -> CoordsList {
  let start = first.first().unwrap();
  println!(
    "retrieving crossings first={} second={}",
    first.len(),
    second.len()
  );
  let mut run = 1;
  let length = second.len();
  second
    .into_iter()
    .filter(|coord| {
      run = run + 1;
      if run % 500 == 0 {
        println!("running run {} / {}", run, length);
      }
      first
        .iter()
        .find(|elem| *elem == coord && *elem != start)
        .is_some()
    })
    .collect()
}

fn manhattan_distance_of(a: Coords, b: Coords) -> i64 {
  let (x0, y0) = a;
  let (x1, y1) = b;
  (x0 - x1).abs() + (y0 - y1).abs()
}

fn find_shortest_distance(start: Coords, list: CoordsList) -> i64 {
  list.into_iter().fold(std::i64::MAX, |min_dist, coord| {
    let dist = manhattan_distance_of(start, coord);
    if dist > min_dist {
      min_dist
    } else {
      dist
    }
  })
}

fn command_string_to_cable(commands: &str) -> Cable {
  commands
    .split(",")
    .into_iter()
    .map(|string_command| into_command(&string_command.to_owned()))
    .fold(vec![(0, 0)], |mut cable, command| wire(&mut cable, command))
}

fn find_lowest_steps(cable_a: Cable, cable_b: Cable) -> i64 {
  let length = cable_a.len();
  let start = cable_a.first().unwrap();
  let mut step = -1;
  let second = cable_b.iter().map(|coord| {
    step = step + 1;
    (coord, step)
  });

  let mut min_steps = std::i64::MAX;
  let mut step = 0;
  let mut run = 0;
  for coord in cable_a.iter() {
    run = run + 1;
    if run % 500 == 0 {
      println!("running run {} / {}", run, length);
    }

    if step > min_steps {
      break;
    }

    let mut more_steps = 0;
    for other in cable_b.iter() {
      let necessary_steps = step + more_steps;
      if (*coord == *other) && necessary_steps > 0 && (min_steps > necessary_steps) {
        min_steps = necessary_steps;
        break;
      }
      more_steps = more_steps + 1;
    }
    step = step + 1;
  }
  min_steps
}

#[cfg(test)]
mod test {
  use super::*;

  mod part_2 {
    use super::*;

    #[test]
    fn find_lowest_steps_example_00() {
      let a = command_string_to_cable("R8,U5,L5,D3");
      let b = command_string_to_cable("U7,R6,D4,L4");
      let result = find_lowest_steps(a, b);
      assert_eq!(30, result);
    }

    #[test]
    fn find_lowest_steps_example_01() {
      let a = command_string_to_cable("R75,D30,R83,U83,L12,D49,R71,U7,L72");
      let b = command_string_to_cable("U62,R66,U55,R34,D71,R55,D58,R83");
      let result = find_lowest_steps(a, b);
      assert_eq!(610, result);
    }
  }

  mod part_1 {
    use super::*;
    #[test]
    fn example_01() {
      let dist = compute_nearest_crossing(
        &"R75,D30,R83,U83,L12,D49,R71,U7,L72".to_owned(),
        &"U62,R66,U55,R34,D71,R55,D58,R83".to_owned(),
      );
      assert_eq!(159, dist);
    }

    #[test]
    fn example_02() {
      let dist = compute_nearest_crossing(
        &"R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51".to_owned(),
        &"U98,R91,D20,R16,D67,R40,U7,R15,U6,R7".to_owned(),
      );
      assert_eq!(135, dist);
    }
  }

  mod manhattan_distance_of {
    use super::*;

    #[test]
    fn calculates_correctly() {
      let result = manhattan_distance_of((10, 10), (13, 13));
      assert_eq!(6, result);
    }
  }

  mod retrieve_crossings {
    use super::*;

    fn generate_cable_a() -> Cable {
      let mut cable: Cable = Vec::new();
      cable.push((10, 10));
      wire(&mut cable, Command::Right(1));
      wire(&mut cable, Command::Up(3));
      cable
    }

    fn generate_cable_b() -> Cable {
      let mut cable: Cable = Vec::new();
      cable.push((10, 10));
      wire(&mut cable, Command::Up(1));
      wire(&mut cable, Command::Right(2));
      wire(&mut cable, Command::Up(1));
      wire(&mut cable, Command::Left(2));
      wire(&mut cable, Command::Up(1));
      wire(&mut cable, Command::Right(2));
      cable
    }

    #[test]
    fn finds_crossings() {
      let a = generate_cable_a();
      let b = generate_cable_b();

      let crossings = retrieve_crossings(a, b);
      assert_eq!(vec![(11, 11), (11, 12), (11, 13)], crossings);
    }
  }

  mod wire {
    use super::*;
    #[test]
    fn generates_long_line_through_commands() {
      let mut cable: Cable = Vec::new();
      cable.push((10, 10));
      wire(&mut cable, Command::Right(3));
      wire(&mut cable, Command::Down(2));
      wire(&mut cable, Command::Left(4));
      wire(&mut cable, Command::Up(1));
      assert_eq!(
        vec![
          (10, 10),
          (11, 10),
          (12, 10),
          (13, 10),
          (13, 9),
          (13, 8),
          (12, 8),
          (11, 8),
          (10, 8),
          (9, 8),
          (9, 9)
        ],
        cable
      );
    }
  }

  mod turn_string_into_command {
    use super::*;

    #[test]
    fn one_chars() {
      assert_eq!(Command::Down(1), into_command(&"D1".to_owned()));
      assert_eq!(Command::Left(1), into_command(&"L1".to_owned()));
      assert_eq!(Command::Right(1), into_command(&"R1".to_owned()));
      assert_eq!(Command::Up(1), into_command(&"U1".to_owned()));
    }

    #[test]
    fn multiple_chars() {
      assert_eq!(Command::Down(231), into_command(&"D231".to_owned()));
      assert_eq!(Command::Left(1005), into_command(&"L1005".to_owned()));
      assert_eq!(Command::Right(15), into_command(&"R15".to_owned()));
      assert_eq!(Command::Up(11), into_command(&"U11".to_owned()));
    }
  }

  mod turn_string_into_coords {
    use super::*;

    #[test]
    fn right_counts_plus_on_x() {
      let coords = into_line((0, 0), Command::Right(3));
      assert_eq!(vec!((1, 0), (2, 0), (3, 0)), coords);
    }

    #[test]
    fn down_counts_minus_on_y() {
      let coords = into_line((10, 10), Command::Down(3));
      assert_eq!(vec!((10, 9), (10, 8), (10, 7)), coords);
    }
  }

  #[test]
  fn example_in_text() {
    let first = "R8,U5,L5,D3";
    let second = "U7,R6,D4,L4";
    let result = compute_nearest_crossing(&first.to_owned(), &second.to_owned());
    assert_eq!(result, 6);
  }
}
