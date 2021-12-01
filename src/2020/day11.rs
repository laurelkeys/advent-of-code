//! --- Day 11: Seating System ---

use crate::solver::Solver;
use std::io::{self, BufRead, BufReader};

/// https://adventofcode.com/2020/day/11
pub struct Day11;

#[derive(Clone, Debug)]
pub struct SeatingArea {
    seats: Vec<Vec<Seat>>,
    rows: usize,
    cols: usize,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Seat {
    Floor,
    Empty,
    Occupied,
}

#[rustfmt::skip]
const NEIGHBORHOOD: [(i32, i32); 8] = [
    (-1, -1), (0, -1), (1, -1),
    (-1,  0),          (1,  0),
    (-1,  1), (0,  1), (1,  1),
];

// @Cleanup: add borders to the seating area and remove bound checks.

fn adjacent(s: &SeatingArea, row: usize, col: usize) -> Vec<Seat> {
    NEIGHBORHOOD
        .iter()
        .map(|&(dx, dy)| {
            let y = row as i32 + dy;
            let x = col as i32 + dx;
            if (0..s.rows as i32).contains(&y) && (0..s.cols as i32).contains(&x) {
                Some(s.seats[y as usize][x as usize])
            } else {
                None
            }
        })
        .flatten()
        .collect()
}

fn in_sight(s: &SeatingArea, row: usize, col: usize) -> Vec<Seat> {
    NEIGHBORHOOD
        .iter()
        .map(|&(dx, dy)| {
            let mut y = row as i32 + dy;
            let mut x = col as i32 + dx;

            while (0..s.rows as i32).contains(&y)
                && (0..s.cols as i32).contains(&x)
                && s.seats[y as usize][x as usize] == Seat::Floor
            {
                y += dy;
                x += dx;
            }

            if (0..s.rows as i32).contains(&y) && (0..s.cols as i32).contains(&x) {
                Some(s.seats[y as usize][x as usize])
            } else {
                None
            }
        })
        .flatten()
        .collect()
}

fn neighbors<'s, S>(seats: S) -> usize
where
    S: IntoIterator<Item = &'s Seat>,
{
    seats
        .into_iter()
        .filter(|&&neighbor| neighbor == Seat::Occupied)
        .count()
}

impl SeatingArea {
    fn next<N>(
        &mut self,
        neighborhood: N,
        become_occupied: fn(usize) -> bool,
        become_empty: fn(usize) -> bool,
    ) -> bool
    where
        N: Fn(&Self, usize, usize) -> Vec<Seat>,
    {
        let mut next_seats = self.seats.clone();
        let mut changed = false;

        (0..self.rows).for_each(|row| {
            (0..self.cols).for_each(|col| match self.seats[row][col] {
                Seat::Empty if become_occupied(neighbors(&neighborhood(self, row, col))) => {
                    next_seats[row][col] = Seat::Occupied;
                    changed = true;
                }
                Seat::Occupied if become_empty(neighbors(&neighborhood(self, row, col))) => {
                    next_seats[row][col] = Seat::Empty;
                    changed = true;
                }
                _ => {}
            })
        });

        self.seats = next_seats;
        changed
    }
}

impl Solver for Day11 {
    type Input = SeatingArea;
    type Output1 = usize;
    type Output2 = usize;

    fn solve_part1(&self, input: &Self::Input) -> Self::Output1 {
        let mut seating_area = input.clone();

        // Seating rules:
        fn become_occupied(neighbors: usize) -> bool {
            neighbors == 0
        }
        fn become_empty(neighbors: usize) -> bool {
            neighbors >= 4
        }
        let neighborhood = adjacent;

        // Simulate your seating area by applying the seating rules repeatedly
        // until no seats change state. How many seats end up occupied?
        while seating_area.next(neighborhood, become_occupied, become_empty) {}

        neighbors(seating_area.seats.iter().flatten())
    }

    fn solve_part2(&self, input: &Self::Input) -> Self::Output2 {
        let mut seating_area = input.clone();

        // New seating rules:
        fn become_occupied(neighbors: usize) -> bool {
            neighbors == 0
        }
        fn become_empty(neighbors: usize) -> bool {
            neighbors >= 5
        }
        let neighborhood = in_sight;

        // Given the new visibility method and the rule change for occupied seats
        // becoming empty, once equilibrium is reached, how many seats end up occupied?
        while seating_area.next(neighborhood, become_occupied, become_empty) {}

        neighbors(seating_area.seats.iter().flatten())
    }

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        let seats = BufReader::new(r)
            .lines()
            .flatten()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '.' => Seat::Floor,
                        'L' => Seat::Empty,
                        '#' => Seat::Occupied,
                        _ => unreachable!(),
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let rows = seats.len();
        let cols = seats[0].len();

        SeatingArea { seats, rows, cols }
    }
}
