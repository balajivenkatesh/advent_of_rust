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
    let mut win_row = 9999;
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
        win_row = i;
      };
      i += 5;
    }
    let win_number = inputs[win_board_min_win_pos as usize];
    let struck_numbers = inputs[0..win_board_min_win_pos as usize + 1].to_vec();
    let win_board = boards[curr_win_board_id * 5..curr_win_board_id * 5 + 5].to_vec();
    println!("{:?}", win_board);
    println!("win_number = {}", win_number);
    println!("win_row = {}", win_row);
    let mut flat_win_board: Vec<i32> = win_board.iter().flatten().map(|x| *x).collect();
    flat_win_board.retain(|x| !struck_numbers.contains(x));
    let sum: i32 = flat_win_board.clone().iter().sum();

    sum * win_number
  }
}

// pub mod p32 {
//   use crate::p3;

//   fn filter_readings(subset_readings: &Vec<String>, i: usize, filter_bit: u8) -> Vec<String> {
//     let mut clone_readings = subset_readings.clone();
//     clone_readings.retain(|x| x.as_bytes()[i] == filter_bit + 48);
//     clone_readings
//   }

//   fn bin_to_i32(bin: &String) -> i32 {
//     let mut out = 0;
//     for b in bin.as_bytes() {
//       out = out * 2 + ((b - 48) as i32);
//     }
//     out as i32
//   }

//   // oxygen and CO2 rating
//   pub fn solve() -> i32 {
//     let mut curr_readings = p3::parse_p3();
//     let mut i = 0;
//     while curr_readings.len() > 1 {
//       let curr_common_bit = p3::common_bit(&curr_readings, i, curr_readings.len());
//       curr_readings = filter_readings(&curr_readings, i, curr_common_bit);
//       i += 1;
//     }
//     let oxy = bin_to_i32(&curr_readings[0]);

//     let mut curr_readings = p3::parse_p3();
//     let mut i = 0;
//     while curr_readings.len() > 1 {
//       let curr_common_bit = p3::common_bit(&curr_readings, i, curr_readings.len());
//       curr_readings = filter_readings(&curr_readings, i, if curr_common_bit == 0 { 1 } else { 0 });
//       i += 1;
//     }
//     let co2 = bin_to_i32(&curr_readings[0]);

//     oxy * co2
//   }
// }
