#![feature(array_zip, unchecked_math, int_abs_diff)]

pub mod vent;

use clap::{App, Arg};
use std::collections::HashMap;
use std::{fs, process::exit};

use crate::vent::Vent;

fn main() {
    let matches = App::new("Day 5: Hydrothermal Venture")
        .version("1.0.0")
        .about("Advent of Code 2021")
        .arg(
            Arg::new("file")
                .help("Relative location of file containing input.")
                .takes_value(true)
                .value_name("FILE")
                .index(1)
                .required(true),
        )
        .arg(Arg::new("diagonal").short('d').long("diagonal"))
        .get_matches();

    let input =
        fs::read_to_string(matches.value_of("file").unwrap()).expect("Could not read your input.");
    let diagonal = matches.is_present("diagonal");

    solve(input, diagonal);

    exit(0);
}

fn solve(input: String, diagonal: bool) {
    let mut counter = HashMap::<(u32, u32), u8>::new();

    for line in input.lines() {
        let vent = Vent::parse(line);

        if vent.is_line() || (!diagonal || vent.is_diagonal()) {
            for coord in vent.spanning_set(diagonal) {
                counter.insert(coord, *counter.get(&coord).unwrap_or(&0) + 1);
            }
        }
    }

    let overlaps: u32 = counter
        .values()
        .fold(0u32, |acc, curr| acc + (if *curr > 1 { 1 } else { 0 }));

    println!("Number of overlaps: {}", overlaps)
}
