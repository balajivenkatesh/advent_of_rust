#![allow(dead_code)]

use crate::utils;

fn parse_p2() -> Vec<String> {
    let contents = utils::read_lines("/p2.txt");
    contents
}

pub mod p21 {
    use crate::p2;
    use crate::utils;

    pub fn solve() -> i32 {
        let lines = p2::parse_p2();

        let mut score = 0;

        for line in lines {
            let round = utils::split_str(&line, &String::from(" "));
            let left = round[0].as_str();
            let right = round[1].as_str();
            // println!("left = {}, right = {}", left, right);

            let left_val = match left {
                "A" => 1,
                "B" => 2,
                "C" => 3,
                _ => 4,
            };
            let right_val = match right {
                "X" => 1,
                "Y" => 2,
                "Z" => 3,
                _ => 0,
            };

            score = score + right_val;
            if left_val == 1 && right_val == 3 {
            } else if left_val == 3 && right_val == 1 {
                score = score + 6;
            } else if left_val < right_val {
                score = score + 6;
            } else if left_val == right_val {
                score = score + 3;
            }
        }
        score
    }
}

pub mod p22 {
    use crate::p2;

    // winning bingo board is a vert board
    pub fn solve() -> i32 {
        0
    }
}
