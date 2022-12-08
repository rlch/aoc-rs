pub mod position;

use clap::{App, Arg};
use position::Position;
use std::{fs, process::exit};

fn main() {
    let matches = App::new("Day 2: Dive!")
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
            Arg::new("aim")
                .about("Whether to track the submarines aim.")
                .short('a')
                .long("aim"),
        )
        .get_matches();

    let input =
        fs::read_to_string(matches.value_of("file").unwrap()).expect("Could not read your input.");

    let should_aim = matches.is_present("aim");

    let mut position = Position::origin(should_aim);

    for line in input.lines() {
        let mut iter = line.split_whitespace();
        let command = iter.next().expect("Could not obtain command.");
        let units = iter
            .next()
            .expect("Could not obtain units.")
            .parse::<i64>()
            .expect("Could not parse units as integer.");

        match command {
            "forward" => position.forward(units),
            "down" => position.down(units),
            "up" => position.up(units),
            x => panic!("Unrecognized command: {}", x),
        }
    }

    println!("Reached position: {}", position);
    println!(
        "The multiple of these values is: {}",
        position.depth * position.horizontal
    );

    exit(0);
}
