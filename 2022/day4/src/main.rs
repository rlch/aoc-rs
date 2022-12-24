mod part1;
mod part2;

use std::path::PathBuf;

use clap::{arg, command, value_parser, Command};

fn main() {
    let matches = command!() // requires `cargo` feature
        .name("Advent of Code 2022: Day 4")
        .about("Day 4: Camp Cleanup")
        .arg(
            arg!(
                -f --file <FILE> "Sets the problem input"
            )
            .default_value("input.txt")
            .value_parser(value_parser!(PathBuf)),
        )
        .arg(arg!(
            -v --verbose "Turn verbose logging on"
        ))
        .subcommand_required(true)
        .subcommand(Command::new("1").about("Part 1"))
        .subcommand(Command::new("2").about("Part 2"))
        .get_matches();

    // Parse input
    let input = matches
        .get_one::<PathBuf>("file")
        .map(|file_path| std::fs::read_to_string(file_path).expect("Could not read your input."))
        .expect("Expected file input to be provided");

    // Initialize debugger
    match *matches
        .get_one::<bool>("verbose")
        .expect("Count's are defaulted")
    {
        true => simple_logger::init_with_level(log::Level::Debug),
        false => simple_logger::init_with_level(log::Level::Info),
    }
    .unwrap();

    match matches.subcommand() {
        Some(("1", _sub_matches)) => println!("{}", part1::run(input)),
        Some(("2", _sub_matches)) => println!("{}", part2::run(input)),
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}
