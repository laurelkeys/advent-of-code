//! --- Day 19: Monster Messages ---

use crate::solver::Solver;
use std::{collections::HashMap, io, str::FromStr};

/// https://adventofcode.com/2020/day/19
pub struct Day19;

#[derive(Clone, Debug)]
pub enum Rule {
    Single(char),
    Sequences(Vec<Vec<u8>>),
}

fn matches(msg: &str, rule: u8, rules: &HashMap<u8, Rule>) -> bool {
    eat_matches(msg, rule, rules).contains(&"") // full match
}

fn eat_matches<'m>(msg: &'m str, rule: u8, rules: &HashMap<u8, Rule>) -> Vec<&'m str> {
    match &rules[&rule] {
        Rule::Single(c) if msg.starts_with(*c) => vec![&msg[1..]],
        Rule::Sequences(seqs) => seqs
            .iter()
            .flat_map(|seq| {
                seq.iter().fold(vec![msg], |msg_matches, &seq_rule| {
                    msg_matches
                        .into_iter()
                        .flat_map(|msg_match| {
                            if msg_match != "" {
                                eat_matches(msg_match, seq_rule, rules)
                            } else {
                                vec![]
                            }
                        })
                        .collect()
                })
            })
            .collect::<Vec<&str>>(),
        _ => vec![],
    }
}

impl Solver for Day19 {
    type Input = (HashMap<u8, Rule>, Vec<String>);
    type Output1 = usize;
    type Output2 = usize;

    fn solve_part1(&self, input: &Self::Input) -> Self::Output1 {
        let (rules, messages) = input;

        messages.iter().filter(|msg| matches(msg, 0, rules)).count()
    }

    fn solve_part2(&self, input: &Self::Input) -> Self::Output2 {
        let (rules, messages) = input;

        let rules: HashMap<u8, Rule> = rules
            .iter()
            .map(|(&i, rule)| match i {
                11 => (i, "42 31 | 42 11 31".parse().unwrap()),
                8 => (i, "42 | 42 8".parse().unwrap()),
                _ => (i, rule.clone()),
            })
            .collect();

        messages
            .iter()
            .filter(|msg| matches(msg, 0, &rules))
            .count()
    }

    fn parse_input<R: io::Read>(&self, mut r: R) -> Self::Input {
        let mut input = String::new();
        r.read_to_string(&mut input).unwrap();
        let mut input = input.trim_end().split("\n\n");

        let rules = input
            .next()
            .unwrap()
            .lines()
            .map(|line| {
                let mut line = line.splitn(2, ": ");
                let number = line.next().unwrap().parse().unwrap();
                let rule = line.next().unwrap().parse().unwrap();
                (number, rule)
            })
            .collect();

        let messages = input.next().unwrap().lines().map(String::from).collect();

        (rules, messages)
    }
}

impl FromStr for Rule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(if s.starts_with('\"') {
            assert!(s.len() == 3);
            assert!(s.ends_with('\"'));
            Rule::Single(s.chars().nth(1).unwrap())
        } else {
            Rule::Sequences(
                s.split(" | ")
                    .map(|seqs| {
                        seqs.split_whitespace()
                            .map(|n| n.parse().unwrap())
                            .collect()
                    })
                    .collect(),
            )
        })
    }
}
