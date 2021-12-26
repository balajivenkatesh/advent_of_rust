#![allow(dead_code)]

use crate::utils;

fn parse_p6() -> Vec<i32> {
  let contents = utils::read_data("/p6test.txt");
  utils::split_parse_i32(&contents, ",")
}

pub mod p61 {
  use crate::p6;

  // winning bingo board is a vert board
  pub fn solve() -> i32 {
    let inputs = p6::parse_p6();
    let mut data = [[0; 80]; 9];
    for i in 0..9 {
      data[i][0] = i;
    }
    for j in 1..80 {
      for i in 0..9 {
        let next = if data[i][j - 1] != 0 {
          data[i][j - 1] - 1
        } else {
          6
        };
        data[i][j] = next;
      }
    }
    for i in 0..9 {
      let mut ct = 1;
      for j in 0..80 {
        let timer = data[i][j];
        data[i][j] = ct;
        if timer == 0 {
          ct += 1;
        }
      }
    }

    // println!("data = {:?}", data);

    let mut jelly_ct = inputs.len();
    let curr_gen = inputs.clone();
    let next_gen = Vec::<i32>::new();
    for i in 0..80 {}
    0
  }
}
