#![allow(dead_code)]

const DATA_BASE_PATH: &str = "./2022/data";

pub fn read_data(px: &str) -> String {
  let filename = DATA_BASE_PATH.to_owned() + "/" + px;
  // println!("filename = {}", filename);

  std::fs::read_to_string(filename).expect("Somenthing went wrong reading file")
}

pub fn read_lines(px: &str) -> Vec<String> {
  let contents = read_data(px);

  split_str(&contents, &String::from("\n"))
}

pub fn split_parse_i32(text: &str, split: &str) -> Vec<i32> {
  text
    .split(split)
    .filter(|x| *x != "")
    .map(|x| x.parse().expect("NaN"))
    .collect()
}

pub fn split_str(text: &String, split: &String) -> Vec<String> {
  text.split(split).map(|x| x.to_string()).collect()
}
