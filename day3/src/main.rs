#![feature(drain_filter)]
pub mod part1;
pub mod part2;

use clap::{App, Arg};
use std::{fs, process::exit};

use crate::{part1::part1, part2::part2};

fn main() {
    let matches = App::new("Day 3: Binary Diagnostic")
        .version("1.0.0")
        .about("Advent of Code 2021")
        .arg(
            Arg::new("file")
                .about("Relative location of file containing input.")
                .takes_value(true)
                .value_name("FILE")
                .index(1)
                .required(true),
        )
        .arg(
            Arg::new("part")
                .about("Part 1 or 2")
                .short('p')
                .long("part")
                .takes_value(true)
                .value_name("1 | 2")
                .default_value("1"),
        )
        .get_matches();

    let input =
        fs::read_to_string(matches.value_of("file").unwrap()).expect("Could not read your input.");
    let part = matches.value_of("part").unwrap().parse::<u32>();

    match part {
        Ok(1) => part1(input),
        Ok(2) => part2(input),
        _ => panic!("part must be 1 or 2."),
    }

    exit(0);
}
