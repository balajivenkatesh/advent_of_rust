#![allow(dead_code)]

const BASE_PATH: &str = "D:/rust-projects/advent_of_rust/src/data";

fn parse_p3() -> Vec<String> {
  let filename = crate::p3::BASE_PATH.to_owned() + "/p3.txt";
  // println!("filename = {}", filename);

  let contents = std::fs::read_to_string(filename).expect("Somenthing went wrong reading file");
  contents.split("\n").map(|x| x.trim().to_string()).collect()
}

fn count_bit(readings: &Vec<String>, i: usize) -> usize {
  let mut count = 0;
  for reading in readings {
    if reading.as_bytes()[i] == 48 {
      count += 1;
    }
  }
  return count;
}

pub mod p31 {
  use crate::p3;

  // product of gamma and epsilon
  pub fn solve() -> i32 {
    let readings = p3::parse_p3();
    let n = readings.len();
    let mut i: usize = 0;

    let mut gamma = 0;
    let mut epsilon = 0;

    while i < readings[0].len() {
      let x = p3::count_bit(&readings, i);
      if x > (n / 2) {
        gamma = gamma * 2 + 0;
        epsilon = epsilon * 2 + 1;
      } else {
        gamma = gamma * 2 + 1;
        epsilon = epsilon * 2 + 0;
      }
      i += 1;
    }
    gamma * epsilon
  }
}

// pub mod p22 {
//   // product of horizontal and vert but using aim
//   pub fn solve() -> i32 {
//     let commands = crate::p3::parse_p3();
//     let mut aim = 0;
//     let mut horiz = 0;
//     let mut vert = 0;

//     for command in commands {
//       match command.direction {
//         crate::p2::Dir::Up => aim -= command.dist,
//         crate::p2::Dir::Down => aim += command.dist,
//         crate::p2::Dir::Forward => {
//           horiz += command.dist;
//           vert += aim * command.dist;
//         }
//         _ => (),
//       }
//     }

//     // println!("horiz = {}, vert = {}", horiz, vert);
//     horiz * vert
//   }
// }
