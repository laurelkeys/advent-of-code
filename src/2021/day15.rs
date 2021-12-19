//! --- Day 15: Chiton ---

use crate::solver::Solver;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
    io::{self, BufRead, BufReader},
};

/// https://adventofcode.com/2021/day/15
pub struct Day15;

impl Solver for Day15 {
    type Input = Vec<Vec<i32>>;
    type Output1 = i32;
    type Output2 = i32;

    /// You start in the top left position, your destination is the bottom right position,
    /// and you cannot move diagonally. The number at each position is its risk level;
    /// to determine the total risk of an entire path, add up the risk levels of each
    /// position you enter (that is, don't count the risk level of your starting position
    /// unless you enter it; leaving it adds no risk to your total).
    ///
    /// What is the lowest total risk of any path from the top left to the bottom right?
    fn solve_part1(&self, input: &Self::Input) -> Self::Output1 {
        let (height, width) = (input.len(), input[0].len());
        let mut total_risk = input.to_vec();
        total_risk[0][0] = 0;

        for x in 1..width {
            total_risk[0][x] += total_risk[0][x - 1];
        }

        for y in 1..height {
            total_risk[y][0] += total_risk[y - 1][0];
            for x in 1..width {
                total_risk[y][x] += Ord::min(total_risk[y][x - 1], total_risk[y - 1][x]);
            }
            for x in (0..width - 1).rev() {
                if input[y][x] + total_risk[y][x + 1] < total_risk[y][x] {
                    total_risk[y][x] = input[y][x] + total_risk[y][x + 1];
                }
            }
        }

        total_risk[height - 1][width - 1]
    }

    /// The entire cave is actually five times larger in both dimensions than you thought;
    /// the area you originally scanned is just one tile in a 5x5 tile area that forms the
    /// full map. Your original map tile repeats to the right and downward; each time the
    /// tile repeats to the right or downward, all of its risk levels are 1 higher than the
    /// tile immediately up or left of it. However, risk levels above 9 wrap back around to 1.
    ///
    /// Using the full map, what is the lowest total risk of any path from the top left to
    /// the bottom right?
    fn solve_part2(&self, input: &Self::Input) -> Self::Output2 {
        let (height_div_5, width_div_5) = (input.len(), input[0].len());

        let risk = (0..height_div_5 * 5)
            .map(|y| {
                (0..width_div_5 * 5)
                    .map(|x| {
                        let risk = input[y % height_div_5][x % width_div_5]
                            + (y / height_div_5 + x / width_div_5) as i32;
                        // if risk > 9 { risk - 9 } else { risk }
                        (risk - 1) % 9 + 1
                    })
                    .collect()
            })
            .collect::<Vec<Vec<i32>>>();

        let (height, width) = (risk.len(), risk[0].len());
        let mut total_risk = vec![vec![i32::MAX; width]; height];
        total_risk[0][0] = 0;

        // @Note: `BinaryHeap` implements a max-heap. By using `Reverse` pop() returns the smallest
        // value instead of the greatest one, making it a min-heap. Also PartialOrd will by default
        // compare based on the first element in the tuple (hence why the coordinates come second).
        let mut min_risk = BinaryHeap::<(Reverse<i32>, (usize, usize))>::new();
        min_risk.push((Reverse(0_i32), (0, 0)));

        let mut visited = HashSet::new();

        while let Some((Reverse(tile_risk), (y, x))) = min_risk.pop() {
            if !visited.contains(&(y, x)) {
                visited.insert((y, x));

                total_risk[y][x] = tile_risk;
                if (y, x) == (height - 1, width - 1) {
                    break;
                }

                for (dy, dx) in &[(1, 0), (0, 1), (0, -1), (-1, 0)] {
                    let (yy, xx) = (y as i32 + dy, x as i32 + dx);
                    if 0 <= yy && yy < height as i32 && 0 <= xx && xx < width as i32 {
                        let (yy, xx) = (yy as usize, xx as usize);
                        let new_risk = total_risk[yy][xx].min(tile_risk + risk[yy][xx]);
                        min_risk.push((Reverse(new_risk), (yy, xx)));
                    }
                }
            }
        }

        total_risk[height - 1][width - 1]
    }

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        BufReader::new(r)
            .lines()
            .flatten()
            .map(|line| {
                line.chars()
                    .map(|c| (c as u8 - b'0') as i32)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
    }
}
