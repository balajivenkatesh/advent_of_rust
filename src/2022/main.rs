mod p1;
// mod p10;
mod p2;
// mod p3;
// mod p4;
// mod p5;
// mod p6;
// mod p8;
// mod p9;
mod utils;

use clap::{App, Arg};

fn main() {
    let matches = App::new("Advent of Code 2022")
        .version("1.0")
        .author("Balaji")
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

    println!("");

    let day = matches.value_of("day").unwrap_or("1");
    if day.parse::<i32>().expect("Day is NaN") > 25 {
        clap::Error::with_description(
            "Invalid day, must be between 1 and 25\n",
            clap::ErrorKind::InvalidValue,
        )
        .exit();
    }

    let part = matches.value_of("part").unwrap_or("1");
    match part {
        "1" | "2" => {}
        _ => {
            clap::Error::with_description(
                "Invalid part, must be 1 or 2\n",
                clap::ErrorKind::InvalidValue,
            )
            .exit();
        }
    }

    println!("Running day = {}, part = {}\n", day, part);

    let mut puzzle = day.to_owned();
    puzzle.push_str(".");
    puzzle.push_str(part);

    let puzzle = &puzzle[..];

    match puzzle {
        "1.1" => {
            let out = p1::p11::solve();
            println!("out [puzzle {}] = {}", puzzle, out);
            assert_eq!(out, 72240);
        }
        "1.2" => {
            let out = p1::p12::solve();
            println!("out [puzzle {}] = {}", puzzle, out);
            assert_eq!(out, 210957);
        }
        "2.1" => {
            let out = p2::p21::solve();
            println!("out [puzzle {}] = {}", puzzle, out);
            assert_eq!(out, 12535);
        }
        // "2.2" => {
        //   let out = p2::p22::solve();
        //   println!("out = {}", out);
        //   assert_eq!(out, 1463827010);
        // }
        // "3.1" => {
        //   let out = p3::p31::solve();
        //   println!("out = {}", out);
        //   assert_eq!(out, 1025636);
        // }
        // "3.2" => {
        //   let out = p3::p32::solve();
        //   println!("out = {}", out);
        //   assert_eq!(out, 793873);
        // }
        // "4.1" => {
        //   let out = p4::p41::solve();
        //   println!("out = {}", out);
        //   assert_eq!(out, 71708);
        // }
        // "4.2" => {
        //   let out = p4::p42::solve();
        //   println!("out = {}", out);
        //   assert_eq!(out, 34726);
        // }
        // "5.1" => {
        //   let out = p5::p51::solve();
        //   println!("out = {}", out);
        //   assert_eq!(out, 5294);
        // }
        // "5.2" => {
        //   let out = p5::p52::solve();
        //   println!("out = {}", out);
        //   assert_eq!(out, 21698);
        // }
        // "7.1" => {
        //   let out = 347449;
        //   println!("out = {}", out);
        //   assert_eq!(out, 347449);
        // }
        // "7.2" => {
        //   let out = 98039527;
        //   println!("out = {}", out);
        //   assert_eq!(out, 98039527);
        // }
        // "8.1" => {
        //   let out = p8::p81::solve();
        //   println!("out = {}", out);
        //   assert_eq!(out, 321);
        // }
        // "9.1" => {
        //   let out = p9::p91::solve();
        //   println!("out = {}", out);
        //   assert_eq!(out, 456);
        // }
        // "9.2" => {
        //   let out = p9::p92::solve();
        //   println!("out = {}", out);
        //   assert_eq!(out, 1047744);
        // }
        // "10.1" => {
        //   let out = p10::p1::solve();
        //   println!("out = {}", out);
        //   assert_eq!(out, 318099);
        // }
        // "10.2" => {
        //   let out = p10::p2::solve();
        //   println!("out = {}", out);
        //   assert_eq!(out, 2389738699);
        // }
        _ => {
            println!("Unsolved");
        }
    }
    println!("");
}
