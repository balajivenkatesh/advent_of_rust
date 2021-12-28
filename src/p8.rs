#![allow(dead_code)]

use crate::utils;

fn parse_p8() -> Vec<(Vec<String>, Vec<String>)> {
  let lines = utils::read_lines("/p8.txt");
  lines
    .iter()
    .map(|line| {
      let split_contents = utils::split_str(line, &String::from(" | "));
      (
        utils::split_str(&(split_contents[0]), &String::from(" ")),
        utils::split_str(&(split_contents[1]), &String::from(" ")),
      )
    })
    .collect()
}

pub mod p81 {
  use crate::p8;

  pub fn solve() -> i32 {
    let data = p8::parse_p8();
    let mut count = 0;
    for datum in data {
      // println!("ip = {:?}", datum.1);
      for note in datum.1 {
        if [2, 3, 4, 7].contains(&(note.len() as i32)) {
          // println!("note = {} {}", note, note.len());
          count += 1;
        }
      }
    }
    count
  }
}
