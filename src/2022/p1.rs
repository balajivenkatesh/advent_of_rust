#![allow(dead_code)]

use crate::utils;

fn parse_p1() -> Vec<i32> {
    let contents = utils::read_data("/p1.txt");
    contents
        .split("\n")
        .map(|x| match x.parse() {
            Ok(int_val) => int_val,
            Err(_) => 0,
        })
        .collect()
}

pub mod p11 {
    use crate::p1;

    pub fn solve() -> i32 {
        let data = p1::parse_p1();
        let mut max = 0;
        let mut sum = 0;

        for datum in data {
            sum += datum;
            if datum == 0 {
                if sum > max {
                    max = sum;
                }
                sum = 0;
            }
        }
        max
    }
}

pub mod p12 {
    use crate::p1;

    // winning bingo board is a vert board
    pub fn solve() -> i32 {
        let data = p1::parse_p1();
        let mut cals: Vec<i32> = Vec::new();
        let mut sum = 0;

        for datum in data {
            sum += datum;
            if datum == 0 {
                cals.push(sum);
                sum = 0;
            }
        }

        cals.sort();
        cals.reverse();

        cals[0] + cals[1] + cals[2]
    }
}
