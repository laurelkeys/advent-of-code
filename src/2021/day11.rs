//! --- Day 11: Dumbo Octopus ---

use crate::solver::Solver;
use std::{
    convert::TryInto,
    io::{self, BufRead, BufReader},
};

/// https://adventofcode.com/2021/day/11
pub struct Day11;

impl Solver for Day11 {
    type Input = [[u8; 10]; 10];
    type Output1 = usize;
    type Output2 = usize;

    /// There are 100 octopuses arranged neatly in a 10 by 10 grid. Each octopus
    /// slowly gains energy over time and flashes brightly for a moment when its
    /// energy is full. The energy level of each octopus is a value between 0 and 9.
    ///
    /// You can model the energy levels and flashes of light in steps.
    /// During a single step, the following occurs:
    /// - First, the energy level of each octopus increases by 1.
    /// - Then, any octopus with an energy level greater than 9 flashes.
    ///   This increases the energy level of all adjacent octopuses by 1, including
    ///   octopuses that are diagonally adjacent. If this causes an octopus to have
    ///   an energy level greater than 9, it also flashes.
    ///  (An octopus can only flash at most once per step.)
    /// - Finally, any octopus that flashed during this step has its energy level
    ///   set to 0, as it used all of its energy to flash.
    ///
    /// How many total flashes are there after 100 steps?
    fn solve_part1(&self, input: &Self::Input) -> Self::Output1 {
        println!("{:?}", &input);
        todo!()
    }

    ///
    fn solve_part2(&self, _input: &Self::Input) -> Self::Output2 {
        todo!()
    }

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        BufReader::new(r)
            .lines()
            .flatten()
            .map(|line| {
                line.chars()
                    .map(|c| c as u8 - b'0')
                    .collect::<Vec<u8>>()
                    .try_into()
                    .unwrap()
            })
            .collect::<Vec<[u8; 10]>>()
            .try_into()
            .unwrap()
    }
}
