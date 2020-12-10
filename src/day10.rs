//! --- Day 10: Adapter Array ---

use crate::solver::Solver;
use std::io::{self, BufRead, BufReader};

/// https://adventofcode.com/2020/day/10
pub struct Day10;

// Treat the charging outlet near your seat as having an effective
// joltage rating of 0.
const CHARGING_OUTLET: usize = 0;

// Any given adapter can take an input 1, 2, or 3 jolts lower
// than its rating and still produce its rated output joltage.
const MAX_DELTA: usize = 3;

impl Solver for Day10 {
    type Input = Vec<usize>;
    type Output1 = usize;
    type Output2 = usize;

    fn solve_part1(&self, input: &Self::Input) -> Self::Output1 {
        // What is the number of 1-jolt differences multiplied by
        // the number of 3-jolt differences?
        let mut joltage_differences = [0; MAX_DELTA + 1];

        joltage_differences[input[0] - CHARGING_OUTLET] += 1;

        for pair in input.windows(2) {
            joltage_differences[pair[1] - pair[0]] += 1;
        }

        // Your device has a built-in joltage adapter rated for 3 jolts
        // higher than the highest-rated adapter in your bag.
        joltage_differences[1] * (joltage_differences[3] + 1)
    }

    fn solve_part2(&self, input: &Self::Input) -> Self::Output2 {
        // What is the total number of distinct ways you can arrange the
        // adapters to connect the charging outlet to your device?
        let mut adapter_arrangements = input
            .iter()
            .map(|&adapter| (adapter - CHARGING_OUTLET <= MAX_DELTA) as usize)
            .collect::<Vec<_>>();

        for i in 0..input.len() {
            adapter_arrangements[i] += (0..i)
                .rev()
                .map(|j| {
                    if input[i] - input[j] <= MAX_DELTA {
                        Some(adapter_arrangements[j])
                    } else {
                        None
                    }
                })
                .take_while(|arrangements| arrangements.is_some())
                .flatten()
                .sum::<usize>();
        }

        *adapter_arrangements.last().unwrap()
    }

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        let mut adapters = BufReader::new(r)
            .lines()
            .flatten()
            .flat_map(|line| line.parse::<usize>())
            .collect::<Vec<_>>();

        adapters.sort_unstable();

        adapters
    }
}
