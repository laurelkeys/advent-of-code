//! --- Day 10: Syntax Scoring ---

use crate::solver::Solver;
use std::io::{self, BufRead, BufReader};

/// https://adventofcode.com/2021/day/10
pub struct Day10;

impl Solver for Day10 {
    type Input = Vec<String>;
    type Output1 = usize;
    type Output2 = usize;

    /// The navigation subsystem syntax is made of several lines containing chunks.
    /// Every chunk must open and close with one of four legal matching pairs: (), [], {}, <>.
    /// Some lines are incomplete, but others are corrupted. A corrupted line is one where a
    /// chunk closes with the wrong character. Find and discard the corrupted lines first.
    ///
    /// To calculate the syntax error score for a line, take the first illegal character on
    /// the line and look it up in the following table:
    /// - ) : 3 points.
    /// - ] : 57 points.
    /// - } : 1197 points.
    /// - > : 25137 points.
    ///
    /// Find the first illegal character in each corrupted line of the navigation subsystem.
    /// What is the total syntax error score for those errors?
    fn solve_part1(&self, input: &Self::Input) -> Self::Output1 {
        println!("{:?}", &input);
        todo!()
    }

    ///
    fn solve_part2(&self, _input: &Self::Input) -> Self::Output2 {
        todo!()
    }

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        BufReader::new(r).lines().flatten().collect::<Vec<String>>()
    }
}
