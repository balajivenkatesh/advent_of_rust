#![allow(dead_code)]

use crate::utils;

fn parse_p10() -> Vec<String> {
  utils::read_lines("/p10test.txt")
}

pub mod p1 {
  use crate::p10;
  use crate::utils;
  use std::collections::HashMap;

  pub fn solve() -> i32 {
    let lines = p10::parse_p10();

    let closing = HashMap::from([("[", "]"), ("{", "}"), ("(", ")"), ("<", ">")]);
    let scoring = HashMap::from([("]", 57), ("}", 1197), (")", 3), (">", 25137)]);

    let mut score = 0;

    for line in lines {
      let mut stack: Vec<&str> = Vec::new();
      for s in utils::split_str(&line, &String::from("")).iter() {
        let y = &s[..];
        match y {
          "[" | "{" | "<" | "(" => stack.push(y),
          "]" | "}" | ">" | ")" => {
            let expected_closing = *closing.get(stack.last().unwrap()).unwrap();
            if expected_closing != y {
              score += scoring.get(y).unwrap();
              break;
            } else {
              stack.pop();
            }
          }
          _ => {}
        };
      }
    }
    score
  }
}
