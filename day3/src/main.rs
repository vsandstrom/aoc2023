use regex::Regex;
// get indexes of strings beginning and end

use std::collections::HashSet;

fn parser(path: &str) -> Vec<String> {
  if let Ok(data) = std::fs::read_to_string(path) {
    data.lines().map(|s| s.to_string()).collect()
  } else {
    panic!("no such file")
  }
}

fn task1(data: Vec<String>, re_sym: Regex, re_num: Regex) -> i32 {

  let mut sym: HashSet<(isize, isize)> = HashSet::new();
  let mut v:isize = 0;
  let mut sum = 0;
  for line in data.clone() {
    let m = re_sym.find_iter(&line);
    for x in m {
      (v, x.start());
      sym.insert((v, x.start() as isize));
    }
    v+=1;
  }

  for (v, h) in &sym {
    println!("{} :: {}", v, h);
  }

  v=0;
  for line in data.clone() {
    let n = re_num.find_iter(&line);
    'search: for num in n {
      let start = num.start() as isize - 1;
      let end = num.end() as isize + 1;
      let mut val = 0;
      for h in start..end {
        if sym.contains(&(v+1, h)) {
          val = num.as_str().parse::<i32>().unwrap();
          println!("below {}", val);
          sum += val;
          continue 'search;
        }
      }

      for h in start..end {
        if sym.contains(&(v-1, h)) {
          val = num.as_str().parse::<i32>().unwrap();
          println!("above {}", val);
          sum += val;
          continue 'search;
        }
      }
        
      if sym.contains(&(v, start)) {
        val = num.as_str().parse::<i32>().unwrap();
        println!("at start {}", val);
        sum += val;
        continue 'search;
      }

      if sym.contains(&(v, end-1)) {
        val = num.as_str().parse::<i32>().unwrap();
        println!("at end {}", val);
        sum += val;
        continue 'search;
      }
    }
    v+=1;
    
  //   'search: for num in n {
  //       for hor in ((num.start() as isize)-1)..=((num.end() as isize)+1) {
  //         println!("{}-{} :: {}-{} :: {}-{} ", v-1, hor, v,hor, v+1, hor);
  //         if sym.contains(&(v, hor)) || 
  //            sym.contains(&(v+1, hor)) || 
  //            sym.contains(&(v-1, hor)) {
  //           println!("--> {}-{}", v, hor);
  //           sum += &line[num.start()..num.end()].parse::<i32>().unwrap();
  //         }
  //
  //       }
  //
  //     }
  //   v+=1;
  }
  sum
}



fn main() {
  let re = Regex::new(r"[^0-9.]").unwrap();
  let rf = Regex::new(r"[0-9]+").unwrap();
  let data = parser("./input");
  let sum = task1(data, re, rf);
  println!("4361 == sum = {}", sum);
}
