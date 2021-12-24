#![allow(dead_code)]

use crate::utils;
use std::collections::HashMap;

fn parse_p4() -> (Vec<i32>, Vec<Vec<i32>>) {
  let contents = utils::read_data("/p4.txt");
  let lines: Vec<String> = contents.split("\n").map(|x| x.trim().to_string()).collect();
  let (inputs, boards) = lines.split_at(1);
  let inputs = inputs[0]
    .split(",")
    .map(|x| x.trim().parse::<i32>().expect("not a number"))
    .collect::<Vec<i32>>();

  let mut boards = boards.to_vec();
  boards.retain(|x| x != "");
  // let boards: Vec<Vec<String>> = boards.iter().map(|x| {
  let boards: Vec<Vec<&str>> = boards.iter().map(|x| x.split(" ").collect()).collect();
  let boards: Vec<Vec<i32>> = boards
    .iter()
    .map(|x| {
      let mut z = x.clone();
      z.retain(|y| !y.to_string().is_empty());
      z.iter().map(|y| y.parse().expect("not a number")).collect()
    })
    .collect();
  (inputs, boards)
}

fn pos_map(inputs: &Vec<i32>) -> HashMap<i32, i32> {
  let mut pos_map: HashMap<i32, i32> = HashMap::new();
  let mut i = 0;
  for input in inputs {
    pos_map.insert(*input, i);
    i += 1;
  }
  pos_map
}

fn min_win_pos(inputs: &Vec<i32>, pos_map: &HashMap<i32, i32>) -> i32 {
  let x: Vec<i32> = inputs.iter().map(|i| *pos_map.get(i).unwrap()).collect();
  *x.iter().max().unwrap()
}

pub mod p41 {
  use crate::p4;

  // winning bingo board is a vert board
  pub fn solve() -> i32 {
    let (inputs, boards) = p4::parse_p4();
    let pos_map = p4::pos_map(&inputs);

    let mut i = 0;
    let mut curr_win_board_id: usize = 999999;
    let mut win_board_min_win_pos = 999999;

    while i < boards.len() {
      let mut j = 0;
      let mut board_min_win_pos = 999999;
      while j < 5 {
        let row_min_win_pos = p4::min_win_pos(
          &[
            boards[i][j],
            boards[i + 1][j],
            boards[i + 2][j],
            boards[i + 3][j],
            boards[i + 4][j],
          ]
          .to_vec(),
          // &boards[i],
          &pos_map,
        );
        if row_min_win_pos < board_min_win_pos {
          board_min_win_pos = row_min_win_pos;
        };
        j += 1;
        // i += 1;
      }
      if board_min_win_pos < win_board_min_win_pos {
        win_board_min_win_pos = board_min_win_pos;
        curr_win_board_id = i / 5;
      };
      i += 5;
    }
    let win_number = inputs[win_board_min_win_pos as usize];
    let struck_numbers = inputs[0..win_board_min_win_pos as usize + 1].to_vec();
    let win_board = boards[curr_win_board_id * 5..curr_win_board_id * 5 + 5].to_vec();

    let mut flat_win_board: Vec<i32> = win_board.iter().flatten().map(|x| *x).collect();
    flat_win_board.retain(|x| !struck_numbers.contains(x));
    let sum: i32 = flat_win_board.clone().iter().sum();

    sum * win_number
  }
}

pub mod p42 {
  use crate::p4;

  // winning bingo board is a vert board
  pub fn solve() -> i32 {
    let (inputs, boards) = p4::parse_p4();
    let pos_map = p4::pos_map(&inputs);

    let mut i = 0;
    let mut curr_win_board_id: usize = 999999;
    let mut win_board_min_win_pos = -1;

    while i < boards.len() {
      let mut j = 0;
      let mut board_min_win_pos = 9999;
      while j < 5 {
        let row_min_win_pos = p4::min_win_pos(
          &[
            boards[i][j],
            boards[i + 1][j],
            boards[i + 2][j],
            boards[i + 3][j],
            boards[i + 4][j],
          ]
          .to_vec(),
          // &boards[i],
          &pos_map,
        );
        if row_min_win_pos < board_min_win_pos {
          board_min_win_pos = row_min_win_pos;
        };
        j += 1;
        // i += 1;
      }
      if board_min_win_pos > win_board_min_win_pos {
        win_board_min_win_pos = board_min_win_pos;
        curr_win_board_id = i / 5;
      };
      i += 5;
    }
    let win_number = inputs[win_board_min_win_pos as usize];
    let struck_numbers = inputs[0..win_board_min_win_pos as usize + 1].to_vec();
    let win_board = boards[curr_win_board_id * 5..curr_win_board_id * 5 + 5].to_vec();

    let mut flat_win_board: Vec<i32> = win_board.iter().flatten().map(|x| *x).collect();
    flat_win_board.retain(|x| !struck_numbers.contains(x));
    let sum: i32 = flat_win_board.clone().iter().sum();

    sum * win_number
  }
}
