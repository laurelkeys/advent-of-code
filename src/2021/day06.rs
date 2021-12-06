//! --- Day 6: Lanternfish ---

use crate::solver::Solver;
use std::io::{self, BufRead, BufReader};

/// https://adventofcode.com/2021/day/6
pub struct Day06;

/// You can model each fish as a single number that represents the number of days until
/// it creates a new lanternfish. Furthermore, you reason, a new lanternfish would surely
/// need slightly longer before it's capable of producing more lanternfish: two more days
/// for its first cycle.
fn count_after(days: u64, ages: &[u8]) -> u64 {
    let mut count = [0; 9];
    for &age in ages {
        count[age as usize] += 1;
    }
    for _ in 0..days {
        count.rotate_left(1);
        // Account for the "parents" which go back to 6
        // (while the "new born" lanterfish start at 8).
        count[6] += count[8];
    }
    count.iter().sum()
}

impl Solver for Day06 {
    type Input = Vec<u8>;
    type Output1 = u64;
    type Output2 = u64;

    /// How many lanternfish would there be after 80 days?
    fn solve_part1(&self, input: &Self::Input) -> Self::Output1 {
        count_after(80, input)
    }

    /// How many lanternfish would there be after 256 days?
    fn solve_part2(&self, input: &Self::Input) -> Self::Output2 {
        count_after(256, input)
    }

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        let input = BufReader::new(r)
            .lines()
            .flatten()
            .flat_map(|line| {
                line.split(',')
                    .map(|age| age.parse().unwrap())
                    .collect::<Vec<u8>>()
            })
            .collect();

        input
    }
}
