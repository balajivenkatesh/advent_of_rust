const BASE_PATH: &str = "D:/rust-projects/advent_of_rust/src/data";

pub mod p11 {
  // count depth increases
  pub fn solve() -> i32 {
    let filename = crate::p1::BASE_PATH.to_owned() + "/p11.txt";
    // println!("filename = {}", filename);

    let contents = std::fs::read_to_string(filename).expect("Somenthing went wrong reading file");
    let depths = contents
      .split("\n")
      .map(|depth_str| depth_str.trim().parse::<i32>().expect("Not a number"));

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
