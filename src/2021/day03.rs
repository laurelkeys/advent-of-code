//! --- Day 3: Binary Diagnostic ---

use crate::solver::Solver;
use std::io::{self, BufRead, BufReader};

/// https://adventofcode.com/2021/day/3
pub struct Day03;

impl Solver for Day03 {
    type Input = (Vec<u16>, u8);
    type Output1 = usize;
    type Output2 = usize;

    /// Use the binary numbers in your diagnostic report to calculate the gamma rate
    /// and epsilon rate, then multiply them together. What is the power consumption
    /// of the submarine? (Be sure to represent your answer in decimal, not binary.)
    fn solve_part1(&self, input: &Self::Input) -> Self::Output1 {
        let (numbers, bit_count) = input;
        let (gamma_rate, epsilon_rate) = compute_rates(numbers, *bit_count);

        gamma_rate * epsilon_rate
    }

    fn solve_part2(&self, input: &Self::Input) -> Self::Output2 {
        todo!()
    }

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        let binary_numbers = BufReader::new(r).lines().flatten().collect::<Vec<String>>();
        let bit_count = binary_numbers.first().unwrap().len() as u8;

        (
            binary_numbers
                .into_iter()
                .map(|line| u16::from_str_radix(&line, 2).unwrap())
                .collect::<Vec<u16>>(),
            bit_count,
        )
    }
}

/// Each bit in the gamma rate can be determined by finding the most common bit
/// in the corresponding position of all numbers in the diagnostic report.
/// The epsilon rate is calculated in a similar way; rather than use the
/// most common bit, the least common bit from each position is used.
fn compute_rates(numbers: &[u16], bit_count: u8) -> (usize, usize) {
    fn compute_bit_counts(numbers: &[u16], bit_index: u8) -> (usize, usize) {
        let bit_mask = 1 << bit_index;
        numbers.iter().fold((0, 0), |(count_0, count_1), n| {
            if n & bit_mask == 0 {
                (count_0 + 1, count_1)
            } else {
                (count_0, count_1 + 1)
            }
        })
    }

    let mut gamma_rate = 0;
    let mut epsilon_rate = 0;
    for bit_shift in 0..bit_count {
        let (count_0, count_1) = compute_bit_counts(numbers, bit_shift);
        if count_0 > count_1 {
            // gamma_rate |= 0 << bit_shift;
            epsilon_rate |= 1 << bit_shift;
        } else {
            assert!(count_0 != count_1);
            gamma_rate |= 1 << bit_shift;
            // epsilon_rate |= 0 << bit_shift
        }
    }

    (gamma_rate, epsilon_rate)
}
