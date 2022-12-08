#![feature(int_abs_diff)]

use clap::{App, Arg};
use std::{fs, process::exit};

fn main() {
    let matches = App::new("Day 7: The Treachery of Whales")
        .version("1.0.0")
        .about("Advent of Code 2021")
        .arg(
            Arg::new("file")
                .help("Relative location of file containing input.")
                .value_name("FILE")
                .index(1)
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::new("part")
                .short('p')
                .long("part")
                .takes_value(true)
                .default_value("1"),
        )
        .get_matches();

    let input =
        fs::read_to_string(matches.value_of("file").unwrap()).expect("Could not read your input.");
    let part = matches.value_of("part").unwrap().parse::<u32>().unwrap();

    match part {
        1 => part1(input),
        2 => part2(input),
        _ => panic!("Invalid part"),
    }

    exit(0);
}

fn part1(input: String) {
    let positions = input
        .split(',')
        .map(|f| f.trim().parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    let m = median(&positions);

    println!(
        "median: {}, fuel spent: {}",
        m,
        positions
            .iter()
            .fold(0u32, |acc, curr| acc + curr.abs_diff(m))
    );
}

fn part2(input: String) {
    let positions = input
        .split(',')
        .map(|f| f.trim().parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    let min = positions
        .iter()
        .fold(0u32, |acc, x| if x < &acc { *x } else { acc });
    let max = positions
        .iter()
        .fold(0u32, |acc, x| if x > &acc { *x } else { acc });

    let mut lowest = u32::MAX;
    for m in min..max {
        lowest = lowest.min(positions.iter().fold(0u32, |acc, x| acc + sum_to_n(x.abs_diff(m))))
    }

    println!("fuel: {}", lowest);
}

fn median(arr: &[u32]) -> u32 {
    let mut sorted = arr.to_owned();
    sorted.sort_unstable();

    let n = sorted.len();
    if n % 2 == 0 {
        sorted[n / 2]
    } else {
        (sorted[n / 2 - 1] + sorted[n / 2]) / 2
    }
}

fn sum_to_n(n: u32) -> u32 {
    n * (n + 1) / 2
}
