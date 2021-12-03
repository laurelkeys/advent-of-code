//! --- Day 1: Sonar Sweep ---

use crate::solver::Solver;
use std::io::{self, BufRead, BufReader};

/// https://adventofcode.com/2021/day/1
pub struct Day01;

impl Solver for Day01 {
    type Input = Vec<usize>;
    type Output1 = usize;
    type Output2 = usize;

    /// Count the number of times a depth measurement increases from the previous measurement.
    /// How many measurements are larger than the previous measurement?
    fn solve_part1(&self, input: &Self::Input) -> Self::Output1 {
        input
            .iter()
            .fold((input[0], 0), |(prev, count), &curr| {
                (curr, if curr > prev { count + 1 } else { count })
            })
            .1
    }

    /// Consider sums of a three-measurement sliding window.
    /// Count the number of times the sum of measurements in this sliding window increases.
    /// How many sums are larger than the previous sum?
    fn solve_part2(&self, input: &Self::Input) -> Self::Output2 {
        // Note that: if (B + C + D) > (A + B + C), then A < D.
        input.windows(4).fold(0, |count, window| {
            if let [curr, .., prev] = window {
                if curr < prev {
                    count + 1
                } else {
                    count
                }
            } else {
                unreachable!()
            }
        })
    }

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        let input = BufReader::new(r)
            .lines()
            .flatten()
            .map(|line| line.parse().unwrap())
            .collect::<Vec<usize>>();

        input
    }
}
