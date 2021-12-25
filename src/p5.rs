#![allow(dead_code)]

use crate::utils;

#[derive(Copy, Clone)]
struct Point {
  x: i32,
  y: i32,
}

struct Line {
  a: Point,
  b: Point,
}

fn parse_p5() -> Vec<Line> {
  let contents = utils::read_data("/p5.txt");
  let lines: Vec<String> = contents.split("\n").map(|x| x.trim().to_string()).collect();

  let lines: Vec<Line> = lines
    .iter()
    .map(|x| {
      let txt_line = x.split(" -> ").collect::<Vec<&str>>();
      let pts: Vec<Point> = txt_line
        .iter()
        .map(|&txt_pt| {
          let nums = utils::split_parse_i32(txt_pt, ",");
          Point {
            x: nums[0],
            y: nums[1],
          }
        })
        .collect();

      Line {
        a: pts[0],
        b: pts[1],
      }
    })
    .collect();

  lines
}

pub mod p51 {
  use crate::p5;
  use std::cmp;

  // winning bingo board is a vert board
  pub fn solve() -> i32 {
    let lines = p5::parse_p5();

    let mut grid = Vec::<Vec<u8>>::new();

    let size = 1000;
    for _i in 0..size {
      let mut row = Vec::<u8>::new();
      for _j in 0..size {
        row.push(0);
      }
      grid.push(row);
    }

    for line in lines {
      if line.a.x == line.b.x {
        for j in cmp::min(line.a.y, line.b.y)..(cmp::max(line.a.y, line.b.y) + 1) {
          grid[line.a.x as usize][j as usize] += 1;
        }
      } else if line.a.y == line.b.y {
        for i in cmp::min(line.a.x, line.b.x)..(cmp::max(line.a.x, line.b.x) + 1) {
          grid[i as usize][line.a.y as usize] += 1;
        }
      } else {
        // println!("here");
      }
    }
    let mut count = 0;
    for row in grid {
      // println!("{:?}", row);
      for cell in row {
        if cell > 1 {
          count += 1;
        }
      }
    }
    count
  }
}
