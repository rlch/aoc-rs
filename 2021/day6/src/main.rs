pub mod lanternfish;

use clap::{App, Arg};
use lanternfish::Lanternfish;
use std::{fs, process::exit};

fn main() {
    let matches = App::new("Day 6: Lanternfish")
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
            Arg::new("days")
                .short('d')
                .long("days")
                .value_name("INT")
                .required(true)
                .takes_value(true),
        )
        .arg(Arg::new("verbose").short('v').long("verbose"))
        .get_matches();

    let input =
        fs::read_to_string(matches.value_of("file").unwrap()).expect("Could not read your input.");
    let days = matches.value_of("days").unwrap().parse::<u32>().unwrap();
    // let verbose = matches.is_present("verbose");

    solve_smart(input, days as usize);

    exit(0);
}

fn solve_smart(input: String, days: usize) {
    let mut counter = vec![0u64; days + 9];
    let start_fish = input
        .split(',')
        .map(|f| f.trim().parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    for f in &start_fish {
        counter[*f] += 1;
    }
    let mut num_fish = start_fish.len();

    for i in 0..days {
        let new = counter[i];
        counter[i + 7] += new;
        counter[i + 9] += new;
        num_fish += new as usize;
    }
    println!("num_fish: {}", num_fish);
}

fn solve_dumb(input: String, days: u32, verbose: bool) {
    let mut fishes = Vec::<Lanternfish>::new();

    for start_timer in input.split(',') {
        fishes.push(Lanternfish::new(start_timer.trim().parse::<u32>().unwrap()));
    }

    if verbose {
        println!(
            "Initial state: {}",
            fishes
                .iter()
                .map(|f| f.timer.to_string())
                .collect::<Vec<String>>()
                .join(",")
        );
    }

    for i in 1..=days {
        for fish in &mut fishes {
            fish.end_day();
        }

        let joined = fishes
            .iter()
            .map(|f| f.timer.to_string())
            .collect::<Vec<String>>()
            .join(",");

        if verbose {
            println!(
                "After {: <width$} day{} {} ({})",
                i.to_string(),
                if i == 1 { ": " } else { "s:" },
                joined,
                fishes.len(),
                width = days.to_string().len()
            );
        } else {
            println!("Day {}: {} fishies", i, fishes.len());
        }

        let mut new_fish = Vec::<Lanternfish>::new();
        for fish in &mut fishes {
            if let Some(f) = fish.start_new_day() {
                new_fish.push(f)
            }
        }
        fishes.extend(new_fish);
    }
}
