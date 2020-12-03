use crate::solver::Solver;
use std::{
    fmt::Display,
    io::{self, BufRead, BufReader},
};

/// https://adventofcode.com/2020/day/3
pub struct Day03;

enum GridCell {
    Open,
    Tree,
}
pub struct Grid {
    cells: Vec<Vec<GridCell>>,
    width: usize,
    height: usize,
}

#[derive(Copy, Clone)]
struct Coord(usize, usize);
#[derive(Copy, Clone)]
struct Slope(usize, usize);

const START: Coord = Coord(0, 0); // top-left (x, y)
const SLOPE: Slope = Slope(3, 1); // right 3, down 1

fn checked_positions(start: Coord, slope: Slope, width: usize, height: usize) -> Vec<Coord> {
    assert!(slope.1 > 0);

    let mut checked_positions = vec![start];
    for y in ((start.1 + slope.1)..=(height)).step_by(slope.1) {
        let &Coord(x, _) = checked_positions.last().unwrap();
        checked_positions.push(Coord((x + slope.0) % width, y));
    }

    checked_positions
}

impl Grid {
    fn count_trees(&self, positions: &[Coord]) -> usize {
        positions
            .iter()
            .filter(|&&Coord(x, y)| {
                matches!(
                    self.cells.get(y).and_then(|row| row.get(x)),
                    Some(GridCell::Tree)
                )
            })
            .count()
    }
}

impl Solver for Day03 {
    type Input = Grid;
    type Output1 = usize;
    type Output2 = usize;

    fn solve_part1(&self, input: &Self::Input) -> Self::Output1 {
        input.count_trees(&checked_positions(START, SLOPE, input.width, input.height))
    }

    fn solve_part2(&self, input: &Self::Input) -> Self::Output2 {
        [
            Slope(1, 1), // right 1, down 1
            Slope(3, 1), // right 3, down 1 (SLOPE)
            Slope(5, 1), // right 5, down 1
            Slope(7, 1), // right 7, down 1
            Slope(1, 2), // right 1, down 2
        ]
        .iter()
        .map(|&slope| {
            input.count_trees(&checked_positions(START, slope, input.width, input.height))
        })
        .product()
    }

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        let input = BufReader::new(r)
            .lines()
            .flatten()
            .map(|line| {
                line.bytes()
                    .map(|byte| GridCell::from(byte as char).unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let width = input.first().map_or(0, |row| row.len());
        let height = input.len();

        assert!(input.iter().all(|row| row.len() == width));

        Grid {
            cells: input,
            width,
            height,
        }
    }
}

impl GridCell {
    fn from(c: char) -> Option<Self> {
        match c {
            '.' => Some(GridCell::Open),
            '#' => Some(GridCell::Tree),
            _ => None,
        }
    }
}

impl Display for GridCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GridCell::Open => write!(f, "."),
            GridCell::Tree => write!(f, "#"),
        }
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.cells {
            for cell in row {
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
