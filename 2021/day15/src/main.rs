use std::{
    collections::{BinaryHeap, HashMap},
    fmt::Display,
    fs,
    process::exit,
    vec,
};

use clap::{App, Arg};

type P = (usize, usize);

struct Cave {
    risk_map: Vec<Vec<u8>>,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct PState {
    point: P,
    // f_score[P] is the best guess of how short a path is from start to finish if it goes
    // through n.
    // f_score[P] = g_score[P] + h(n)
    f_score: usize,
    risk: u8,
}

impl Ord for PState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.f_score
            .cmp(&other.f_score)
            .then_with(|| self.point.cmp(&other.point))
    }
}

impl PartialOrd for PState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.f_score.cmp(&self.f_score))
    }
}

impl Cave {
    pub fn new(input: &str) -> Self {
        Self {
            risk_map: input
                .lines()
                .map(|l| {
                    l.chars()
                        .map(|c| c.to_digit(10).unwrap() as u8)
                        .collect::<Vec<u8>>()
                })
                .collect(),
        }
    }

    pub fn dims(&self) -> (usize, usize) {
        (self.risk_map.len(), self.risk_map.first().unwrap().len())
    }

    fn reconstruct_path(&self, came_from: &HashMap<P, P>, end: &P) -> Vec<P> {
        let mut cur = *end;
        let mut path = vec![*end];
        while let Some(&before) = came_from.get(&cur) {
            cur = before;
            path.insert(0, before);
        }
        path
    }

    pub fn astar(&self, start: P, end: P, h: fn(&P, &P) -> usize) -> Vec<P> {
        let (n, m) = self.dims();

        // Discovered nodes to be re-expanded.
        // Implemented as a priority queue with those having the lowest `f`-score prioritized.
        let mut open = BinaryHeap::<PState>::new();
        open.push(PState {
            point: start,
            f_score: h(&start, &end),
            risk: self.risk_map[start.1][start.0],
        });

        // came_from[P] is the node preceding P on the cheapest path from `start`.
        let mut came_from = HashMap::<P, P>::new();

        // g_score[P] is the cost of the cheapest path from start to g currently known.
        let mut g_score = HashMap::<P, usize>::new();
        g_score.insert(start, 0);

        const NEIGHBOUR_IDXS: [(i64, i64); 4] = [(-1, 0), (1, 0), (0, 1), (0, -1)];

        let mut success = false;
        while let Some(cur) = open.pop() {
            if cur.point == end {
                success = true;
                break;
            }

            let neighbours: Vec<((usize, usize), u8)> = NEIGHBOUR_IDXS
                .iter()
                .map(|idx| (idx.0 + cur.point.0 as i64, idx.1 + cur.point.1 as i64))
                .filter(|idx| {
                    idx.0 >= 0 && idx.1 >= 0 && (idx.0 as usize) < m && (idx.1 as usize) < n
                })
                .map(|idx| {
                    let cidx = (idx.0 as usize, idx.1 as usize);
                    (cidx, self.risk_map[cidx.1][cidx.0])
                })
                .collect();

            for &(nb_idx, nb_score) in &neighbours {
                // distance from `start` to the `neighbour` through `cur`
                let tentative_g_score = g_score.get(&cur.point).unwrap_or(&usize::MAX)
                    + cur.risk as usize
                    + nb_score as usize;
                if tentative_g_score < *g_score.get(&nb_idx).unwrap_or(&usize::MAX) {
                    came_from.insert(nb_idx, cur.point);
                    g_score.insert(nb_idx, tentative_g_score);
                    // f_score.insert(nb_idx, tentative_g_score as usize + h(&nb_idx, &end));

                    if !open.iter().any(|e| e.point == nb_idx) {
                        open.push(PState {
                            point: nb_idx,
                            risk: nb_score,
                            f_score: tentative_g_score as usize + h(&nb_idx, &end),
                        });
                    }
                }
            }
        }
        if success {
            self.reconstruct_path(&came_from, &end)
        } else {
            panic!("Could not find a path!");
        }
    }

    pub fn display_with_path(&self, path: &[P]) {
        let mut pathed_risk_map = self.risk_map.clone();
        let mut total_risk = 0u64;

        for p in path[1..path.len()].iter() {
            total_risk += pathed_risk_map[p.1][p.0] as u64;
            pathed_risk_map[p.1][p.0] = 10; // (only single digits valid)
        }

        for row in &pathed_risk_map {
            println!(
                "{}",
                row.iter().fold(String::new(), |str, &p| {
                    str + &{
                        if p == 10 {
                            "•".to_string()
                        } else {
                            p.to_string()
                        }
                    }
                })
            );
        }

        println!("Total risk: {}", total_risk);
    }
}

impl Display for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res = Ok(());
        for row in &self.risk_map {
            res = res.and(writeln!(
                f,
                "{}",
                row.iter().fold(String::new(), |a, d| a + &d.to_string())
            ))
        }
        res
    }
}

fn main() {
    let matches = App::new("Day 15: Chiton")
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

    let cave = Cave::new(&input);

    match part {
        1 => part1(cave),
        2 => part2(cave),
        _ => panic!("Invalid part"),
    }

    exit(0);
}

fn sub_point(a: P, b: P) -> P {
    (a.0 - b.0, a.1 - b.1)
}

fn manhattan_dist(from: &P, to: &P) -> usize {
    to.0.checked_sub(from.0)
        .unwrap_or_else(|| from.0.checked_sub(to.0).unwrap())
        + to.1
            .checked_sub(from.1)
            .unwrap_or_else(|| from.1.checked_sub(to.1).unwrap())
}

fn part1(cave: Cave) {
    let path = cave.astar((0, 0), sub_point(cave.dims(), (1, 1)), |_, _| 0);
    cave.display_with_path(&path);
}

fn part2(_cave: Cave) {}
