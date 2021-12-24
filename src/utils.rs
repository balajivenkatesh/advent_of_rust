const BASE_PATH: &str = "D:/rust-projects/advent_of_rust/src/data";

pub fn read_data(px: &str) -> String {
  let filename = BASE_PATH.to_owned() + "/" + px;
  println!("filename = {}", filename);

  std::fs::read_to_string(filename).expect("Somenthing went wrong reading file")
}
