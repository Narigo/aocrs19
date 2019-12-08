use crate::intcode_compute::{computer_1202, interprete};
use std::collections::HashMap;
use std::fs;

pub fn orbit_checksum() -> u32 {
  check_orbits("./src/aoc06/input.txt")
}

pub fn orbit_transfers() -> u32 {
  check_transfer_orbits("./src/aoc06/input.txt")
}

type OrbitMap<'a> = HashMap<String, Vec<String>>;

pub fn check_transfer_orbits(filename: &str) -> u32 {
  let orbit_map = generate_orbit_map(filename);
  calculate_orbit_transfers(&orbit_map, &"YOU".to_owned(), &"SAN".to_owned())
}

pub fn check_orbits(filename: &str) -> u32 {
  let orbit_map = generate_orbit_map(filename);
  calculate_orbit_checksum(&orbit_map)
}

fn generate_orbit_map(filename: &str) -> OrbitMap {
  let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
  let orbit_lines = contents.lines();
  let mut orbit_map = OrbitMap::new();
  for orbit in orbit_lines {
    let mut splitted = orbit.split(")");
    let left = splitted.next().unwrap().to_string();
    let right = splitted.next().unwrap().to_string();
    let possible_values = orbit_map.get(&left);
    let mut v = match possible_values {
      Some(orbits) => orbits.clone(),
      None => Vec::new(),
    };
    v.push(right);
    orbit_map.insert(left, v);
  }
  orbit_map
}

fn calculate_orbit_checksum(orbit_map: &OrbitMap) -> u32 {
  calculate_orbits_of_child(orbit_map, &"COM".to_string(), 0)
}

fn calculate_orbit_transfers(orbit_map: &OrbitMap, from: &String, to: &String) -> u32 {
  let mut parents_from = get_parents_of_object(orbit_map, from);
  let mut parents_to = get_parents_of_object(orbit_map, to);
  while parents_from.pop() == parents_to.pop() {}
  (parents_from.len() + parents_to.len() + 2) as u32
}

fn get_parents_of_object(orbit_map: &OrbitMap, o: &String) -> Vec<String> {
  let mut my_parents = vec![];
  let mut my_parent = find_parent(orbit_map, o);
  while my_parent.is_some() {
    let parent = my_parent.unwrap();
    my_parents.push(parent.clone());
    my_parent = find_parent(orbit_map, &parent);
  }
  my_parents
}

fn find_parent(orbit_map: &OrbitMap, o: &String) -> Option<String> {
  for possible_parent in orbit_map.keys() {
    for object in orbit_map.get(possible_parent).unwrap_or(&vec![]) {
      if object == o {
        return Some(possible_parent.to_string());
      }
    }
  }
  None
}

fn calculate_orbits_of_child(orbit_map: &OrbitMap, child: &String, orbits_of_parent: u32) -> u32 {
  let mut sum = orbits_of_parent;
  if orbit_map.contains_key(child) {
    let children = orbit_map.get(child).unwrap();
    for orbiting in children {
      sum = sum + calculate_orbits_of_child(orbit_map, orbiting, orbits_of_parent + 1);
    }
  }
  sum
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn orbiting_example_transfers() {
    let checksum = check_transfer_orbits("./src/aoc06/example_02.txt");
    assert_eq!(4, checksum);
  }

  #[test]
  fn orbiting_example_checksum() {
    let checksum = check_orbits("./src/aoc06/example.txt");
    assert_eq!(42, checksum);
  }
}
