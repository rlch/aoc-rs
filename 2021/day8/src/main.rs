#![feature(int_abs_diff)]

use clap::{App, Arg};
use itertools::Itertools;
use std::{collections::HashSet, fs, process::exit};
use tuple_map::*;

fn parse(line: &str) -> (Vec<&str>, Vec<&str>) {
    line.split_once('|')
        .unwrap()
        .map(|i| i.split_whitespace().collect::<Vec<&str>>())
}

fn main() {
    let matches = App::new("Day 8: Seven Segment Search")
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
    let mut count = 0u32;

    for line in input.lines() {
        let io = parse(line);
        for o in io.1 {
            match o.len() {
                2 | 3 | 4 | 7 => count += 1,
                _ => continue,
            }
        }
    }

    println!("count: {}", count);
}

const CANONICAL: [u8; 10] = [
    0b1110111, 0b0010010, 0b1011101, 0b1011011, 0b0111010, 0b1101011, 0b1101111, 0b1010010,
    0b1111111, 0b1111011,
];
const ALPHABET: [char; 7] = ['a', 'b', 'c', 'd', 'e', 'f', 'g'];

fn encode(word: &str, configuration: &[char; 7]) -> u8 {
    let mut bin = 0u8;
    for c in word.chars() {
        let i: usize = configuration
            .iter()
            .position(|&x| x == c)
            .expect("Unexpected character.");
        bin += 1u8 << i;
    }
    bin
}

fn part2(input: String) {
    let canonical_set: HashSet<u8> = CANONICAL.iter().cloned().collect();

    let mut sum = 0usize;
    for line in input.lines() {
        let io = parse(line);

        let configuration: [char; 7] = {
            let mut correct: Option<[char; 7]> = None;
            for conf_vec in ALPHABET.iter().cloned().permutations(7) {
                let configuration: [char; 7] = conf_vec
                    .try_into()
                    .expect("Expected configuration to be of length 7.");

                let encoded_set: HashSet<u8> =
                    io.0.iter()
                        .map(|&word| encode(word, &configuration))
                        .collect();

                if canonical_set == encoded_set {
                    println!(
                        "config: {:?}, word: {}, encoded: {:#09b}",
                        configuration,
                        io.0[0],
                        encode(io.0[0], &configuration)
                    );
                    correct = Some(configuration);
                    break;
                }
            }
            correct.expect("Could not find the correct configuration.")
        };

        let output =
            io.1.iter()
                .map(|&entry| {
                    CANONICAL
                        .iter()
                        .position(|&bin| bin == encode(entry, &configuration))
                        .unwrap()
                        .to_string()
                })
                .collect::<String>();
        println!("{}: {}", io.1.iter().join(" "), output,);

        sum += output.parse::<usize>().unwrap();
    }
    println!("sum: {}", sum);
}
