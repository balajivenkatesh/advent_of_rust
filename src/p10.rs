#![allow(dead_code)]

use crate::utils;
use std::collections::HashMap;

fn parse_p10() -> Vec<String> {
  utils::read_lines("/p10.txt")
}

fn score_line(line: &String) -> (i32, Vec<String>) {
  let mut score = 0;
  let closing = HashMap::from([("[", "]"), ("{", "}"), ("(", ")"), ("<", ">")]);
  let scoring = HashMap::from([("]", 57), ("}", 1197), (")", 3), (">", 25137)]);

  let mut stack: Vec<String> = Vec::new();
  for s in utils::split_str(&line, &String::from("")).iter() {
    let y = &s[..];
    match y {
      "[" | "{" | "<" | "(" => stack.push(y.to_owned()),
      "]" | "}" | ">" | ")" => {
        let expected_closing = *closing.get(&stack.last().unwrap()[..]).unwrap();
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
  (score, stack)
}

pub mod p1 {
  use crate::p10;

  pub fn solve() -> i32 {
    let lines = p10::parse_p10();

    let mut score = 0;

    for line in lines {
      let (line_score, _) = p10::score_line(&line);
      score += line_score;
    }
    score
  }
}

pub mod p2 {
  use crate::p10;
  use std::collections::HashMap;

  fn line_autocomplete_score(stack: &mut Vec<String>) -> i64 {
    let score_closing = HashMap::from([("[", 2), ("{", 3), ("(", 1), ("<", 4)]);
    let mut score: i64 = 0;
    stack.reverse();
    for s in stack {
      score = score * 5 + score_closing.get(&s[..]).unwrap_or(&0);
    }
    score
  }

  pub fn solve() -> i64 {
    let lines = p10::parse_p10();

    let mut line_scores: Vec<i64> = Vec::new();

    for line in lines {
      let (line_score, mut stack) = p10::score_line(&line);
      if line_score == 0 {
        line_scores.push(line_autocomplete_score(&mut stack));
      }
    }
    line_scores.sort();
    line_scores[line_scores.len() / 2]
  }
}
