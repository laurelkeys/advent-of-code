//! --- Day 13: Shuttle Search ---

use crate::solver::Solver;
use std::io::{self, BufRead, BufReader};

/// https://adventofcode.com/2020/day/13
pub struct Day13;

pub struct Bus(usize);

/// Returns the ceilling of the (non-integer) division `x / y`.
fn divup(x: usize, y: usize) -> usize {
    (x + y - 1) / y
}

impl Solver for Day13 {
    type Input = (usize, Vec<Option<Bus>>);
    type Output1 = usize;
    type Output2 = usize;

    fn solve_part1(&self, input: &Self::Input) -> Self::Output1 {
        let (earliest_depart, buses) = input;
        let (earliest_bus, depart) = buses
            .iter()
            .filter_map(|bus| match bus {
                None => None,
                Some(bus) => {
                    let depart = bus.0 * divup(*earliest_depart, bus.0);
                    Some((bus, depart))
                }
            })
            .min_by_key(|(_, depart)| *depart)
            .unwrap();

        earliest_bus.0 * (depart - earliest_depart)
    }

    fn solve_part2(&self, input: &Self::Input) -> Self::Output2 {
        let (_, buses) = input;
        let (earliest_timestamp, _) = buses
            .iter()
            .enumerate()
            .filter_map(|(offset, bus)| bus.as_ref().map(|bus| (offset, bus)))
            .fold(
                (0, 1),
                |(mut earliest_timestamp, timestep), (offset, bus)| {
                    earliest_timestamp = (earliest_timestamp..)
                        .step_by(timestep)
                        .find(|timestamp| (timestamp + offset) % bus.0 == 0)
                        .unwrap();
                    (earliest_timestamp, timestep * bus.0)
                },
            );

        earliest_timestamp
    }

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        let mut input = BufReader::new(r).lines().flatten();

        let earliest_depart = input.next().unwrap();
        let bus_ids = input.next().unwrap();

        (
            earliest_depart.parse::<usize>().unwrap(),
            bus_ids
                .split(',')
                .map(|id| match id {
                    "x" => None,
                    _ => Some(Bus(id.parse::<usize>().unwrap())),
                })
                .collect(),
        )
    }
}
