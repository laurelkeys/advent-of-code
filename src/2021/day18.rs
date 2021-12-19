//! --- Day 18: Snailfish ---

use crate::solver::Solver;
use std::io::{self, BufRead, BufReader};

/// https://adventofcode.com/2021/day/18
pub struct Day18;

impl Solver for Day18 {
    type Input = Vec<String>;
    type Output1 = i64;
    type Output2 = i64;

    /// Pairs are written as [x,y], where x and y are the elements within the pair.
    /// Each element of the pair can be either a regular number or another pair.
    ///
    /// To add two snailfish numbers, form a pair from the left and right parameters
    /// of the addition operator. There's only one problem: snailfish numbers must
    /// always be reduced, and the process of adding two snailfish numbers can result
    /// in snailfish numbers that need to be reduced.
    ///
    /// To reduce a snailfish number, you must repeatedly do the first action in this
    /// list that applies to the snailfish number:
    /// - If any pair is nested inside four pairs, the leftmost such pair explodes.
    /// - If any regular number is 10 or greater, the leftmost such regular number splits.
    ///
    /// Once no action in the above list applies, the snailfish number is reduced.
    /// During reduction, at most one action applies, after which the process returns
    /// to the top of the list of actions.
    ///
    /// To explode a pair, the pair's left value is added to the first regular number
    /// to the left of the exploding pair (if any), and the pair's right value is added
    /// to the first regular number to the right of the exploding pair (if any).
    /// Exploding pairs will always consist of two regular numbers. Then, the entire
    /// exploding pair is replaced with the regular number 0.
    ///
    /// To split a regular number, replace it with a pair; the left element of the pair
    /// should be the regular number divided by two and rounded down, while the right
    /// element of the pair should be the regular number divided by two and rounded up.
    ///
    /// The magnitude of a pair is 3 times the magnitude of its left element plus 2
    /// times the magnitude of its right element. The magnitude of a regular number
    /// is just that number.
    ///
    /// Add up all of the snailfish numbers from the homework assignment in the order
    /// they appear. What is the magnitude of the final sum?
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
