fn main() {
  let input = parser("./input");
  println!("{}", task1(input))

}

fn parser(path: &str) -> Vec<i32> {
  std::fs::read_to_string(path)
    .unwrap()
    .split(&['\n', ' '])
    .filter_map(|s| match s.parse::<i32>() {
      Ok(n) => Some(n),
      _ => None
    }
  ).collect::<Vec<i32>>()
}

fn task1(input: Vec<i32>) -> i64 {
  let mut sum: i64 = 1;
  'outer: for n in 0..(input.len()/2) {
    let ms = input[n];
    let wr = input[n+(input.len()/2)];
    let mut i = 0;
    'inner: loop{
      if i >= ms/2 {
        sum *= (ms/2) as i64;
        continue 'outer;
      }
      let val = i * (ms - i);
      let nr = val;
      if nr > wr {
        break 'inner;
      }
      i += 1;
    }
    sum *= (ms - (i*2 - 1)) as i64;
  }
  sum
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn math() {
    let input = parser("./test");
    let mut sum = 1;
    'outer: for n in 0..3 {
      let ms = input[n];
      let wr = input[n+3];
      let mut nr = 0;
      let mut i = 0;
      loop{
        if i > ms / 2 {
          sum = 1;
          continue 'outer;
        }
        let val = i * (ms - i);
        nr = val;
        if nr > wr {
          break;
        }
        i += 1;
      }
      sum *= ms - (i*2 - 1);
      // if ms % 2 == 1 {
      //   println!(" hello {}", ms - (i*2 - 1));
      // } else {
      //   println!(" hello {}", ms - (i*2));
      // }
    }
    assert_eq!(288, sum);
  }

  #[test]
  fn task1_test() {
    let input = parser("./test");
    assert_eq!(288, task1(input));
  }
}
