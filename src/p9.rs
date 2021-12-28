#![allow(dead_code)]

use crate::utils;

fn parse_p9() -> Vec<Vec<u8>> {
  let lines = utils::read_lines("/p9.txt");
  lines
    .iter()
    .map(|line| {
      utils::split_parse_i32(line, &String::from(""))
        .iter()
        .map(|num| *num as u8)
        .collect()
    })
    .collect()
}

pub mod p91 {
  use crate::p9;

  pub fn solve() -> i32 {
    let data = p9::parse_p9();
    let mut count = 0;

    for i in 0..data.len() {
      for j in 0..data[i].len() {
        let top = if i > 0 { data[i - 1][j] } else { 10 };
        let right = if j < data[i].len() - 1 {
          data[i][j + 1]
        } else {
          10
        };
        let bot = if i < data.len() - 1 {
          data[i + 1][j]
        } else {
          10
        };
        let left = if j > 0 { data[i][j - 1] } else { 10 };
        if data[i][j] < top && data[i][j] < right && data[i][j] < bot && data[i][j] < left {
          count += (1 + data[i][j]) as i32;
        }
      }
    }
    count
  }
}
