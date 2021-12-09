//! --- Day 9: Smoke Basin ---

use crate::solver::Solver;
use std::io::{self, BufRead, BufReader};

/// https://adventofcode.com/2021/day/9
pub struct Day09;

impl Solver for Day09 {
    type Input = (Vec<u8>, usize);
    type Output1 = usize;
    type Output2 = usize;

    /// Your first goal is to find the low points - the locations that are lower than any of its
    /// adjacent locations. The risk level of a low point is 1 plus its height.
    /// What is the sum of the risk levels of all low points on your heightmap?
    fn solve_part1(&self, input: &Self::Input) -> Self::Output1 {
        let (heightmap, width) = input;
        let length = heightmap.len() / width;

        let adjacent_heights = |index: usize| {
            let x = index % width;
            let y = index / width;

            let xs = match x {
                0 => vec![x + 1],
                _ if x < width - 1 => vec![x - 1, x + 1],
                _ => vec![x - 1],
            }
            .into_iter();
            let ys = match y {
                0 => vec![y + 1],
                _ if y < length - 1 => vec![y - 1, y + 1],
                _ => vec![y - 1],
            }
            .into_iter();

            xs.map(move |xx| (xx, y))
                .chain(ys.map(move |yy| (x, yy)))
                .map(|(xx, yy)| heightmap[xx + yy * width])
        };

        heightmap
            .iter()
            .enumerate()
            .filter(|(i, height)| adjacent_heights(*i).all(|h| **height < h))
            .fold(0, |risk_level_sum, (_, height)| {
                risk_level_sum + (*height as usize + 1)
            })
    }

    ///
    fn solve_part2(&self, input: &Self::Input) -> Self::Output2 {
        todo!()
    }

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        let lines = BufReader::new(r).lines().flatten().collect::<Vec<String>>();
        let width = lines.first().unwrap().len();

        (
            lines
                .iter()
                .flat_map(|line| line.chars().map(|c| c as u8 - b'0').collect::<Vec<_>>())
                .collect::<Vec<u8>>(),
            width,
        )
    }
}
