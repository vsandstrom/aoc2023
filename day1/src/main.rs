
use std::{fs::{read, self}, io::BufRead, collections::{HashSet, HashMap}};
use regex::Regex;

fn parser(data: String) -> Vec<String> {
  data.lines().map(|s| s.to_string()).collect()
}
  
fn task1(input: Vec<String>) -> i32 {
  let mut sum = 0;
  for s in input {
    let ch = s.chars();
    for c in ch.clone() {
      if c.is_digit(10) {
        sum += (c as i32 - '0' as i32) * 10;
        break;
      }
    }

    for c in ch.rev() {
      if c.is_digit(10) {
        sum += c as i32 - '0' as i32;
        break;
      }
    }
  }
  sum
}


fn task2(input: Vec<String>) -> i32 {
  let hm =  HashMap::from([
    ("1", 1),
    ("one", 1),
    ("2", 2),
    ("two", 2),
    ("3", 3),
    ("three", 3),
    ("4", 4),
    ("four", 4),
    ("5", 5), 
    ("five", 5), 
    ("6", 6),
    ("six", 6),
    ("7", 7),
    ("seven", 7),
    ("8", 8),
    ("eight", 8),
    ("9", 9),
    ("nine", 9),
  ]);

  let mut sum = 0;

  let re = match Regex::new(r"one|two|three|four|five|six|seven|eight|nine|[0-9]") {
    Ok(re) => re,
    Err(e) => panic!("{}", e)
  };
  
  for line in input {
    for i in 0..line.len()+1 {
      if let Some(word) = re.find(&line[0..i]) {
        sum += hm.get(&word.as_str()).unwrap() * 10;
        break;
      }
    }

    for i in 0..line.len()+1 {
      let l = line.len();
      if let Some(word) = re.find(&line[l-i..l]) {
        sum+= hm.get(&word.as_str()).unwrap();
        break;
      }
    }
  }
  sum
}

fn main() {
    let data = fs::read_to_string("./input").unwrap();
    let input = parser(data);
    let sum = task1(input.clone());
    println!("task1: {}", sum);
    
    let x: Vec<String> = Vec::from([
      "two1nine".to_string(),
      "eightwothree".to_string(),
      "abcone2threexyz".to_string(),
      "xtwone3four".to_string(),
      "4nineeightseven2".to_string(),
      "zoneight234".to_string(),
      "7pqrstsixteen".to_string()
    ]);

    println!("task2: {}", task2(input));
}

#[allow(unused)]
fn task2_notworking(mut input: Vec<String>) -> i32 {
  let hm =  HashMap::from([
    ("1", 1),
    ("one", 1),
    ("2", 2),
    ("two", 2),
    ("3", 3),
    ("three", 3),
    ("4", 4),
    ("four", 4),
    ("5", 5), 
    ("five", 5), 
    ("6", 6),
    ("six", 6),
    ("7", 7),
    ("seven", 7),
    ("8", 8),
    ("eight", 8),
    ("9", 9),
    ("nine", 9),
  ]);

  let mut idx: Vec<(usize, usize)> = vec!();
  let mut sum = 0;

  for line in &input {
    for k in hm.keys() {
      if let Some(val) = line.find(k) {
        let offset = val + k.len();
        idx.push((val, offset)); 
      } 
    }
    idx.sort_by_key(|x| x.0);
    let (start, end) = idx[0];
    if let Some(val) = hm.get(&line[start..end]) {
      sum += (val * 10);
    };
    let (start, end) = idx[idx.len()-1];
    if let Some(val) = hm.get(&line[start..end]) {
      sum += val;
    };
    idx.clear();
  }
  sum
}

#[cfg(test)]

mod tests {
  use super::*;

  #[test]
  fn replace() {
    let hs =  HashMap::from([
      ("one", "1"),
      ("two", "2"),
      ("three", "3"),
      ("four", "4"),
      ("five", "5"), 
      ("six", "6"),
      ("seven", "7"),
      ("eight", "8"),
      ("nine", "9")
    ]);
    let mut x = "hello1nineeighthello".to_string();
    for (k, v) in &hs {
      x = x.replace(k, v);
    }

    assert_eq!("hello198hello".to_string(), x);
  }

  #[test]
  fn sum_task1() {
    let hs =  HashMap::from([
      ("1", 1),
      ("one", 1),
      ("2", 2),
      ("two", 2),
      ("3", 3),
      ("three", 3),
      ("4", 4),
      ("four", 4),
      ("5", 5), 
      ("five", 5), 
      ("6", 6),
      ("six", 6),
      ("7", 7),
      ("seven", 7),
      ("8", 8),
      ("eight", 8),
      ("9", 9),
      ("nine", 9),
    ]);

    let mut x: Vec<String> = Vec::from([
      "two1nine".to_string(),
      "eightwothree".to_string(),
      "abcone2threexyz".to_string(),
      "xtwone3four".to_string(),
      "4nineeightseven2".to_string(),
      "zoneight234".to_string(),
      "7pqrstsixteen".to_string()]);

    let mut idx: Vec<(usize, usize)> = vec!();
    let mut sum = 0;

    for s in x {
      for k in hs.keys() {
        if let Some(val) = s.find(k) {
          let offset = val + k.len();
          
          idx.push((val, offset)); 
        } 
      } 

      idx.sort_by_key(|x| x.0);

      let (start, end) = idx[0];
      if let Some(val) = hs.get(&s[start..end]) {
        sum += val * 10;
      };
      let (start, end) = idx[idx.len()-1];
      if let Some(val) = hs.get(&s[start..end]) {
        sum += val;
      };
      idx.clear();
    } 
    assert_eq!(281, sum)
  }

  #[test]
  fn test_task2() {
    let x: Vec<String> = Vec::from([
      "two1nine".to_string(),
      "eightwothree".to_string(),
      "abcone2threexyz".to_string(),
      "xtwone3four".to_string(),
      "4nineeightseven2".to_string(),
      "zoneight234".to_string(),
      "7pqrstsixteen".to_string()
    ]);

    let sum = task2(x);
    assert_eq!(281, sum)
  }
}


