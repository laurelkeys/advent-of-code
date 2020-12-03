use crate::solver::Solver;
use std::io::{self, BufRead, BufReader};

/// https://adventofcode.com/2020/day/1
pub struct Day01;

const TARGET_SUM: usize = 2020;

impl Solver for Day01 {
    type Input = Vec<usize>;
    type Output1 = usize;
    type Output2 = usize;

    fn solve_part1(&self, input: &Self::Input) -> Self::Output1 {
        // @Note: while the natural solution would be to use a HashSet,
        // using a Vec with less than 2048 positions will do just fine.
        let mut has_complement = [false; TARGET_SUM + 1];

        for &entry in input {
            if has_complement[entry] {
                return entry * (TARGET_SUM - entry);
            }
            has_complement[TARGET_SUM - entry] = true;
        }

        unreachable!()
    }

    fn solve_part2(&self, input: &Self::Input) -> Self::Output2 {
        // @Note: same idea as above.. but O(n^2) instead of O(n).
        let mut has_complement: [bool; TARGET_SUM + 1];

        for current_target in input.iter().map(|&entry| TARGET_SUM - entry) {
            has_complement = [false; TARGET_SUM + 1];

            for &entry in input.iter().filter(|&&entry| entry <= current_target) {
                if has_complement[entry] {
                    return entry * (current_target - entry) * (TARGET_SUM - current_target);
                }
                has_complement[current_target - entry] = true;
            }
        }

        unreachable!()
    }

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        let input = BufReader::new(r)
            .lines()
            .flatten()
            .map(|line| line.parse().unwrap())
            .collect::<Vec<usize>>();

        assert!(input.iter().all(|&entry| entry <= TARGET_SUM));

        input
    }
}
