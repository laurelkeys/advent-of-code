//! --- Day 5: Hydrothermal Venture ---

use crate::solver::Solver;
use core::iter::Iterator;
use std::{
    collections::HashMap,
    io::{self, BufRead, BufReader},
};

/// https://adventofcode.com/2021/day/5
pub struct Day05;

impl Solver for Day05 {
    type Input = Vec<((i32, i32), (i32, i32))>;
    type Output1 = usize;
    type Output2 = usize;

    /// Consider only horizontal and vertical lines.
    /// At how many points do at least two lines overlap?
    fn solve_part1(&self, input: &Self::Input) -> Self::Output1 {
        let mut points = HashMap::<(i32, i32), i32>::new();

        for &((x1, y1), (x2, y2)) in input {
            if x1 == x2 {
                if y1 < y2 { y1..=y2 } else { y2..=y1 }
                    .for_each(|y| *points.entry((x1, y)).or_insert(0) += 1)
            } else if y1 == y2 {
                if x1 < x2 { x1..=x2 } else { x2..=x1 }
                    .for_each(|x| *points.entry((x, y1)).or_insert(0) += 1)
            }
        }

        points.values().filter(|&count| *count > 1).count()
    }

    /// Unfortunately, considering only horizontal and vertical lines doesn't give you the
    /// full picture; you need to also consider diagonal lines. The lines in your list
    /// will only ever be horizontal, vertical, or a diagonal line at exactly 45 degrees.
    /// At how many points do at least two lines overlap?
    fn solve_part2(&self, input: &Self::Input) -> Self::Output2 {
        let mut points = HashMap::<(i32, i32), i32>::new();

        for &((x1, y1), (x2, y2)) in input {
            if x1 == x2 {
                if y1 < y2 { y1..=y2 } else { y2..=y1 }
                    .for_each(|y| *points.entry((x1, y)).or_insert(0) += 1)
            } else if y1 == y2 {
                if x1 < x2 { x1..=x2 } else { x2..=x1 }
                    .for_each(|x| *points.entry((x, y1)).or_insert(0) += 1)
            } else {
                match (x1 < x2, y1 < y2) {
                    (true, true) => Iterator::zip(x1..=x2, y1..=y2)
                        .for_each(|xy| *points.entry(xy).or_insert(0) += 1),
                    (true, false) => Iterator::zip(x1..=x2, (y2..=y1).rev())
                        .for_each(|xy| *points.entry(xy).or_insert(0) += 1),
                    (false, true) => Iterator::zip((x2..=x1).rev(), y1..=y2)
                        .for_each(|xy| *points.entry(xy).or_insert(0) += 1),
                    (false, false) => Iterator::zip((x2..=x1).rev(), (y2..=y1).rev())
                        .for_each(|xy| *points.entry(xy).or_insert(0) += 1),
                }
            }
        }

        points.values().filter(|&count| *count > 1).count()
    }

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        let input = BufReader::new(r)
            .lines()
            .flatten()
            .map(|line| {
                match line
                    .split(" -> ")
                    .flat_map(|coord| coord.split(',').flat_map(|c| c.parse()))
                    .collect::<Vec<i32>>()[..]
                {
                    [x1, y1, x2, y2] => ((x1, y1), (x2, y2)),
                    _ => unreachable!(),
                }
            })
            .collect::<Vec<((i32, i32), (i32, i32))>>();

        input
    }
}
