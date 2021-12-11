//! --- Day 2: Dive! ---

use crate::solver::Solver;
use std::io::{self, BufRead, BufReader};

/// https://adventofcode.com/2021/day/2
pub struct Day02;

pub enum Command {
    Forward(usize),
    Down(usize),
    Up(usize),
}

impl Solver for Day02 {
    type Input = Vec<Command>;
    type Output1 = usize;
    type Output2 = usize;

    /// What do you get if you multiply your final horizontal position by your final depth?
    fn solve_part1(&self, input: &Self::Input) -> Self::Output1 {
        let (position, depth) =
            input
                .iter()
                .fold((0, 0), |(position, depth), command| match command {
                    Command::Forward(x) => (position + x, depth),
                    Command::Down(x) => (position, depth + x),
                    Command::Up(x) => (position, depth - x),
                });

        position * depth
    }

    /// What do you get if you multiply your final horizontal position by your final depth?
    fn solve_part2(&self, input: &Self::Input) -> Self::Output2 {
        let (_, position, depth) =
            input
                .iter()
                .fold((0, 0, 0), |(aim, position, depth), command| match command {
                    Command::Forward(x) => (aim, position + x, depth + aim * x),
                    Command::Down(x) => (aim + x, position, depth),
                    Command::Up(x) => (aim - x, position, depth),
                });

        position * depth
    }

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        BufReader::new(r)
            .lines()
            .flatten()
            .map(|line| line.parse().unwrap())
            .collect::<Vec<Command>>()
    }
}

impl std::str::FromStr for Command {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (command, x) = s.split_once(' ').unwrap();
        let x = x.parse().unwrap();
        match command {
            "forward" => Ok(Command::Forward(x)),
            "down" => Ok(Command::Down(x)),
            "up" => Ok(Command::Up(x)),
            _ => unreachable!(),
        }
    }
}
