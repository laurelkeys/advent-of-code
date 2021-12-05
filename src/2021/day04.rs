//! --- Day 4: Giant Squid ---

use crate::solver::Solver;
use std::{
    convert::TryInto,
    io::{self, BufRead, BufReader},
};

/// https://adventofcode.com/2021/day/4
pub struct Day04;

#[allow(clippy::unusual_byte_groupings)]
const MARKED_BINGO_ROW: [u32; 5] = [
    0b11111_00000_00000_00000_00000, // 1st row
    0b00000_11111_00000_00000_00000, // 2nd row
    0b00000_00000_11111_00000_00000, // 3rd row
    0b00000_00000_00000_11111_00000, // 4th row
    0b00000_00000_00000_00000_11111, // 5th row
];

#[allow(clippy::unusual_byte_groupings)]
const MARKED_BINGO_COL: [u32; 5] = [
    0b10000_10000_10000_10000_10000, // 1st col
    0b01000_01000_01000_01000_01000, // 2nd col
    0b00100_00100_00100_00100_00100, // 3rd col
    0b00010_00010_00010_00010_00010, // 4th col
    0b00001_00001_00001_00001_00001, // 5th col
];

#[derive(Clone, Debug)]
pub struct Board {
    numbers: [u8; 25],
    marked: u32,
    wins: bool,
}

impl Board {
    pub fn new(numbers: [u8; 25]) -> Self {
        Self {
            numbers,
            marked: 0,
            wins: false,
        }
    }

    fn row_col_from_index(index: usize) -> (usize, usize) {
        assert!(index < 25);
        let row = index / 5;
        let col = index % 5;
        (row, col)
    }

    fn is_marked(&self, number: u8) -> bool {
        if let Some(index) = self.numbers.iter().position(|n| *n == number) {
            (self.marked >> (24 - index)) & 1 == 1
        } else {
            false
        }
    }

    fn mark(&mut self, number: u8) {
        if let Some(index) = self.numbers.iter().position(|n| *n == number) {
            let (i, j) = Board::row_col_from_index(index);

            // @Note: (0, 0) is at `1 << 24`.
            self.marked |= 1 << (24 - index);

            if (self.marked & MARKED_BINGO_ROW[i] == MARKED_BINGO_ROW[i])
                || (self.marked & MARKED_BINGO_COL[j] == MARKED_BINGO_COL[j])
            {
                self.wins = true; // bingo!
            }
        }
    }
}

impl Solver for Day04 {
    type Input = (Vec<u8>, Vec<Board>);
    type Output1 = u32;
    type Output2 = u32;

    /// The score of the winning board can be calculated by finding the sum of all
    /// unmarked numbers on that board; then, multiply that sum by the number that
    /// was just called when the board won to get the final score.
    ///
    /// Figure out which board will win first. What will your final score be if you
    /// choose that board?
    fn solve_part1(&self, input: &Self::Input) -> Self::Output1 {
        let (numbers, boards) = input;

        let boards_and_win_rounds = boards
            .iter()
            .map(|board| {
                let mut board = board.clone();
                for (round, &number) in numbers.iter().enumerate() {
                    board.mark(number);
                    if board.wins {
                        return (board, Some(round));
                    }
                }
                (board, None)
            })
            .collect::<Vec<(Board, Option<usize>)>>();

        let (winning_board, winning_round) = boards_and_win_rounds
            .iter()
            .min_by_key(|(_, win_round)| win_round)
            .unwrap();

        let winning_round = winning_round.unwrap();

        let sum_of_unmarked_numbers: u32 = winning_board
            .numbers
            .iter()
            .filter_map(|&n| (!winning_board.is_marked(n)).then(|| n as u32))
            .sum();

        sum_of_unmarked_numbers * numbers[winning_round] as u32
    }

    /// Figure out which board will win last. Once it wins, what would its final score be?
    fn solve_part2(&self, input: &Self::Input) -> Self::Output2 {
        let (numbers, boards) = input;

        let (last_winning_board, winning_round) = boards
            .iter()
            .map(|board| {
                let mut board = board.clone();
                for (round, &number) in numbers.iter().enumerate() {
                    board.mark(number);
                    if board.wins {
                        return (board, Some(round));
                    }
                }
                (board, None)
            })
            .max_by_key(|(_, win_round)| *win_round)
            .unwrap();

        let winning_round = winning_round.unwrap();

        let sum_of_unmarked_numbers: u32 = last_winning_board
            .numbers
            .iter()
            .filter_map(|&n| (!last_winning_board.is_marked(n)).then(|| n as u32))
            .sum();

        sum_of_unmarked_numbers * numbers[winning_round] as u32
    }

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        let lines = BufReader::new(r).lines().flatten().collect::<Vec<String>>();

        let numbers = lines
            .first()
            .unwrap()
            .split(',')
            .map(|n| n.parse().unwrap())
            .collect::<Vec<u8>>();

        let boards = lines[1..]
            .chunks(6)
            .map(|board| {
                let numbers = board
                    .iter()
                    .flat_map(|row| {
                        row.split(' ')
                            .filter(|n| !n.is_empty())
                            .map(|n| n.parse().unwrap())
                    })
                    .collect::<Vec<u8>>();

                Board::new(numbers.try_into().unwrap())
            })
            .collect::<Vec<Board>>();

        (numbers, boards)
    }
}
