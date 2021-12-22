#![allow(dead_code)]

const BASE_PATH: &str = "D:/rust-projects/advent_of_rust/src/data";

enum Dir {
  Unspecified,
  Up,
  Down,
  Forward,
}

pub struct Command {
  direction: Dir,
  dist: i32,
}

pub fn parse_p2() -> Vec<Command> {
  let filename = crate::p2::BASE_PATH.to_owned() + "/p2.txt";
  // println!("filename = {}", filename);

  let contents = std::fs::read_to_string(filename).expect("Somenthing went wrong reading file");
  contents
    .split("\n")
    .map(|command_str| {
      let dir_dist: Vec<&str> = command_str.trim().split(" ").collect();
      let dir = match dir_dist[0] {
        "forward" => Dir::Forward,
        "up" => Dir::Up,
        "down" => Dir::Down,
        _ => Dir::Unspecified,
      };
      let dist = dir_dist[1].parse().expect("not a number");
      Command {
        direction: dir,
        dist: dist,
      }
    })
    .collect()
}

pub mod p21 {
  // product of horizontal and vert
  pub fn solve() -> i32 {
    let commands = crate::p2::parse_p2();
    let mut horiz = 0;
    let mut vert = 0;

    for command in commands {
      match command.direction {
        crate::p2::Dir::Up => vert -= command.dist,
        crate::p2::Dir::Down => vert += command.dist,
        crate::p2::Dir::Forward => horiz += command.dist,
        _ => (),
      }
    }

    // println!("horiz = {}, vert = {}", horiz, vert);
    horiz * vert
  }
}

pub mod p22 {
  // product of horizontal and vert but using aim
  pub fn solve() -> i32 {
    let commands = crate::p2::parse_p2();
    let mut aim = 0;
    let mut horiz = 0;
    let mut vert = 0;

    for command in commands {
      match command.direction {
        crate::p2::Dir::Up => aim -= command.dist,
        crate::p2::Dir::Down => aim += command.dist,
        crate::p2::Dir::Forward => {
          horiz += command.dist;
          vert += aim * command.dist;
        }
        _ => (),
      }
    }

    // println!("horiz = {}, vert = {}", horiz, vert);
    horiz * vert
  }
}
