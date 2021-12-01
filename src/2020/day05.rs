//! --- Day 5: Binary Boarding ---

use crate::solver::Solver;
use std::io::{self, BufRead, BufReader};

/// https://adventofcode.com/2020/day/5
pub struct Day05;

impl Solver for Day05 {
    type Input = Vec<u16>;
    type Output1 = u16;
    type Output2 = u16;

    fn solve_part1(&self, input: &Self::Input) -> Self::Output1 {
        // What is the highest seat ID on a boarding pass?
        *input.iter().max().unwrap()
    }

    fn solve_part2(&self, input: &Self::Input) -> Self::Output2 {
        let input = {
            let mut input = input.clone();
            input.sort_unstable();
            input
        };

        // What is the ID of your seat?
        for seats in input.windows(2) {
            if let [left, right] = seats {
                // Your seat wasn't at the very front or back, though;
                // the seats with IDs +1 and -1 from yours will be in your list.
                if right - left == 2 {
                    return left + 1;
                }
            }
        }

        unreachable!();
    }

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        BufReader::new(r)
            .lines()
            .flatten()
            .map(|line| {
                line.bytes().fold(0, |seat_id, seat_char| {
                    (seat_id << 1)
                        + match seat_char as char {
                            'F' => 0, // front
                            'B' => 1, // back
                            'L' => 0, // left
                            'R' => 1, // right
                            _ => unreachable!(),
                        } as u16
                })
            })
            .collect::<Vec<_>>()
    }
}
