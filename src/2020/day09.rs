//! --- Day 9: Encoding Error ---

use crate::solver::Solver;
use std::io::{self, BufRead, BufReader};

/// https://adventofcode.com/2020/day/9
pub struct Day09;

const PREAMBLE_LENGTH: usize = 25;

fn sum_two(numbers: &[usize], target: usize) -> Option<(usize, usize)> {
    for i in 1..numbers.len() {
        for j in 0..i {
            if target == numbers[i] + numbers[j] {
                return Some((numbers[i], numbers[j]));
            }
        }
    }
    None
}

fn min_max(range: &[usize]) -> Option<(usize, usize)> {
    if range.is_empty() {
        return None;
    }

    let mut range = range.iter();
    let first = *range.next().unwrap();

    Some(range.fold((first, first), |(smallest, largest), &number| {
        if number < smallest {
            (number, largest)
        } else if number > largest {
            (smallest, number)
        } else {
            (smallest, largest)
        }
    }))
}

impl Solver for Day09 {
    type Input = Vec<usize>;
    type Output1 = usize;
    type Output2 = usize;

    fn solve_part1(&self, input: &Self::Input) -> Self::Output1 {
        // Find the first number in the list (after the preamble) which
        // is not the sum of two of the 25 numbers before it.
        for i in PREAMBLE_LENGTH..input.len() {
            let preamble = &input[i - PREAMBLE_LENGTH..i];
            let number = input[i];
            if sum_two(preamble, number).is_none() {
                return number;
            }
        }

        unreachable!()
    }

    fn solve_part2(&self, input: &Self::Input) -> Self::Output2 {
        // Find a contiguous set of at least two numbers in your list
        // which sum to the invalid number from part 1.
        let invalid_number = self.solve_part1(input);

        let (mut first, mut last, mut sum) = (0, 0, input[0]);

        while sum != invalid_number {
            while sum < invalid_number {
                last += 1;
                sum += input[last];
            }
            while sum > invalid_number {
                sum -= input[first];
                first += 1;
            }
        }

        // To find the encryption weakness, add together the smallest
        // and largest number in this contiguous range.
        let (smallest, largest) = min_max(&input[first..=last]).unwrap();

        smallest + largest
    }

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        BufReader::new(r)
            .lines()
            .flatten()
            .flat_map(|line| line.parse::<usize>())
            .collect::<Vec<_>>()
    }
}
