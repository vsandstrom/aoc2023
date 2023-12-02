use std::{collections::HashMap, vec, u64};

// index + 1 är spelets id
//
fn parser(path: &str) -> Vec<String>{
  let data = std::fs::read_to_string(path).unwrap();
  data.lines().map(|s| s.to_string()).collect::<Vec<String>>()
}

fn task1(parsed: Vec<String>, limit: HashMap<&str, i32>) -> usize {
  let mut sum = 0;

  for (i, line) in parsed.iter().enumerate() {
    let mut valid = true;
    let s: Vec<String> = line.split(&[':',';']).map(|s|s.to_string()).collect();
    'hands: for hand in &s[1..] {
      let vals = hand.split(&[' ', ',']).filter(|s| !s.is_empty()).collect::<Vec<&str>>();
      for x in vals.chunks(2) {
        if let Some(lim) = limit.get(x[1]) {
          if x[0].parse::<i32>().unwrap() > *lim {
            valid = false;
            break 'hands;
          }
        }
      }
    }
    if valid {
      sum += i + 1;
    }
  }
  sum 

}



fn task2(parsed: Vec<String>) -> u64 {
  // red | green | blue
  let mut min: [u64; 3] = [0, 0, 0];
  let mut sum: u64 = 0;
  for line in parsed {
    let s: Vec<String> = line.split(&[':',';']).map(|s|s.to_string()).collect();
    for hand in &s[1..] { 
      let vals = hand.split(&[' ', ',']).filter(|s| !s.is_empty()).collect::<Vec<&str>>();
      for x in vals.chunks(2) {
        let num = x[0].parse::<u64>().unwrap();
        match x[1] {
          x if x == "red" => {
            if min[0] < num {
              min[0] = num;
            }
          },
          x if x == "green" => {
            if min[1] < num {
              min[1] = num;
            }
          }, x if x == "blue" => {
            if min[2] < num {
              min[2] = num;
            }
          }, 
          _ => panic!("ew!!!")

        }
      }

    }
    sum += min[0] * min[1] * min[2];
    min[0] = 0;
    min[1] = 0;
    min[2] = 0;
  }
  sum
}


fn main() {
  let limit = HashMap::from([
    ("red", 12), 
    ("green", 13), 
    ("blue", 14)
  ]);

  let parsed = parser("./input");

  println!("{}", task1(parsed.clone(), limit));
  println!("{}", task2(parsed));
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
    fn test1() {
    let limit = HashMap::from([
      ("red", 12), 
      ("green", 13), 
      ("blue", 14)
    ]);
    let parsed = parser("./test");
    assert_eq!(8, task1(parsed, limit))
  }
  #[test]
    fn test2() {
      let parsed = parser("./test");
      assert_eq!(2286, task2(parsed))
    }
}
