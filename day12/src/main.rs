#![feature(drain_filter)]

pub mod node;
pub mod render;

use std::{
    collections::{HashMap, HashSet},
    fs,
    hash::Hash,
    process::exit,
};

use clap::{App, Arg};

fn create_or_insert<T: Eq + Hash + Clone + Copy>(graph: &mut HashMap<T, HashSet<T>>, k: T, v: T) {
    graph.insert(
        k,
        graph
            .get(&k)
            .map(|s| {
                let mut new = s.clone();
                new.insert(v);
                new
            })
            .unwrap_or_else(|| HashSet::from_iter(vec![v])),
    );
}

fn main() {
    let matches = App::new("Day 12: Passage Pathing")
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

    // let entry = Node::parse("start", &input);

    /* render_to(
        &entry,
        &mut File::create(format!("passage{}.dot", part)).unwrap(),
    ); */

    let mut graph = HashMap::<&str, HashSet<&str>>::new();

    for line in input.lines() {
        match line.split_once('-') {
            Some(("start", b)) => {
                create_or_insert(&mut graph, "start", b);
            }
            Some((a, "end")) => {
                create_or_insert(&mut graph, a, "end");
            }
            Some((a, b)) => {
                create_or_insert(&mut graph, a, b);
                create_or_insert(&mut graph, b, a);
            }
            _ => panic!("Unexpected input"),
        }
    }

    println!("{:?}", graph);
    match part {
        1 => part1(graph),
        2 => part2(graph),
        _ => panic!("Invalid part"),
    }

    exit(0);
}

type Path<'a> = Vec<&'a str>;

fn travel_path<'a>(
    from: &str,
    cur_path: Path<'a>,
    paths: &mut Vec<Path<'a>>,
    graph: &HashMap<&'a str, HashSet<&'a str>>,
    second_visited: bool,
) {
    if from == "end" {
        paths.push(cur_path);
    } else {
        for to in graph.get(from).unwrap().iter() {
            if *to == "start" {
                continue;
            }

            let is_upper = to.to_uppercase() == *to;
            let contains = cur_path.contains(to);
            if !second_visited && !is_upper && contains {
                travel_path(
                    *to,
                    {
                        let mut new_path = cur_path.clone();
                        new_path.push(to);
                        new_path
                    },
                    paths,
                    graph,
                    true,
                )
            }
            if is_upper || !contains {
                travel_path(
                    *to,
                    {
                        let mut new_path = cur_path.clone();
                        new_path.push(to);
                        new_path
                    },
                    paths,
                    graph,
                    second_visited,
                )
            }
        }
    }
}

fn part1(graph: HashMap<&str, HashSet<&str>>) {
    let mut paths = vec![];
    travel_path("start", vec!["start"], &mut paths, &graph, true);
    println!("Number of paths: {}", paths.len())
}

fn part2(graph: HashMap<&str, HashSet<&str>>) {
    let mut paths = vec![];
    travel_path("start", vec!["start"], &mut paths, &graph, false);
    for p in &paths {
        for c in p.iter() {
            if c.to_uppercase() == *c {
                continue;
            }
            if p.iter().fold(0u32, |a, n| a + ((n == c) as u32)) > 2 {
                println!("{}", p.join(","));
                break;
            }
        }
    }
    println!("Number of paths: {}", paths.len())
}
