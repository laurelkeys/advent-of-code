//! --- Day 13: Transparent Origami ---

use crate::solver::Solver;
use std::{
    collections::HashSet,
    fmt::{Display, Write},
    io::{self, BufRead, BufReader},
};

/// https://adventofcode.com/2021/day/13
pub struct Day13;

#[derive(Clone, Copy, Debug)]
pub enum FoldAlong {
    X(i32),
    Y(i32),
}

#[derive(Clone, Debug)]
pub struct TransparentPaper {
    dots: HashSet<(i32, i32)>,
    max_x: i32,
    max_y: i32,
}

impl Display for TransparentPaper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            &std::iter::once('\n')
                .chain((0..=self.max_y).flat_map(|y| {
                    (0..=self.max_x)
                        .map(move |x| {
                            if self.dots.contains(&(x, y)) {
                                '█'
                            } else {
                                ' '
                            }
                        })
                        .chain(std::iter::once('\n'))
                }))
                .collect::<String>(),
        )
    }
}

impl TransparentPaper {
    fn new(dots: HashSet<(i32, i32)>) -> Self {
        let (max_x, max_y) = dots.iter().fold((0, 0), |(max_x, max_y), (x, y)| {
            (max_x.max(*x), max_y.max(*y))
        });

        Self { dots, max_x, max_y }
    }

    fn fold_along(&self, fold: &FoldAlong) -> Self {
        match *fold {
            FoldAlong::X(fx) => Self {
                dots: self
                    .dots
                    .iter()
                    .map(|&(x, y)| (if x <= fx { x } else { fx - (x - fx) }, y))
                    .collect(),
                max_x: fx,
                max_y: self.max_y,
            },
            FoldAlong::Y(fy) => Self {
                dots: self
                    .dots
                    .iter()
                    .map(|&(x, y)| (x, if y <= fy { y } else { fy - (y - fy) }))
                    .collect(),
                max_x: self.max_x,
                max_y: fy,
            },
        }
    }
}

impl Solver for Day13 {
    type Input = (TransparentPaper, Vec<FoldAlong>);
    type Output1 = usize;
    type Output2 = String;

    /// Each instruction indicates a line on the transparent paper and wants you to fold
    /// the paper up (for horizontal y=... lines) or left (for vertical x=... lines).
    ///
    /// How many dots are visible after completing just the first fold instruction?
    fn solve_part1(&self, input: &Self::Input) -> Self::Output1 {
        let (paper, folds) = input;

        paper.fold_along(&folds[0]).dots.len()
    }

    /// Finish folding the transparent paper according to the instructions.
    /// The manual says the code is always eight capital letters.
    ///
    /// What code do you use to activate the infrared thermal imaging camera system?
    fn solve_part2(&self, input: &Self::Input) -> Self::Output2 {
        let (paper, folds) = input;

        let mut paper = paper.clone();
        for fold in folds {
            paper = paper.fold_along(fold);
        }

        paper.to_string()
    }

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        let mut input = BufReader::new(r).lines().flatten();

        let coords = input
            .by_ref()
            .take_while(|line| !line.is_empty())
            .map(|coord| {
                let (x, y) = coord.split_once(',').unwrap();
                (x.parse().unwrap(), y.parse().unwrap())
            })
            .collect::<HashSet<_>>();

        let folds = input
            .map(|instr| {
                let (orientation, fold_line) = instr
                    .strip_prefix("fold along ")
                    .and_then(|instr| instr.split_once('='))
                    .unwrap();

                match orientation {
                    "x" => FoldAlong::X(fold_line.parse().unwrap()),
                    "y" => FoldAlong::Y(fold_line.parse().unwrap()),
                    _ => unreachable!(),
                }
            })
            .collect::<Vec<_>>();

        (TransparentPaper::new(coords), folds)
    }
}
