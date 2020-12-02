use crate::solver::Solver;
use std::io::{self, BufRead, BufReader};

/// https://adventofcode.com/2020/day/2
pub struct Day02;

pub struct Password(String);
pub struct Policy {
    pub letter: char,
    pub start: i32,
    pub end: i32,
}

impl Solver for Day02 {
    type Input = Vec<(Policy, Password)>;
    type Output1 = i32;
    type Output2 = i32;

    fn solve_1st(&self, input: &Self::Input) -> Option<Self::Output1> {
        let mut valid_passwords = 0;

        for (policy, password) in input {
            let letter_count = password.0.matches(policy.letter).count() as i32;
            if (policy.start..=policy.end).contains(&letter_count) {
                valid_passwords += 1
            }
        }

        Some(valid_passwords)
    }

    fn solve_2nd(&self, input: &Self::Input) -> Option<Self::Output2> {
        let mut valid_passwords = 0;

        for (policy, password) in input {
            // @Note: a position equal to 1 refers to the first letter (i.e. index 0).
            if let (Some(fst), Some(snd)) = (
                password.0.chars().nth((policy.start - 1) as usize),
                password.0.chars().nth((policy.end - 1) as usize),
            ) {
                // Exactly one of these positions must contain the given letter.
                if (fst == policy.letter) ^ (snd == policy.letter) {
                    valid_passwords += 1;
                }
            }
        }

        Some(valid_passwords)
    }

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        let input = BufReader::new(r)
            .lines()
            .flatten()
            .map(|line| {
                let line: Vec<&str> = line.split_whitespace().collect();

                let mut range = (*line.get(0).unwrap()).split('-');
                let letter = (*line.get(1).unwrap()).strip_suffix(":").unwrap();
                let password = *line.get(2).unwrap();

                (
                    Policy {
                        letter: letter.chars().next().unwrap(),
                        start: range.next().unwrap().parse().unwrap(),
                        end: range.next().unwrap().parse().unwrap(),
                    },
                    Password(password.to_string()),
                )
            })
            .collect::<Vec<(Policy, Password)>>();

        // eprintln!("{:?}", input);

        input
    }
}
