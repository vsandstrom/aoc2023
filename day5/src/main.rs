use std::{iter::Map, str::Split, collections::HashMap};


fn main() {
  let data = parser("./input");
  let mut seeds = parse_seeds(&data[0].to_owned());
  let conv = parse_conversions(data[1..].to_owned());


  println!("seedslen: {}", seeds.len());
  println!("{}", task1(seeds.clone(), conv.clone()));



  let mut seeds2 = vec!();

  for seedrange in seeds.chunks(2) {
    for i in 0..seedrange[1] {
      seeds2.push(seedrange[0]+i);
    }
  }
  println!("seeds2len: {}", seeds2.len());
  println!("{}", task1(seeds2, conv));
}



fn task1(seeds: Vec<i64>, conv: Vec<Vec<i64>>) -> i64 {
  let mut transform: Vec<usize> = Vec::with_capacity(seeds.len());
  for _ in &seeds {
    transform.push(0);
  }

  let mut seeds = seeds;

  for table in conv {
    for range in table.chunks(3) {
      let ori = range[1]..(range[1] + range[2]);
      for i in 0..seeds.len() {
        if ori.contains(&seeds[i]) && transform[i] == 0 {
          let offset = range[0] + seeds[i] - range[1];
          seeds[i] = offset;
          transform[i] = 1;
        }
      }
    }

    for i in 0..transform.len() {
      transform[i] = 0;
    }
  }

  let mut ans = i64::MAX;
  for num in seeds {
    if num < ans {
      ans = num;
    }
  }
  ans
}

fn parse_seeds(data: &String) -> Vec<i64> {
  data.split(&[' ', '\n', ':'])
    .filter_map(|s| match s.parse::<i64>() {
    Ok(num) => Some(num),
    Err(_) => None
  }).collect::<Vec<i64>>()
}

fn parse_conversions(data: Vec<String>) -> Vec<Vec<i64>> {
  let mut conv_tables = vec!();
  for row in data {
    let x = row.split(&['\n', ' '])
      .filter_map(|s| match !s.is_empty() {
        true => {
          match s.parse::<i64>() {
            Ok(n) => Some(n),
            _ => None,
          }
        },
        false => None
      }).collect::<Vec<i64>>();
    conv_tables.push(x.clone());
  }
  conv_tables
}

fn parser(path: &str) -> Vec<String> {
  if let Ok(data) = std::fs::read_to_string(path) {
    return data.split("\n\n").map(|s| s.to_string()).collect::<Vec<String>>();
  }
  panic!();
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn transform() {
    let data = parser("./test");
    let mut seeds = parse_seeds(&data[0].to_owned());
    let conv = parse_conversions(data[1..].to_owned());

    for range in conv[0].chunks(3) {
      let ori = range[1]..(range[1] + range[2]);
      for i in 0..seeds.len() {
        if ori.contains(&seeds[i]) {
          seeds[i] = range[0] + seeds[i] - range[1];
        }
      }
    }
    assert_eq!(vec![81, 14, 57, 13], seeds);
  }

  #[test]
  fn result() {
    let data = parser("./test");
    let mut seeds = parse_seeds(&data[0].to_owned());
    let mut transform: Vec<usize> = Vec::with_capacity(seeds.len());
    for _ in &seeds {
      transform.push(0);
    }

    let conv = parse_conversions(data[1..].to_owned());
    for table in conv {
      for range in table.chunks(3) {
        let ori = range[1]..(range[1] + range[2]);
        println!("{:?}", ori);
        for i in 0..seeds.len() {
          if ori.contains(&seeds[i]) && transform[i] == 0 {
            let offset = range[0] + seeds[i] - range[1];
            println!("{}", offset);
            seeds[i] = offset;
            transform[i] = 1;
          }
          println!("{:?}", &seeds);
        }
      }
      for i in 0..transform.len() {
        transform[i] = 0;
      }
    }
    seeds.sort_unstable();
    assert_eq!(35, seeds[0]);
  }

  #[test]
  fn task1_test() {
    let data = parser("./test");
    let mut seeds = parse_seeds(&data[0].to_owned());
    let conv = parse_conversions(data[1..].to_owned());

    assert_eq!(35, task1(seeds, conv));
  }

  #[test]
  fn task2_test() {
    let data = parser("./test");
    let mut seeds = parse_seeds(&data[0].to_owned());
    let conv = parse_conversions(data[1..].to_owned());

    let mut seeds2 = vec!();

    for seedrange in seeds.chunks(2) {
      for i in 0..seedrange[1] {
        seeds2.push(seedrange[0]+i);
      }
    }

    assert_eq!(46, task1(seeds2, conv));

  }
}
