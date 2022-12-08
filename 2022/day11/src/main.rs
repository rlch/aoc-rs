use std::{fmt::Display, fs, ops::IndexMut, process::exit};

use clap::{App, Arg};

struct Board {
    buffer: [u8; 100],
}

impl Board {
    fn new(input: &str) -> Self {
        Self {
            buffer: input
                .lines()
                .flat_map(|l| {
                    l.chars()
                        .map(|c| c.to_digit(10).unwrap() as u8)
                        .collect::<Vec<u8>>()
                })
                .collect::<Vec<u8>>()
                .try_into()
                .unwrap(),
        }
    }

    fn increment(&mut self) {
        for c in self.buffer.iter_mut() {
            *c += 1;
        }
    }

    fn flash(&mut self) -> u32 {
        let mut flashes = 0u32;
        let mut flash_queue: Vec<i8> = self
            .buffer
            .iter()
            .enumerate()
            .filter_map(|(i, &c)| if c == 10 { Some(i as i8) } else { None })
            .collect();

        while let Some(idx) = flash_queue.pop() {
            flashes += 1;
            assert!(self.buffer[idx as usize] >= 10);

            let adjacent: [i8; 8] = [
                idx - 11,
                idx - 10,
                idx - 9,
                idx - 1,
                idx + 1,
                idx + 9,
                idx + 10,
                idx + 11,
            ];

            let on_left = idx % 10 == 0;
            let on_right = idx % 10 == 9;
            let mut adj = vec![];
            for a in adjacent {
                if (on_left && a % 10 == 9) || (on_right && a % 10 == 0) {
                    continue;
                }
                if (0..100).contains(&a) {
                    adj.push(a);
                    let energy = self.buffer.index_mut(a as usize);
                    *energy += 1;

                    if *energy == 10 && !flash_queue.contains(&a) {
                        flash_queue.push(a);
                    }
                }
            }
        }

        flashes
    }

    fn reset_flashed(&mut self) {
        for c in self.buffer.iter_mut().filter(|x| **x >= 10) {
            *c = 0;
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res = Ok(());
        for row in self.buffer.chunks(10) {
            res = res.and(writeln!(
                f,
                "{}",
                row.iter()
                    .map(ToString::to_string)
                    .collect::<Vec<String>>()
                    .join("")
            ));
        }
        res
    }
}

fn main() {
    let matches = App::new("Day 11: Dumbo Octopus")
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

    let board = Board::new(&input);

    match part {
        1 => solve(board, false),
        2 => solve(board, true),
        _ => panic!("Invalid part"),
    }

    exit(0);
}

fn solve(mut board: Board, stop_at_sync: bool) {
    if stop_at_sync {
        let mut i = 0u32;
        loop {
            board.increment();
            let flashes = board.flash();
            board.reset_flashed();
            i += 1;

            if flashes == 100 {
                break;
            }
        }

        println!("Syncronized at {}", i);
    } else {
        let mut flashes = 0u32;
        for _ in 0..100 {
            board.increment();
            flashes += board.flash();
            board.reset_flashed();
        }
        println!("board:\n{}", board);
        println!("flashes: {}", flashes);
    }
}
