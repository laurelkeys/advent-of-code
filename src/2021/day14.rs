//! --- Day 14: Extended Polymerization ---

use crate::solver::Solver;
use std::io::{self, BufRead, BufReader};

/// https://adventofcode.com/2021/day/14
pub struct Day14;

pub struct Polymer(Vec<char>);

pub struct InsertionRule {
    pair: (char, char),
    element: char,
}

impl Solver for Day14 {
    type Input = (Polymer, Vec<InsertionRule>);
    type Output1 = i32;
    type Output2 = i32;

    ///
    fn solve_part1(&self, input: &Self::Input) -> Self::Output1 {
        todo!()
    }

    ///
    fn solve_part2(&self, input: &Self::Input) -> Self::Output2 {
        todo!()
    }

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        todo!()
    }
}
