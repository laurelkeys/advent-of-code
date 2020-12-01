use crate::solver::Solver;
use std::io::{self, BufRead, BufReader};

const TARGET_SUM: u32 = 2020;

/// https://adventofcode.com/2020/day/1
pub struct Day01;

impl Solver for Day01 {
    type Input = Vec<u32>;
    type Output1 = u32;
    type Output2 = u32;

    fn solve_1st(&self, input: &Self::Input) -> Self::Output1 {
        // @Note: while the natural solution would be to use a HashMap,
        // using a Vec with less than 2048 positions will do just fine.
        let mut has_complement = [false; (TARGET_SUM + 1) as usize];

        for &entry in input {
            if has_complement[entry as usize] {
                return entry * (TARGET_SUM - entry);
            }
            has_complement[(TARGET_SUM - entry) as usize] = true;
        }

        panic!() // @Fixme: this is really ugly.. ugh
    }

    fn solve_2nd(&self, input: &Self::Input) -> Self::Output2 {
        // @Note: same idea as above.. but O(n^2) instead of O(n).
        let mut has_complement: [bool; (TARGET_SUM + 1) as usize];

        for target_sum in input.iter().map(|&entry| TARGET_SUM - entry) {
            has_complement = [false; (TARGET_SUM + 1) as usize];

            for &entry in input.iter().filter(|&&entry| entry <= target_sum) {
                if has_complement[entry as usize] {
                    return entry * (target_sum - entry) * (TARGET_SUM - target_sum);
                }
                has_complement[(target_sum - entry) as usize] = true;
            }
        }

        panic!() // @Fixme: this is really ugly.. ugh
    }

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        let input = BufReader::new(r)
            .lines()
            .flatten()
            .flat_map(|line| line.parse())
            .collect::<Vec<u32>>();

        assert!(input.iter().all(|&entry| entry <= TARGET_SUM));

        input
    }
}
