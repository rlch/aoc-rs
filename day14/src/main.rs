use std::{cmp::Eq, collections::HashMap, fs, hash::Hash, process::exit};

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

fn step_basic(polymer: String, rules: &HashMap<String, char>) -> String {
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
        polymer = step_basic(polymer, &rules);
        // println!("After step {}: {}", i + 1, polymer);
    }

    let mut counts = HashMap::<char, u32>::new();
    for c in polymer.chars() {
        counts.insert(c, counts.get(&c).unwrap_or(&0u32) + 1);
    }

    let bigrams = bigrams(&polymer);
    let mut bigram_counts = HashMap::<String, u32>::new();
    for bg in &bigrams {
        bigram_counts.insert(bg.to_string(), bigram_counts.get(bg).unwrap_or(&0) + 1);
    }

    println!("{:?}", bigram_counts);
    println!("{:?}", counts);

    println!(
        "Diff: {}",
        counts.values().max().unwrap() - counts.values().min().unwrap()
    );
}

fn incremental_insert<K: Eq + Hash + Clone>(k: &K, hm: &mut HashMap<K, u64>, by: u64) {
    hm.insert(k.clone(), hm.get(k).unwrap_or(&0) + by);
}

fn step_eff(
    bigram_counts: &mut HashMap<String, u64>,
    rules: &HashMap<String, char>,
) -> HashMap<char, u64> {
    let mut char_counts = HashMap::<char, u64>::new();
    for (bg, count) in bigram_counts.clone() {
        // println!("before: {:?} with bg: {}", bigram_counts, bg);
        if let Some(&c) = rules.get(&bg) {
            incremental_insert(
                &[bg.chars().next().unwrap(), c].iter().collect(),
                bigram_counts,
                count,
            );
            incremental_insert(
                &[c, bg.chars().nth(1).unwrap()].iter().collect(),
                bigram_counts,
                count,
            );
            bigram_counts.insert(
                bg.to_string(),
                bigram_counts
                    .get(&bg)
                    .unwrap()
                    .checked_sub(count)
                    .unwrap_or(0),
            );
            char_counts.insert(c, char_counts.get(&c).unwrap_or(&0) + count);
        }
        // println!("after: {:?}", bigram_counts);
    }
    char_counts
}

fn part2(template: &str, rules: HashMap<String, char>) {
    let bigrams = bigrams(template);
    let mut bigram_counts = HashMap::<String, u64>::new();
    for bg in &bigrams {
        bigram_counts.insert(bg.to_string(), bigram_counts.get(bg).unwrap_or(&0) + 1);
    }
    let mut char_counts = bigram_counts
        .iter()
        .fold(HashMap::<char, u64>::new(), |mut hm, bg| {
            let a = bg.0.chars().next().unwrap();
            let b = bg.0.chars().nth(1).unwrap();

            hm.insert(a, hm.get(&a).unwrap_or(&0) + bg.1);
            hm.insert(b, hm.get(&b).unwrap_or(&0) + bg.1);

            hm
        });

    for _ in 0..10 {
        for (char, count) in step_eff(&mut bigram_counts, &rules) {
            char_counts.insert(char, char_counts.get(&char).unwrap_or(&0) + count);
        }
    }

    println!("{:?}", bigram_counts);
    println!("{:?}", char_counts);

    println!(
        "Diff: {}",
        char_counts.values().max().unwrap() - char_counts.values().min().unwrap()
    );
}
