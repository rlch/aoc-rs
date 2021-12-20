#![feature(drain_filter)]

pub mod node;
pub mod render;

use std::{
    collections::HashMap,
    fs::{self, File},
    process::exit,
};

use clap::{App, Arg};
use node::Node;
use render::render_to;

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

    let entry = Node::parse("start", &input);

    render_to(
        &entry,
        &mut File::create(format!("passage{}.dot", part)).unwrap(),
    );

    match part {
        1 => part1(entry),
        2 => part2(entry),
        _ => panic!("Invalid part"),
    }

    exit(0);
}

type Path<'a> = Vec<&'a str>;

fn travel_paths<'a>(
    from: &'a Node,
    seen_small: &mut Vec<&'a str>,
    last_big: Option<&'a Node>,
    graph: &'a HashMap<&str, Vec<&Node<'a>>>,
) -> Vec<Path<'a>> {
    let mut paths = vec![vec![from.label]];

    let mut new_seen = seen_small.clone();
    if !from.big {
        new_seen.push(from.label);
    }

    // Check if we're able to return back to the last big cave
    // Check for traversable paths in all child nodes
    for to in &graph[from.label] {
        let mut target_paths = Vec::<Path>::new();
        if !new_seen.contains(&to.label) || to.big {
            target_paths.extend(travel_paths(
                to,
                &mut new_seen,
                {
                    if from.big {
                        Some(from)
                    } else {
                        None
                    }
                },
                graph,
            ));
        }

        for tp in target_paths {
            let mut full_path = vec![from.label];
            full_path.extend(tp);
            paths.push(full_path);
        }
    }

    // Check if we're able to return back to the last big cave
    /* if let Some(big) = last_big {
        paths.extend(travel_paths(big, &mut new_seen, None));
    } */

    /* // Cull invalid paths
    paths.drain_filter(|p| p.last().unwrap_or(&"") == &"end"); */

    paths
}

fn part1(entry: Node) {
    // We need to convert to a cyclic graph. CBF refactoring the whole data structure lol
    let mut graph = HashMap::<&str, Vec<&Node>>::new();
    entry.dfs(&move |n| {
        graph.insert(n.label, n.targets.iter().collect());
        for t in &n.targets {
            if graph.contains_key(t.label) {
                graph[t.label].push(n);
            } else {
                graph[t.label] = vec![n];
            }
        }
    });

    let paths = travel_paths(&entry, &mut vec!["start"], None, &graph);

    for path in paths {
        println!("{}", path.join(","))
    }
}

fn part2(_entry: Node) {}
