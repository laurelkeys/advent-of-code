//! --- Day 3: Binary Diagnostic ---

use crate::solver::Solver;
use std::io::{self, BufRead, BufReader};

/// https://adventofcode.com/2021/day/3
pub struct Day03;

impl Solver for Day03 {
    type Input = (Vec<u16>, u8);
    type Output1 = u32;
    type Output2 = u32;

    /// Use the binary numbers in your diagnostic report to calculate the gamma rate
    /// and epsilon rate, then multiply them together. What is the power consumption
    /// of the submarine? (Be sure to represent your answer in decimal, not binary.)
    fn solve_part1(&self, input: &Self::Input) -> Self::Output1 {
        let (numbers, bit_count) = input;
        let (gamma_rate, epsilon_rate) = compute_rates(numbers, *bit_count);

        gamma_rate as u32 * epsilon_rate as u32
    }

    /// Use the binary numbers in your diagnostic report to calculate the oxygen generator
    /// rating and CO2 scrubber rating, then multiply them together.
    /// What is the life support rating of the submarine?
    fn solve_part2(&self, input: &Self::Input) -> Self::Output2 {
        let (numbers, bit_count) = input;
        let (oxygen_generator_rating, co2_scrubber_rating) = compute_ratings(numbers, *bit_count);

        oxygen_generator_rating as u32 * co2_scrubber_rating as u32
    }

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        let lines = BufReader::new(r).lines().flatten().collect::<Vec<String>>();
        let bit_count = lines.first().unwrap().len() as u8;

        (
            lines
                .into_iter()
                .map(|line| u16::from_str_radix(&line, 2).unwrap())
                .collect::<Vec<u16>>(),
            bit_count,
        )
    }
}

fn compute_bit_counts(numbers: &[u16], bit_shift: u8) -> (u16, u16) {
    numbers.iter().fold((0, 0), |(count_0s, count_1s), n| {
        if n & (1 << bit_shift) == 0 {
            (count_0s + 1, count_1s)
        } else {
            (count_0s, count_1s + 1)
        }
    })
}

/// Each bit in the gamma rate can be determined by finding the most common bit
/// in the corresponding position of all numbers in the diagnostic report.
/// The epsilon rate is calculated in a similar way; rather than use the
/// most common bit, the least common bit from each position is used.
fn compute_rates(numbers: &[u16], bit_count: u8) -> (u16, u16) {
    let mut gamma_rate = 0;
    let mut epsilon_rate = 0;

    // @Speed: this loop could be done in parallel, as one bit lane doesn't depend on others.
    for bit_shift in 0..bit_count {
        let (count_0s, count_1s) = compute_bit_counts(numbers, bit_shift);
        match Ord::cmp(&count_0s, &count_1s) {
            std::cmp::Ordering::Less => gamma_rate |= 1 << bit_shift,
            std::cmp::Ordering::Equal => unreachable!(),
            std::cmp::Ordering::Greater => epsilon_rate |= 1 << bit_shift,
        }
    }

    (gamma_rate, epsilon_rate)
}

/// Start with the full list of binary numbers from your diagnostic report and
/// *consider just the first bit* of those numbers. Then:
/// - Keep only numbers selected by the bit criteria for the type of rating value for
///   which you are searching. Discard numbers which do not match the bit criteria.
/// - If you only have one number left, stop; this is the rating value.
/// - Otherwise, repeat the process, considering the next bit to the right.
///
/// The bit criteria depends on which type of rating value you want to find:
/// - To find *oxygen generator rating*, determine the *most common* value in the
///   current bit position, and keep only numbers with that bit in that position.
///   If 0 and 1 are equally common, keep values with a 1.
/// - To find *CO2 scrubber rating*, determine the least common value in the current
///   bit position, and keep only numbers with that bit in that position.
///   If 0 and 1 are equally common, keep values with a 0.
fn compute_ratings(numbers: &[u16], bit_count: u8) -> (u16, u16) {
    fn bit_matches(bit: u16, n: u16, bit_shift: u8) -> bool {
        n & (1 << bit_shift) == bit << bit_shift
    }

    // Note that, when count_0s == count_1s, we say the most common bit is 1 (and the
    // least common bit is 0) so that we can correctly partition into the two ratings.
    fn common_bits(numbers: &[u16], bit_shift: u8) -> (u16, u16) {
        let (count_0s, count_1s) = compute_bit_counts(numbers, bit_shift);

        match Ord::cmp(&count_0s, &count_1s) {
            std::cmp::Ordering::Less => (1, 0),
            std::cmp::Ordering::Equal => (1, 0),
            std::cmp::Ordering::Greater => (0, 1),
        }
    }

    let bit_shift = bit_count - 1;
    let (most_common_bit, _) = common_bits(numbers, bit_shift);
    let (mut oxygen_generators, mut co2_scrubbers) = numbers
        .iter()
        .partition::<Vec<u16>, _>(|&n| bit_matches(most_common_bit, *n, bit_shift));

    // @Speed: we could compute `oxygen_generators` and `co2_scrubbers` in parallel.
    for bit_shift in (0..bit_shift).rev() {
        if oxygen_generators.len() > 1 {
            let (most_common_bit, _) = common_bits(&oxygen_generators, bit_shift);
            oxygen_generators.retain(|n| bit_matches(most_common_bit, *n, bit_shift));
        }
        if co2_scrubbers.len() > 1 {
            let (_, least_common_bit) = common_bits(&co2_scrubbers, bit_shift);
            co2_scrubbers.retain(|n| bit_matches(least_common_bit, *n, bit_shift));
        }
        if oxygen_generators.len() == 1 && co2_scrubbers.len() == 1 {
            break;
        }
    }

    let oxygen_generator_rating = oxygen_generators[0];
    let co2_scrubber_rating = co2_scrubbers[0];

    (oxygen_generator_rating, co2_scrubber_rating)
}
