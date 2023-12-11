use std::{iter::Map, str::Split, collections::HashMap};


fn main() {
  let data = parser("./input");
  let mut seeds = data[0].to_owned().split(&[' ', '\n', ':'])
    .filter_map(|s| match s.parse::<i64>() {
    Ok(num) => Some(num),
    Err(_) => None
  }).collect::<Vec<i64>>();
  let mut conversions: Vec<HashMap<i64, i64>> = vec!();

  let conv = data[1..].to_owned();
  for row in conv {
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

    let mut hs = HashMap::new();
    for range in x.chunks(3) {
      let des = range[0];
      let ori = range[1];
      let len = range[2];
      for i in 0..=len {
        hs.insert(ori + i, des + i);
      }
    }
    conversions.push(hs);
  }

  for table in conversions {
    for i in 0..(seeds.len()) {

      if let Some(v) = table.get(&seeds[i]) {
        println!("HERE");
        seeds[i] = *v;
      }
    }
  }

  let mut ans = i64::MAX;
  for seed in seeds {
    if seed < ans {
      ans = seed;
    }
  }
  println!("{:?}", ans);
}

fn parser(path: &str) -> Vec<String> {
  if let Ok(data) = std::fs::read_to_string(path) {
    return data.split("\n\n").map(|s| s.to_string()).collect::<Vec<String>>();
  }
  panic!();
}


