#![feature(drain_filter)]
use std::{fmt::Display, fs, process::exit, usize};

use clap::{App, Arg};
use regex::Regex;

#[derive(Clone)]
struct Paper {
    dots: Vec<Dot>,
    instructions: Vec<Fold>,
    dims: (usize, usize),
}

type Dot = (usize, usize);

#[derive(Clone)]
enum Fold {
    X(usize),
    Y(usize),
}

impl Display for Fold {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Fold along {}",
            match self {
                Fold::X(x) => format!("x={}", x),
                Fold::Y(y) => format!("y={}", y),
            }
        )
    }
}

#[derive(Debug)]
enum FoldError {
    NoInstructions,
}

impl Paper {
    pub fn new(input: &str) -> Self {
        let mut n_dots = 0;
        let parse_line = |line: &str| -> Dot {
            let pos = line.split_once(',').unwrap();
            n_dots += 1;
            (
                pos.0.parse::<usize>().unwrap(),
                pos.1.parse::<usize>().unwrap(),
            )
        };

        let dots: Vec<Dot> = input
            .lines()
            .take_while(|l| !l.is_empty())
            .map(parse_line)
            .collect();

        let (m, n) = dots
            .iter()
            .fold((0, 0), |a, p| (a.0.max(p.0 + 1), a.1.max(p.1 + 1)));

        let re = Regex::new(r"(x|y)=(\d+)").unwrap();
        let instructions = input
            .lines()
            .skip(n_dots + 1)
            .map(|l| -> Fold {
                let caps = re.captures(l).unwrap();
                let num = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
                if caps.get(1).unwrap().as_str() == "x" {
                    Fold::X(num)
                } else {
                    Fold::Y(num)
                }
            })
            .collect();

        Self {
            dots,
            instructions,
            dims: (n, m),
        }
    }

    pub fn fold(&self) -> Result<Paper, FoldError> {
        let mut instructions = self.instructions.clone();
        if instructions.is_empty() {
            return Err(FoldError::NoInstructions);
        }

        let ins = instructions.remove(0);

        let (n, m) = self.dims;
        let (mut dots, dims): (Vec<Dot>, (usize, usize)) = match ins {
            Fold::X(f) => {
                let mut dots = self.dots.clone();
                for dot in &mut dots {
                    if dot.0 >= f {
                        dot.0 = 2 * f - dot.0;
                    }
                }
                (dots, (n, m / 2))
            }
            Fold::Y(f) => {
                let mut dots = self.dots.clone();
                for dot in &mut dots {
                    if dot.1 >= f {
                        dot.1 = 2 * f - dot.1;
                    }
                }
                (dots, (n / 2, m))
            }
        };
        dots.sort_unstable();
        dots.dedup();

        Ok(Paper {
            dots,
            instructions,
            dims,
        })
    }

    pub fn fold_all(&self) -> Paper {
        let mut paper: Paper = self.clone();
        while !paper.instructions.is_empty() {
            paper = paper.fold().unwrap();
        }
        paper
    }
}

impl Display for Paper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res = Ok(());
        let buffer =
            self.dots
                .iter()
                .fold(vec![vec![false; self.dims.1]; self.dims.0], |mut buf, d| {
                    buf[d.1][d.0] = true;
                    buf
                });

        for row in buffer {
            res = res.and(writeln!(
                f,
                "{}",
                row.iter()
                    .map(|d| if *d { "#" } else { "." })
                    .collect::<String>()
            ));
        }

        res = res.and(writeln!(f, "\nInstructions:"));
        for i in &self.instructions {
            res = res.and(write!(f, "{}", i))
        }
        res
    }
}

fn main() {
    let matches = App::new("Day 13: Transparent Origami")
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

    let paper = Paper::new(&input);

    match part {
        1 => part1(paper),
        2 => part2(paper),
        _ => panic!("Invalid part"),
    }

    exit(0);
}

fn part1(paper: Paper) {
    let folded = paper.fold().unwrap();

    println!("{}", folded);

    println!("Dots visible: {}", folded.dots.len());
}

fn part2(paper: Paper) {
    let folded = paper.fold_all();
    println!("{}", folded);
}
