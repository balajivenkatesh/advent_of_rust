#![allow(dead_code)]

const BASE_PATH: &str = "D:/rust-projects/advent_of_rust/src/data";
pub fn parse_p11() -> Vec<i32> {
  let filename = crate::p1::BASE_PATH.to_owned() + "/p1.txt";
  // println!("filename = {}", filename);

  let contents = std::fs::read_to_string(filename).expect("Somenthing went wrong reading file");
  let depths = contents
    .split("\n")
    .map(|depth_str| depth_str.trim().parse::<i32>().expect("Not a number"))
    .collect();

  depths
}

pub mod p11 {
  // count depth increases
  pub fn solve() -> i32 {
    let depths = crate::p1::parse_p11();
    let mut prev_depth = -1;
    let mut count = 0;
    for depth in depths {
      if prev_depth < depth {
        count += 1;
      }
      prev_depth = depth;
    }

    println!("count = {}", count - 1);
    count - 1
  }
}

pub mod p12 {
  // count 3 window depth increases
  pub fn solve() -> i32 {
    let depths = crate::p1::parse_p11();

    let n = depths.len();

    let mut three_depths = Vec::<i32>::new();

    let mut i = 0;
    while i < n {
      let d1 = depths[i];
      let d2 = if i + 1 < n { depths[i + 1] } else { 0 };
      let d3 = if i + 2 < n { depths[i + 2] } else { 0 };
      three_depths.push(d1 + d2 + d3);

      i += 1;
    }

    let mut prev_depth = -1;
    let mut count = 0;
    for depth in three_depths {
      if prev_depth < depth {
        count += 1;
      }
      prev_depth = depth;
    }

    println!("count = {}", count - 1);
    count - 1
  }
}
