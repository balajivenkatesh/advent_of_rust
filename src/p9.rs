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

pub mod p92 {
  use crate::p9;

  pub fn traverse_basin(data: &mut Vec<Vec<u8>>, x: i32, y: i32) -> i32 {
    if x < 0 || y < 0 || x >= data.len() as i32 || y >= data[0].len() as i32 {
      return 0;
    }
    if data[x as usize][y as usize] == 9 || data[x as usize][y as usize] == 10 {
      return 0;
    }
    data[x as usize][y as usize] = 10;
    1 + traverse_basin(data, x + 1, y)
      + traverse_basin(data, x, y + 1)
      + traverse_basin(data, x - 1, y)
      + traverse_basin(data, x, y - 1)
  }

  pub fn solve() -> i32 {
    let mut data = p9::parse_p9();

    let mut basin_sizes: Vec<i32> = Vec::new();

    for i in 0..data.len() {
      for j in 0..data[i].len() {
        if data[i][j] == 9 || data[i][j] == 10 {
          continue;
        }
        basin_sizes.push(traverse_basin(&mut data, i as i32, j as i32));
      }
    }
    basin_sizes.sort();
    basin_sizes.reverse();
    basin_sizes[0] * basin_sizes[1] * basin_sizes[2]
  }
}
