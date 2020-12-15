//! --- Day 15: Rambunctious Recitation ---

use crate::solver::Solver;
use std::{
    collections::HashMap,
    io::{self, BufRead, BufReader},
};

/// https://adventofcode.com/2020/day/15
pub struct Day15;

struct MemoryGame<'a>(&'a [usize]);

impl<'a> MemoryGame<'a> {
    fn nth_number(&self, n: usize) -> usize {
        let starting_numbers = self.0;

        if n < starting_numbers.len() {
            return starting_numbers[n];
        }

        let mut prev = HashMap::new();
        let mut last = starting_numbers
            .iter()
            .enumerate()
            .map(|(turn, &starting_number)| (starting_number, turn))
            .collect::<HashMap<_, _>>();

        let mut number = *starting_numbers.last().unwrap();

        for turn in starting_numbers.len()..n {
            number = match prev.get(&number) {
                None => 0,
                Some(&prev_turn) => {
                    let &last_turn = last.get(&number).unwrap();
                    last_turn - prev_turn
                }
            };

            if let Some(last_turn) = last.insert(number, turn) {
                prev.insert(number, last_turn);
            }
        }

        number
    }
}

impl Solver for Day15 {
    type Input = Vec<usize>;
    type Output1 = usize;
    type Output2 = usize;

    fn solve_part1(&self, input: &Self::Input) -> Self::Output1 {
        MemoryGame(input).nth_number(2020)
    }

    fn solve_part2(&self, input: &Self::Input) -> Self::Output2 {
        MemoryGame(input).nth_number(30000000)
    }

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        BufReader::new(r)
            .lines()
            .flatten()
            .next()
            .unwrap()
            .split(',')
            .map(|starting_number| starting_number.parse().unwrap())
            .collect()
    }
}
