//! --- Day 24: Lobby Layout ---

use crate::solver::Solver;
use lazy_static::lazy_static;
use std::{
    collections::{HashMap, HashSet},
    io::{self, BufRead, BufReader},
    str::FromStr,
};

/// https://adventofcode.com/2020/day/24
pub struct Day24;

#[derive(Debug)]
pub enum Neighbor {
    E,  // east
    Se, // southeast
    Sw, // southwest
    W,  // west
    Nw, // northwest
    Ne, // northeast
}

use Neighbor::*;
const NEIGHBORS: [Neighbor; 6] = [E, Se, Sw, W, Nw, Ne];

fn flip(tile: (i64, i64), tiles: &mut HashSet<(i64, i64)>) {
    if tiles.contains(&tile) {
        tiles.remove(&tile);
    } else {
        tiles.insert(tile);
    }
}

impl Solver for Day24 {
    type Input = Vec<Vec<Neighbor>>;
    type Output1 = usize;
    type Output2 = usize;

    fn solve_part1(&self, input: &Self::Input) -> Self::Output1 {
        let mut black_tiles: HashSet<(i64, i64)> = HashSet::new();

        for instruction in input {
            flip(
                instruction
                    .iter()
                    .fold((0, 0), |tile, neighbor| neighbor.of(tile)),
                &mut black_tiles,
            );
        }

        // How many tiles are left with the black side up?
        black_tiles.len()
    }

    fn solve_part2(&self, input: &Self::Input) -> Self::Output2 {
        let mut black_tiles: HashSet<(i64, i64)> = HashSet::new();

        for instruction in input {
            flip(
                instruction
                    .iter()
                    .fold((0, 0), |tile, neighbor| neighbor.of(tile)),
                &mut black_tiles,
            );
        }

        // Every day, the tiles are all flipped according to the following rules:
        //  - Any black tile with zero or more than 2 adjacent black tiles.
        //  - Any white tile with exactly 2 adjacent black tiles.
        for _ in 1..=100 {
            let mut black_neighbors: HashMap<(i64, i64), usize> = black_tiles
                .iter()
                .map(|&black_tile| (black_tile, 0))
                .collect();

            for black_tile in &black_tiles {
                NEIGHBORS
                    .iter()
                    .map(|neighbor| neighbor.of(*black_tile))
                    .for_each(|tile| *black_neighbors.entry(tile).or_insert(0) += 1);
            }

            let tiles_to_flip: HashSet<(i64, i64)> = black_neighbors
                .iter()
                .flat_map(|(&tile, &count)| match black_tiles.contains(&tile) {
                    true if count == 0 || count > 2 => Some(tile),
                    false if count == 2 => Some(tile),
                    _ => None,
                })
                .collect();

            for tile in tiles_to_flip {
                flip(tile, &mut black_tiles);
            }
        }

        // How many tiles will be black after 100 days?
        black_tiles.len()
    }

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        use regex::Regex; // https://docs.rs/regex/1.4.2/regex/#syntax

        lazy_static! {
            static ref INSTR_RE: Regex = Regex::new(r"(e|se|sw|w|nw|ne)").unwrap();
        };

        BufReader::new(r)
            .lines()
            .flatten()
            .map(|instructions| {
                INSTR_RE
                    .captures_iter(&instructions)
                    .map(|cap| Neighbor::from_str(&cap[0]).unwrap())
                    .collect()
            })
            .collect()
    }
}

impl Neighbor {
    fn of(&self, tile: (i64, i64)) -> (i64, i64) {
        let (x, y) = tile;
        match self {
            E => (x + 2, y),
            Se => (x + 1, y - 1),
            Sw => (x - 1, y - 1),
            W => (x - 2, y),
            Nw => (x - 1, y + 1),
            Ne => (x + 1, y + 1),
        }
    }
}

impl FromStr for Neighbor {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "e" => E,
            "se" => Se,
            "sw" => Sw,
            "w" => W,
            "nw" => Nw,
            "ne" => Ne,
            _ => return Err(()),
        })
    }
}
