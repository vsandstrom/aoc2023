use std::{collections::HashMap, vec};

// index + 1 är spelets id

fn task1(path: &str, limit: HashMap<&str, i32>) -> usize {
  let mut sum = 0;
  let data = std::fs::read_to_string(path).unwrap();
  let parsed: Vec<String> = data.lines().map(|s| s.to_string()).collect();

  for (i, line) in parsed.iter().enumerate() {
    let mut valid = true;
    let s: Vec<String> = line.split(&[':',';']).map(|s|s.to_string()).collect();
    let id:i32 = s[0].to_owned().split(' ').last().unwrap().parse().unwrap();
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


fn main() {
  let limit = HashMap::from([
    ("red", 12), 
    ("green", 13), 
    ("blue", 14)
  ]);

  println!("{}", task1("./input", limit));
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

    assert_eq!(8, task1("./test", limit));


  }
}
