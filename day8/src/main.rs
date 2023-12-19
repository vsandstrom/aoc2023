use std::{collections::HashMap, sync::Arc, clone};

struct CrossRoads {
  left: String,
  right: String
}


struct Input {
  instructions: Vec<u8>,
  paths: HashMap<String, CrossRoads>
}

fn main() {
  let input = parser("./input");
  println!("{}", task1(input));
}

fn task1(input: Input) -> usize {
  let mut i: usize = 0;
  // let mut traversed = vec!();
  let mut pos = match input.paths.get_key_value("AAA") {
    Some(val) => val,
    _ => panic!()
  };

  
  loop {
    if pos.0 == "ZZZ" {
      break;
    } else {
      match input.instructions[i% input.instructions.len()] {
        // LEFT
        0 => {
          pos = match input.paths.get_key_value(&pos.1.left) {
            Some(val) => val,
            _ => panic!()
          };
          i+=1;
        },
        // RIGHT
        1 => {
          pos = match input.paths.get_key_value(&pos.1.right) {
            Some(val) => val,
            _ => panic!()
          };
          i+=1;
        },
      _ => panic!() 
      }
    }
  }
  i 
}

fn parser(path: &str) -> Input {
  let mut hs: HashMap<String, CrossRoads> = HashMap::new();
  let input = std::fs::read_to_string(path).unwrap();
  let lines = input.lines();
  let instr = lines.clone().next().unwrap().chars().map(|c| match c {
      x if x == 'R' => { 1 },
      x if x == 'L' => { 0 },
      _ => panic!()
    }).collect::<Vec<u8>>();

  let x = lines
    .skip(1)
    .map(|l| 
      l.split(&[' ', '=', ',', '(', ')'])
      .filter_map(|x| match !x.is_empty() {
        true => { Some(x.to_string())},
        _ => None 
        }
      ).collect()
    ).collect::<Vec<Vec<String>>>();

  for row in x.into_iter().skip(1) {
    hs.insert(row[0].to_string(), CrossRoads{left: row[1].to_owned(), right: row[2].to_owned()});
  }
  Input{instructions: instr, paths: hs}
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test1() {
    let input = parser("./test");
    assert_eq!(2, task1(input));
  }

  #[test]
  fn test1_2() {

    let input = parser("./test2");
    assert_eq!(6, task1(input));
  }
}
