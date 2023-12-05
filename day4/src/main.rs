use std::collections::HashSet;

fn parser(path: &str) -> Vec<Vec<String>> {
  if let Ok(data) = std::fs::read_to_string(path) {
    return data.lines().map(|s| s.split(&[':', '|']).map(|s| s.to_string()).collect::<Vec<String>>()).collect::<Vec<Vec<String>>>();
  } else {
    panic!()
  }
}

fn task1(data: Vec<Vec<String>>) -> i32 {
  let mut sum = 0;
  for card in data {
    let mut hash: HashSet<i32> = HashSet::new();
    for num in card[1].trim().split(" ").filter(|s| !s.is_empty()).map(|n| n.parse::<i32>().unwrap()) 
        //.collect::<Vec<i32>>());
    {
      hash.insert(num);
    }

    let mut  i = 0;

    for num in 
      card[2].trim().split(" ").filter(|s| !s.is_empty()).map(|n| n.parse::<i32>().unwrap()) 
        //.collect::<Vec<i32>>())
    {
      if hash.contains(&num) {
        if i == 0 { i = 1 }
        else { i += i }
      }
    }
    sum += i;
  }

  sum
}

fn main() {
  let data = parser("./input");
  println!("{}", task1(data));
}
