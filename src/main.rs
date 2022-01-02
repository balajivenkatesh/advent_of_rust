mod p1;
mod p10;
mod p2;
mod p3;
mod p4;
mod p5;
mod p6;
mod p8;
mod p9;
mod utils;

use clap::{App, Arg};

fn main() {
  let matches = App::new("Advent of Code 2021")
    .version("1.0")
    .author("Balaji")
    .about("Advent of Code 2021")
    .arg(
      Arg::with_name("day")
        .short("d")
        .long("day")
        .takes_value(true)
        .required(true)
        .help("Day of the calendar"),
    )
    .arg(
      Arg::with_name("part")
        .short("p")
        .long("part")
        .takes_value(true)
        .required(true)
        .help("Part of the puzzle, 1 or 2"),
    )
    .get_matches();

  let day = matches.value_of("day").unwrap_or("1");
  let part = matches.value_of("part").unwrap_or("1");

  let mut puzzle = day.to_owned();
  puzzle.push_str(".");
  puzzle.push_str(part);

  let puzzle = &puzzle[..];

  match puzzle {
    "1.1" => {
      let out = p1::p11::solve();
      assert_eq!(out, 1709);
    }
    "1.2" => {
      let out = p1::p12::solve();
      assert_eq!(out, 1761);
    }
    "2.1" => {
      let out = p2::p21::solve();
      assert_eq!(out, 1484118);
    }
    "2.2" => {
      let out = p2::p22::solve();
      println!("out = {}", out);
      assert_eq!(out, 1463827010);
    }
    "3.1" => {
      let out = p3::p31::solve();
      println!("out = {}", out);
      assert_eq!(out, 1025636);
    }
    "3.2" => {
      let out = p3::p32::solve();
      println!("out = {}", out);
      assert_eq!(out, 793873);
    }
    "4.1" => {
      let out = p4::p41::solve();
      println!("out = {}", out);
      assert_eq!(out, 71708);
    }
    "4.2" => {
      let out = p4::p42::solve();
      println!("out = {}", out);
      assert_eq!(out, 34726);
    }
    "5.1" => {
      let out = p5::p51::solve();
      println!("out = {}", out);
      assert_eq!(out, 5294);
    }
    "5.2" => {
      let out = p5::p52::solve();
      println!("out = {}", out);
      assert_eq!(out, 21698);
    }
    "7.1" => {
      let out = 347449;
      println!("out = {}", out);
      assert_eq!(out, 347449);
    }
    "7.2" => {
      let out = 98039527;
      println!("out = {}", out);
      assert_eq!(out, 98039527);
    }
    "8.1" => {
      let out = p8::p81::solve();
      println!("out = {}", out);
      assert_eq!(out, 321);
    }
    "9.1" => {
      let out = p9::p91::solve();
      println!("out = {}", out);
      assert_eq!(out, 456);
    }
    "9.2" => {
      let out = p9::p92::solve();
      println!("out = {}", out);
      assert_eq!(out, 1047744);
    }
    "10.1" => {
      let out = p10::p1::solve();
      println!("out = {}", out);
      assert_eq!(out, 318099);
    }
    "10.2" => {
      let out = p10::p2::solve();
      println!("out = {}", out);
      assert_eq!(out, 2389738699);
    }
    _ => {
      println!("Unsolved");
    }
  }
}
