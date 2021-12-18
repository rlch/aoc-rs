use std::{collections::HashSet, fs, process::exit, vec};

use clap::{App, Arg};

fn main() {
    let matches = App::new("Day 9: Smoke Basin")
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
        1 => part1(parse(&input)),
        2 => part2(parse(&input)),
        _ => panic!("Invalid part"),
    }

    exit(0);
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn part1(heightmap: Vec<Vec<char>>) {
    let mut total_risk_level = 0u32;
    let n = heightmap.len();
    let m = heightmap[0].len();

    for (i, row) in heightmap.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            let mut lower = true;
            lower &= i == 0 || heightmap[i - 1][j] > c;
            lower &= i >= n - 1 || heightmap[i + 1][j] > c;
            lower &= j == 0 || heightmap[i][j - 1] > c;
            lower &= j >= m - 1 || heightmap[i][j + 1] > c;
            if lower {
                total_risk_level += 1 + c.to_digit(10).unwrap();
            }
        }
    }

    println!("Risk level: {}", total_risk_level);
}

fn part2(heightmap: Vec<Vec<char>>) {
    let n = heightmap.len();
    let m = heightmap[0].len();

    let mut seen = HashSet::<(usize, usize)>::new();
    let mut basins: Vec<Vec<(usize, usize)>> = vec![];

    let try_expand = |at: (usize, usize), queue: &mut Vec<(usize, usize)>| {
        if heightmap[at.0][at.1] != '9' && !queue.contains(&at) {
            queue.push(at);
        }
    };
    for (i, row) in heightmap.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            let mut queue_index = 0;
            let pos = (i, j);
            if c == '9' || seen.contains(&pos) {
                continue;
            }

            let mut pos_queue = vec![pos];
            while pos_queue.len() != queue_index {
                let (i, j) = pos_queue[queue_index];
                seen.insert((i, j));

                if i > 0 {
                    try_expand((i - 1, j), &mut pos_queue);
                }
                if i < n - 1 {
                    try_expand((i + 1, j), &mut pos_queue);
                }
                if j > 0 {
                    try_expand((i, j - 1), &mut pos_queue);
                }
                if j < m - 1 {
                    try_expand((i, j + 1), &mut pos_queue);
                }
                queue_index += 1;
            }
            basins.push(pos_queue);
        }
    }

    let mut sorted_lengths = basins.iter().map(|b| b.len()).collect::<Vec<_>>();
    sorted_lengths.sort_by(|a, &b| b.cmp(a));

    println!("Sizes: {:?}", sorted_lengths);

    println!(
        "Output: {}",
        sorted_lengths.iter().take(3).product::<usize>()
    );
}
