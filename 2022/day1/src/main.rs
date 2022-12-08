use clap::{App, Arg};
use std::{fs, process::exit, usize};

fn main() {
    let matches = App::new("Day 1: Sonar Sweep")
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
            Arg::new("batch")
                .short('b')
                .long("batch")
                .takes_value(true)
                .value_name("INT")
                .default_value("1"),
        )
        .get_matches();

    let batch_size: usize = matches
        .value_of("batch")
        .unwrap()
        .parse()
        .expect("Expected batch size to be a positive integer.");

    let input =
        fs::read_to_string(matches.value_of("file").unwrap()).expect("Could not read your input.");

    let mut batch_sums = Vec::<u32>::new();

    let mut curr_sums = vec![0; batch_size];
    for (n, line) in input.lines().enumerate() {
        let depth = line.parse::<u32>().unwrap();
        for i in 0..batch_size {
            if n.checked_sub(i) == None {
                continue;
            }
            curr_sums[(n - i) % 3] += depth;
        }

        if n + 1 >= batch_size {
            batch_sums.push(curr_sums[(n + 1) % 3]);
            curr_sums[(n + 1) % 3] = 0;
        }
        // println!("n: {}, curr: {:?}, batch: {:?}", n, curr_sums, batch_sums);
    }

    // println!("{:?}", batch_sums);

    let mut prev = u32::MAX;
    let increases = batch_sums.iter().fold(0, |i, sum| {
        let increased = sum > &prev;
        prev = *sum;
        i + if increased { 1 } else { 0 }
    });
    println!("The depth increased {} times.", increases);
    exit(0);
}
