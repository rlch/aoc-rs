use clap::{App, Arg};
use std::{fs, process::exit};

fn main() {
    let matches = App::new("Day 1: Sonar Sweep")
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
        .get_matches();

    if let Some(filename) = matches.value_of("file") {
        let input = fs::read_to_string(filename)
            .unwrap_or_else(|_| panic!("Could not read your input at `{}`.", filename));

        let mut prev_depth: u32 = u32::MAX;
        let mut increases: u32 = 0;
        for line in input.lines() {
            let depth = line.parse::<u32>().unwrap();
            if depth > prev_depth {
                increases += 1;
            }
            prev_depth = depth;
        }

        println!("The depth increased {} times.", increases);
        exit(0);
    }
}
