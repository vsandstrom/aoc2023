use std::{iter::Map, str::Split, collections::HashMap};


fn main() {
  let data = parser("./input");
  let mut seeds = parse_seeds(&data[0].to_owned());
  let conv = parse_conversions(data[1..].to_owned());
    
  for table in conv {
    for range in table.chunks(3) {
      let ori = range[1]..(range[1] + range[2]);
      for i in 0..seeds.len() {
        if ori.contains(&seeds[i]) {
          seeds[i] = range[0] + seeds[i] - range[1];
        }
      }
    }
  }

  let mut ans = i64::MAX;
  for num in seeds {
    if num < ans {
      ans = num;
    }
  }
  println!("{}", ans);

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

    // for range in x.chunks(3) {
    //   println!("{}", range[0] - range[1]);
    // }
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
    let conv = parse_conversions(data[1..].to_owned());
    for table in conv {
      for range in table.chunks(3) {
        let ori = range[1]..(range[1] + range[2]);
        println!("{:?}", ori);
        for i in 0..seeds.len() {
          if ori.contains(&seeds[i]) {
            let offset = range[0] + seeds[i] - range[1];
            println!("{}", offset);
            seeds[i] = offset;
          }
          println!("{:?}", &seeds);
        }
      }
    }
    seeds.sort_unstable();
    assert_eq!(35, seeds[0]);


  }
}
