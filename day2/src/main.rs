use clap::{App, Arg};
use std::{fmt::Display, fs, process::exit};

struct Position {
    pub horizontal: i64,
    pub depth: i64,
    pub should_aim: bool,
    pub aim: i64,
}

impl Position {
    pub fn origin(should_aim: bool) -> Self {
        Self {
            horizontal: 0,
            depth: 0,
            aim: 0,
            should_aim,
        }
    }

    pub fn forward(&mut self, units: i64) {
        self.horizontal += units;
        if self.should_aim {
            self.depth += self.aim * units;
        }
    }

    pub fn up(&mut self, units: i64) {
        if self.should_aim {
            self.aim -= units;
        } else {
            self.depth -= units;
        }
    }

    pub fn down(&mut self, units: i64) {
        if self.should_aim {
            self.aim += units;
        } else {
            self.depth += units;
        }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.should_aim {
            write!(
                f,
                "({}, {}), aim: {}",
                self.horizontal, self.depth, self.aim
            )
        } else {
            write!(f, "({}, {})", self.horizontal, self.depth)
        }
    }
}

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
