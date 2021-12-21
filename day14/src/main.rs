use std::{collections::HashMap, fs, process::exit};

use clap::{App, Arg};

fn main() {
    let matches = App::new("Day 14: Extended Polymerization")
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

    let template = input.lines().next().unwrap();
    let rules: HashMap<String, char> = input.lines().skip(2).fold(HashMap::new(), |mut hm, l| {
        let (bigram, elm) = l.split_once(" -> ").unwrap();
        hm.insert(bigram.to_string(), elm.chars().next().unwrap());
        hm
    });

    match part {
        1 => part1(template, rules),
        2 => part2(template, rules),
        _ => panic!("Invalid part"),
    }

    exit(0);
}

fn bigrams(polymer: &str) -> Vec<String> {
    polymer
        .chars()
        .zip(polymer.chars().skip(1))
        .map(|bg| [bg.0, bg.1].iter().collect())
        .collect()
}

fn step(polymer: String, rules: &HashMap<String, char>) -> String {
    let mut new_poly = "".to_string();
    for bg in &bigrams(&polymer) {
        if rules.contains_key(bg) {
            new_poly += &[bg.chars().next().unwrap(), *rules.get(bg).unwrap()]
                .iter()
                .collect::<String>();
        } else {
            new_poly += bg;
        }
    }
    new_poly + &polymer.chars().last().unwrap().to_string()
}

fn part1(template: &str, rules: HashMap<String, char>) {
    println!("Template: {}", template);

    let mut polymer = template.to_string();
    for _ in 0..10 {
        polymer = step(polymer, &rules);
        // println!("After step {}: {}", i + 1, polymer);
    }

    let mut counts = HashMap::<char, u32>::new();
    for c in polymer.chars() {
        counts.insert(c, counts.get(&c).unwrap_or(&0u32) + 1);
    }

    println!(
        "Diff: {}",
        counts.values().max().unwrap() - counts.values().min().unwrap()
    );
}

fn part2(_template: &str, _rules: HashMap<String, char>) {}
