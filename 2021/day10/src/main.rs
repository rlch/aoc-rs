use std::{fs, process::exit, vec};

use clap::{App, Arg};

fn main() {
    let matches = App::new("Day 10: Syntax Scoring")
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
        1 => part1(&input),
        2 => part2(&input),
        _ => panic!("Invalid part"),
    }

    exit(0);
}

fn is_open(token: &char) -> bool {
    ['(', '[', '\u{007b}', '<'].contains(token)
}

fn part1(input: &str) {
    let mut score = 0;
    for line in input.lines() {
        let mut queue = vec![];
        for token in line.chars() {
            if is_open(&token) {
                queue.push(token);
            } else if !match queue.pop() {
                Some('(') => token == ')',
                Some('[') => token == ']',
                Some('\u{007b}') => token == '\u{007d}',
                Some('<') => token == '>',
                _ => panic!("invalid token"),
            } {
                score += match token {
                    ')' => 3,
                    ']' => 57,
                    '\u{007d}' => 1197,
                    '>' => 25137,
                    _ => panic!("invalid token"),
                };
                break;
            }
        }
    }
    println!("Score: {}", score)
}

fn part2(input: &str) {
    let mut incomplete = vec![];

    for line in input.lines() {
        let mut queue = vec![];

        let mut ok = true;
        for token in line.chars() {
            if is_open(&token) {
                queue.push(token);
            } else if !match queue.pop() {
                Some('(') => token == ')',
                Some('[') => token == ']',
                Some('\u{007b}') => token == '\u{007d}',
                Some('<') => token == '>',
                _ => panic!("invalid token"),
            } {
                ok = false;
                break;
            }
        }
        if ok {
            incomplete.push(line);
        }
    }

    let mut scores: Vec<usize> = vec![0; incomplete.len()];
    for (i, &line) in incomplete.iter().enumerate() {
        let mut queue = vec![];

        for token in line.chars() {
            if is_open(&token) {
                queue.push(token);
            } else {
                queue.pop();
            }
        }

        println!("Queue: {:?}", queue);

        for leftover in queue.iter().rev() {
            let score = scores.get_mut(i).unwrap();
            *score *= 5;
            match *leftover {
                '(' => *score += 1,
                '[' => *score += 2,
                '\u{007b}' => *score += 3,
                '<' => *score += 4,
                _ => panic!("invalid token"),
            };
        }
    }

    scores.sort_unstable();

    println!("Middle score: {:?}", scores[scores.len() / 2])
}
