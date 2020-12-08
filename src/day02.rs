//! --- Day 2: Password Philosophy ---

use crate::solver::Solver;
use std::io::{self, BufRead, BufReader};

/// https://adventofcode.com/2020/day/2
pub struct Day02;

pub struct Password(String);
pub struct Policy {
    pub letter: char,
    pub start: usize,
    pub end: usize,
}

impl Solver for Day02 {
    type Input = Vec<(Policy, Password)>;
    type Output1 = usize;
    type Output2 = usize;

    fn solve_part1(&self, input: &Self::Input) -> Self::Output1 {
        let mut valid_passwords = 0;

        for (policy, password) in input {
            let letter_count = password.0.matches(policy.letter).count();
            if (policy.start..=policy.end).contains(&letter_count) {
                valid_passwords += 1
            }
        }

        valid_passwords
    }

    fn solve_part2(&self, input: &Self::Input) -> Self::Output2 {
        let mut valid_passwords = 0;

        for (policy, password) in input {
            // @Note: a position equal to 1 refers to the first letter (i.e. index 0).
            if let (Some(fst), Some(snd)) = (
                password.0.chars().nth(policy.start - 1),
                password.0.chars().nth(policy.end - 1),
            ) {
                // Exactly one of these positions must contain the given letter.
                if (fst == policy.letter) ^ (snd == policy.letter) {
                    valid_passwords += 1;
                }
            }
        }

        valid_passwords
    }

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        BufReader::new(r)
            .lines()
            .flatten()
            .map(|line| {
                let line: Vec<&str> = line.split_whitespace().collect();

                let mut range = line[0].split('-');
                let letter = line[1].strip_suffix(":").unwrap();
                let password = line[2];

                assert_eq!(letter.len(), 1);
                let letter = letter.chars().next().unwrap();
                let start = range.next().unwrap().parse().unwrap();
                let end = range.next().unwrap().parse().unwrap();
                assert!(start < password.len() && end <= password.len());

                (
                    Policy { letter, start, end },
                    Password(password.to_string()),
                )
            })
            .collect::<Vec<(Policy, Password)>>()
    }
}
