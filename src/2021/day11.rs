//! --- Day 11: Dumbo Octopus ---

use crate::solver::Solver;
use std::{
    collections::VecDeque,
    convert::TryInto,
    io::{self, BufRead, BufReader},
};

/// https://adventofcode.com/2021/day/11
pub struct Day11;

/// You can model the energy levels and flashes of light in steps.
/// During a single step, the following occurs:
/// - First, the energy level of each octopus increases by 1.
/// - Then, any octopus with an energy level greater than 9 flashes.
///   This increases the energy level of all adjacent octopuses by 1, including
///   octopuses that are diagonally adjacent. If this causes an octopus to have
///   an energy level greater than 9, it also flashes.
///  (An octopus can only flash at most once per step.)
/// - Finally, any octopus that flashed during this step has its energy level
///   set to 0, as it used all of its energy to flash.
fn step(energy_levels: &mut [[u8; 10]; 10]) -> usize {
    let mut flashes = energy_levels
        .iter_mut()
        .enumerate()
        .flat_map(|(y, energy_levels_row)| {
            energy_levels_row
                .iter_mut()
                .enumerate()
                .filter_map(|(x, energy_level)| {
                    *energy_level += 1;
                    if *energy_level > 9 {
                        Some((y, x))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<VecDeque<_>>();

    let mut flashes_count = 0;
    while let Some((y, x)) = flashes.pop_front() {
        if energy_levels[y][x] != 0 {
            energy_levels[y][x] = 0;
            flashes_count += 1;
            for (yy, xx) in adjacent(y, x) {
                let energy_level = &mut energy_levels[yy][xx];
                if *energy_level != 0 {
                    *energy_level += 1;
                    if *energy_level > 9 {
                        flashes.push_back((yy, xx));
                    }
                }
            }
        }
    }

    flashes_count
}

fn adjacent(y: usize, x: usize) -> impl Iterator<Item = (usize, usize)> {
    match y {
        0 => 0..=1,
        9 => 8..=9,
        _ => y - 1..=y + 1,
    }
    .flat_map(move |y| {
        match x {
            0 => 0..=1,
            9 => 8..=9,
            _ => x - 1..=x + 1,
        }
        .map(move |x| (y, x))
    })
}

impl Solver for Day11 {
    type Input = [[u8; 10]; 10];
    type Output1 = usize;
    type Output2 = usize;

    /// There are 100 octopuses arranged neatly in a 10 by 10 grid. Each octopus
    /// slowly gains energy over time and flashes brightly for a moment when its
    /// energy is full. The energy level of each octopus is a value between 0 and 9.
    ///
    /// How many total flashes are there after 100 steps?
    fn solve_part1(&self, input: &Self::Input) -> Self::Output1 {
        let mut energy_levels = input.to_owned();
        let mut total_flashes = 0;
        for _ in 0..100 {
            total_flashes += step(&mut energy_levels);
        }
        total_flashes
    }

    /// What is the first step during which all octopuses flash?
    fn solve_part2(&self, input: &Self::Input) -> Self::Output2 {
        let mut energy_levels = input.to_owned();
        let mut step_number = 1;
        while step(&mut energy_levels) != 100 {
            step_number += 1;
        }
        step_number
    }

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        BufReader::new(r)
            .lines()
            .flatten()
            .map(|line| {
                line.chars()
                    .map(|c| c as u8 - b'0')
                    .collect::<Vec<u8>>()
                    .try_into()
                    .unwrap()
            })
            .collect::<Vec<[u8; 10]>>()
            .try_into()
            .unwrap()
    }
}
