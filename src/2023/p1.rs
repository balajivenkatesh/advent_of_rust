#![allow(dead_code)]

use crate::utils;

fn parse_p1() -> Vec<i32> {
    let contents = utils::read_lines("/p1.txt");
    let transformed: Vec<i32> = contents
        .iter()
        .map(|x| {
            let numerals: Vec<&str> = x
                .split("")
                .into_iter()
                .filter(|c| c.chars().next().map_or(false, |c| c.is_ascii_digit()))
                .collect();
            let first = numerals.first().unwrap_or(&"");
            let last = numerals.last().unwrap_or(&"");

            let number = format!("{}{}", first, last);
            number
        })
        .map(|txt| txt.parse().expect("NaN"))
        .collect();
    transformed
}

pub mod p11 {
    use crate::p1;

    pub fn solve() -> i32 {
        let data = p1::parse_p1();
        data.iter().sum()
    }
}

pub mod p12 {
    use crate::p1;

    // winning bingo board is a vert board
    pub fn solve() -> i32 {
        0
    }
}
