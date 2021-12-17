//! --- Day 14: Extended Polymerization ---

use crate::solver::Solver;
use std::{
    collections::HashMap,
    io::{self, BufRead, BufReader},
    mem::swap,
};

/// https://adventofcode.com/2021/day/14
pub struct Day14;

impl Solver for Day14 {
    type Input = (Vec<char>, HashMap<(char, char), char>);
    type Output1 = i64;
    type Output2 = i64;

    /// Apply 10 steps of pair insertion to the polymer template and find the most and
    /// least common elements in the result. What do you get if you take the quantity
    /// of the most common element and subtract the quantity of the least common element?
    fn solve_part1(&self, input: &Self::Input) -> Self::Output1 {
        let (polymer, insertion_rules) = input;

        let mut element_count = {
            let mut ping = polymer.clone();
            let mut pong = Vec::with_capacity(ping.capacity());

            for _ in 0..10 {
                pong.clear();
                pong.push(ping[0]);
                for pair in ping.windows(2) {
                    if let Some(&element) = insertion_rules.get(&(pair[0], pair[1])) {
                        pong.push(element);
                    }
                    pong.push(pair[1]);
                }
                swap(&mut ping, &mut pong);
            }

            ping.into_iter()
                .fold(HashMap::<_, i64>::new(), |mut count, element| {
                    *count.entry(element).or_default() += 1;
                    count
                })
                .into_iter()
        };

        let (mut least_count, mut most_count) = {
            let fst = element_count.next().map(|(_, count)| count).unwrap();
            let snd = element_count.next().map(|(_, count)| count).unwrap();
            if fst <= snd {
                (fst, snd)
            } else {
                (snd, fst)
            }
        };

        for (_, count) in element_count {
            if count < least_count {
                least_count = count;
            } else if count > most_count {
                most_count = count;
            }
        }

        most_count - least_count
    }

    /// Apply 40 steps of pair insertion to the polymer template and find the most and
    /// least common elements in the result. What do you get if you take the quantity
    /// of the most common element and subtract the quantity of the least common element?
    fn solve_part2(&self, input: &Self::Input) -> Self::Output2 {
        let (polymer, insertion_rules) = input;

        let mut pair_count = {
            let mut ping =
                polymer
                    .windows(2)
                    .into_iter()
                    .fold(HashMap::<_, i64>::new(), |mut count, pair| {
                        let pair = (pair[0], pair[1]);
                        *count.entry(pair).or_default() += 1;
                        count
                    });
            let mut pong = HashMap::with_capacity(ping.capacity());

            for _ in 0..40 {
                pong.clear();
                for (pair, count) in ping.iter() {
                    if let Some(&element) = insertion_rules.get(pair) {
                        *pong.entry((pair.0, element)).or_default() += count;
                        *pong.entry((element, pair.1)).or_default() += count;
                    } else {
                        *pong.entry(*pair).or_default() += count;
                    }
                }
                swap(&mut ping, &mut pong);
            }

            ping.into_iter()
        };

        let mut element_count = {
            let mut element_count = HashMap::new();
            let (pair, count) = pair_count.next().unwrap();
            element_count.insert(pair.0, count);

            pair_count
                .fold(element_count, |mut ecount, (pair, pcount)| {
                    *ecount.entry(pair.1).or_default() += pcount;
                    ecount
                })
                .into_iter()
        };

        let (mut least_count, mut most_count) = {
            let fst = element_count.next().map(|(_, count)| count).unwrap();
            let snd = element_count.next().map(|(_, count)| count).unwrap();
            if fst <= snd {
                (fst, snd)
            } else {
                (snd, fst)
            }
        };

        for (_, count) in element_count {
            if count < least_count {
                least_count = count;
            } else if count > most_count {
                most_count = count;
            }
        }

        most_count - least_count
    }

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        let mut lines = BufReader::new(r).lines().flatten();
        let polymer_template = lines.next().unwrap().chars().collect();
        let pair_insertion_rules = lines
            .skip(1) // \n
            .map(|line| {
                let (pair, element) = line.split_once(" -> ").unwrap();
                let mut pair = pair.chars();
                let pair = (pair.next().unwrap(), pair.next().unwrap());
                let element = element.chars().next().unwrap();

                (pair, element)
            })
            .collect::<HashMap<(char, char), char>>();

        (polymer_template, pair_insertion_rules)
    }
}
