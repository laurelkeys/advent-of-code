//! --- Day 16: Packet Decoder ---

use crate::solver::Solver;
use std::io::{self, BufRead, BufReader};

/// https://adventofcode.com/2021/day/16
pub struct Day16;

fn decode(hex: char) -> [char; 4] {
    match hex {
        '0' => ['0', '0', '0', '0'],
        '1' => ['0', '0', '0', '1'],
        '2' => ['0', '0', '1', '0'],
        '3' => ['0', '0', '1', '1'],
        '4' => ['0', '1', '0', '0'],
        '5' => ['0', '1', '0', '1'],
        '6' => ['0', '1', '1', '0'],
        '7' => ['0', '1', '1', '1'],
        '8' => ['1', '0', '0', '0'],
        '9' => ['1', '0', '0', '1'],
        'A' => ['1', '0', '1', '0'],
        'B' => ['1', '0', '1', '1'],
        'C' => ['1', '1', '0', '0'],
        'D' => ['1', '1', '0', '1'],
        'E' => ['1', '1', '1', '0'],
        'F' => ['1', '1', '1', '1'],
        _ => unreachable!(),
    }
}

impl Solver for Day16 {
    type Input = Vec<char>;
    type Output1 = usize;
    type Output2 = usize;

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
