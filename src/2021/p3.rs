#![allow(dead_code)]

const BASE_PATH: &str = "D:/rust-projects/advent_of_rust/src/data";

fn parse_p3() -> Vec<String> {
  let filename = crate::p3::BASE_PATH.to_owned() + "/p3.txt";
  // println!("filename = {}", filename);

  let contents = std::fs::read_to_string(filename).expect("Somenthing went wrong reading file");
  contents.split("\n").map(|x| x.trim().to_string()).collect()
}

fn common_bit(readings: &Vec<String>, i: usize, n: usize) -> u8 {
  let mut count = 0;
  for reading in readings {
    if reading.as_bytes()[i] == 48 {
      count += 1;
    }
  }
  return if count > n / 2 { 0 } else { 1 };
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
      let common_bit = p3::common_bit(&readings, i, n);
      if common_bit == 0 {
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

pub mod p32 {
  use crate::p3;

  fn filter_readings(subset_readings: &Vec<String>, i: usize, filter_bit: u8) -> Vec<String> {
    let mut clone_readings = subset_readings.clone();
    clone_readings.retain(|x| x.as_bytes()[i] == filter_bit + 48);
    clone_readings
  }

  fn bin_to_i32(bin: &String) -> i32 {
    let mut out = 0;
    for b in bin.as_bytes() {
      out = out * 2 + ((b - 48) as i32);
    }
    out as i32
  }

  // oxygen and CO2 rating
  pub fn solve() -> i32 {
    let mut curr_readings = p3::parse_p3();
    let mut i = 0;
    while curr_readings.len() > 1 {
      let curr_common_bit = p3::common_bit(&curr_readings, i, curr_readings.len());
      curr_readings = filter_readings(&curr_readings, i, curr_common_bit);
      i += 1;
    }
    let oxy = bin_to_i32(&curr_readings[0]);

    let mut curr_readings = p3::parse_p3();
    let mut i = 0;
    while curr_readings.len() > 1 {
      let curr_common_bit = p3::common_bit(&curr_readings, i, curr_readings.len());
      curr_readings = filter_readings(&curr_readings, i, if curr_common_bit == 0 { 1 } else { 0 });
      i += 1;
    }
    let co2 = bin_to_i32(&curr_readings[0]);

    oxy * co2
  }
}
