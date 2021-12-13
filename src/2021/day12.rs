//! --- Day 12: Passage Pathing ---

use crate::solver::Solver;
use std::{
    collections::HashMap,
    io::{self, BufRead, BufReader},
};

/// https://adventofcode.com/2021/day/12
pub struct Day12;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Cave {
    Start,
    End,
    Small(String),
    Big(String),
}

impl Cave {
    fn new(cave: &str) -> Self {
        if cave.chars().any(|c| c.is_uppercase()) {
            Cave::Big(cave.to_string())
        } else if cave == "start" {
            Cave::Start
        } else if cave == "end" {
            Cave::End
        } else {
            Cave::Small(cave.to_string())
        }
    }
}

fn paths_from(cave: &Cave, connections: &HashMap<Cave, Vec<Cave>>, visited: &[Cave]) -> usize {
    match *cave {
        Cave::End => 1,
        Cave::Start => 0,
        Cave::Small(_) => visited.contains(cave).then(|| 0).unwrap_or({
            let mut visited = visited.to_vec();
            visited.push(cave.clone());
            connections.get(cave).map_or(0, |caves| {
                caves
                    .iter()
                    .filter(|&next_cave| !visited.contains(next_cave))
                    .fold(0, |paths, next_cave| {
                        paths + paths_from(next_cave, connections, &visited)
                    })
            })
        }),
        Cave::Big(_) => connections.get(cave).map_or(0, |caves| {
            caves
                .iter()
                .filter(|&next_cave| !visited.contains(next_cave))
                .fold(0, |paths, next_cave| {
                    paths + paths_from(next_cave, connections, visited)
                })
        }),
    }
}

impl Solver for Day12 {
    type Input = HashMap<Cave, Vec<Cave>>;
    type Output1 = usize;
    type Output2 = usize;

    /// Your goal is to find the number of distinct paths that start at start,
    /// end at end, and don't visit small caves more than once, and can visit
    /// big caves any number of times.
    fn solve_part1(&self, input: &Self::Input) -> Self::Output1 {
        input[&Cave::Start].iter().fold(0, |paths, cave| {
            paths + paths_from(cave, input, &[Cave::Start])
        })
    }

    /// After reviewing the available paths, you realize you might have time to
    /// visit a single small cave twice. Specifically, big caves can be visited
    /// any number of times, a single small cave can be visited at most twice,
    /// and the remaining small caves can be visited at most once.
    /// However, the caves named start and end can only be visited exactly once each.
    ///
    /// Given these new rules, how many paths through this cave system are there?
    fn solve_part2(&self, input: &Self::Input) -> Self::Output2 {
        todo!()
    }

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        let mut connections = HashMap::new();
        for line in BufReader::new(r).lines().flatten() {
            let (from, to) = line.split_once('-').unwrap();
            let (from, to) = (Cave::new(from), Cave::new(to));
            connections
                .entry(from.clone())
                .or_insert_with(Vec::new)
                .push(to.clone());
            connections.entry(to).or_insert_with(Vec::new).push(from);
        }

        connections
    }
}
