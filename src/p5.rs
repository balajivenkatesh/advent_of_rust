#![allow(dead_code)]

use crate::utils;
use std::cmp;
extern crate num;

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

fn init_grid(size: usize) -> Vec<Vec<u8>> {
  let mut grid = Vec::<Vec<u8>>::new();

  for _i in 0..size {
    let mut row = Vec::<u8>::new();
    for _j in 0..size {
      row.push(0);
    }
    grid.push(row);
  }
  grid
}

fn assess_grid(lines: &Vec<Line>, grid: &mut Vec<Vec<u8>>, assess_diag: bool) {
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
      if !assess_diag {
        continue;
      }
      let mut j = 0;
      let xdir = num::signum(line.b.x - line.a.x);
      let ydir = num::signum(line.b.y - line.a.y);
      while j <= cmp::min((line.a.x - line.b.x).abs(), (line.a.y - line.b.y).abs()) {
        grid[(line.a.x + xdir * j) as usize][(line.a.y + ydir * j) as usize] += 1;
        j += 1;
      }
      // println!("here");
    }
  }
}

fn grid_risk(grid: &Vec<Vec<u8>>) -> i32 {
  let mut count = 0;
  for row in grid {
    // println!("{:?}", row);
    for cell in row {
      if *cell > 1 {
        count += 1;
      }
    }
  }
  count
}

pub mod p51 {
  use crate::p5;

  // winning bingo board is a vert board
  pub fn solve() -> i32 {
    let lines = p5::parse_p5();

    let mut grid = p5::init_grid(1000);

    p5::assess_grid(&lines, &mut grid, false);

    p5::grid_risk(&grid)
  }
}

pub mod p52 {
  use crate::p5;

  // winning bingo board is a vert board
  pub fn solve() -> i32 {
    let lines = p5::parse_p5();

    let mut grid = p5::init_grid(1000);

    p5::assess_grid(&lines, &mut grid, true);

    p5::grid_risk(&grid)
  }
}
