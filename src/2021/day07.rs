//! --- Day 7: The Treachery of Whales ---

use crate::solver::Solver;
use std::{
    collections::HashMap,
    io::{self, BufRead, BufReader},
};

/// https://adventofcode.com/2021/day/7
pub struct Day07;

impl Solver for Day07 {
    type Input = Vec<i32>;
    type Output1 = i32;
    type Output2 = i32;

    /// Determine the horizontal position that the crabs can align to using the
    /// least fuel possible. How much fuel must they spend to align to that position?
    fn solve_part1(&self, input: &Self::Input) -> Self::Output1 {
        let mut crabs_per_pos = HashMap::<i32, i32>::new();
        for &pos in input {
            *crabs_per_pos.entry(pos).or_insert(0) += 1;
        }

        let costs = crabs_per_pos
            .keys()
            .map(|&align_pos| {
                crabs_per_pos
                    .iter()
                    .map(|(&other_pos, &count)| count * (align_pos - other_pos).abs())
                    .sum()
            })
            .collect::<Vec<i32>>();

        *costs.iter().min().unwrap()
    }

    /// As it turns out, crab submarine engines don't burn fuel at a constant rate.
    /// Each change of 1 step in horizontal position costs 1 more unit of fuel than the last.
    /// Determine the horizontal position that the crabs can align to using the least fuel
    /// possible so they can make you an escape route!
    /// How much fuel must they spend to align to that position?
    fn solve_part2(&self, input: &Self::Input) -> Self::Output2 {
        let mut crabs_per_pos = HashMap::<i32, i32>::new();
        for &pos in input {
            *crabs_per_pos.entry(pos).or_insert(0) += 1;
        }

        // @Note: not sure if this wasn't needed in the first case by luck, or if the
        // instructions weren't clear enough... but it worked without it ¯\_(ツ)_/¯.
        let (min, max) = input[1..]
            .iter()
            .fold((input[0], input[0]), |(min, max), &pos| {
                if pos < min {
                    (pos, max)
                } else if pos > max {
                    (min, pos)
                } else {
                    (min, max)
                }
            });

        let costs = (min..=max)
            .map(|align_pos| {
                crabs_per_pos
                    .iter()
                    .map(|(&other_pos, &count)| {
                        let steps = (align_pos - other_pos).abs();
                        // If a crab walks n positions to align, then:
                        // 1 + 2 + ... + n = n * (n + 1) / 2.
                        count * (steps * steps + steps) / 2
                    })
                    .sum()
            })
            .collect::<Vec<i32>>();

        *costs.iter().min().unwrap()
    }

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        BufReader::new(r)
            .lines()
            .flatten()
            .flat_map(|line| {
                line.split(',')
                    .map(|age| age.parse().unwrap())
                    .collect::<Vec<i32>>()
            })
            .collect()
    }
}
