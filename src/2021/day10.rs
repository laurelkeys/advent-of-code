//! --- Day 10: Syntax Scoring ---

use crate::solver::Solver;
use std::io::{self, BufRead, BufReader};

/// https://adventofcode.com/2021/day/10
pub struct Day10;

enum SyntaxError {
    /// `Vec<char>` is the sequence of unclosed chunks.
    Incomplete(Vec<char>),
    /// `char` is the first illegal character in the line.
    Corrupted(char),
}

fn syntax_error(chunks: &str) -> SyntaxError {
    let mut openings = vec![];
    for c in chunks.chars() {
        if "([{<".contains(c) {
            openings.push(c);
        } else {
            match openings.pop() {
                Some(o) => match o {
                    '(' if c != ')' => return SyntaxError::Corrupted(c),
                    '[' if c != ']' => return SyntaxError::Corrupted(c),
                    '{' if c != '}' => return SyntaxError::Corrupted(c),
                    '<' if c != '>' => return SyntaxError::Corrupted(c),
                    _ => {}
                },
                None => return SyntaxError::Corrupted(c),
            }
        }
    }
    assert!(!openings.is_empty()); // otherwise it wouldn't be incomplete
    SyntaxError::Incomplete(openings)
}

impl Solver for Day10 {
    type Input = Vec<String>;
    type Output1 = i32;
    type Output2 = i64;

    /// The navigation subsystem syntax is made of several lines containing chunks.
    /// Every chunk must open and close with one of four legal matching pairs: (), [], {}, <>.
    /// Some lines are incomplete, but others are corrupted. A corrupted line is one where a
    /// chunk closes with the wrong character. Find and discard the corrupted lines first.
    ///
    /// To calculate the syntax error score for a line, take the first illegal character on
    /// the line and look it up in the following table:
    /// - ) : 3 points.
    /// - ] : 57 points.
    /// - } : 1197 points.
    /// - > : 25137 points.
    ///
    /// Find the first illegal character in each corrupted line of the navigation subsystem.
    /// What is the total syntax error score for those errors?
    fn solve_part1(&self, input: &Self::Input) -> Self::Output1 {
        input
            .iter()
            .filter_map(|chunks| match syntax_error(chunks) {
                SyntaxError::Incomplete(_) => None,
                SyntaxError::Corrupted(chunk) => Some(chunk),
            })
            .map(|chunk| match chunk {
                ')' => 3,
                ']' => 57,
                '}' => 1197,
                '>' => 25137,
                _ => unreachable!(),
            })
            .sum()
    }

    /// Now, discard the corrupted lines. The remaining lines are incomplete.
    /// To repair the navigation subsystem, you just need to figure out the sequence of closing
    /// characters that complete all open chunks in the line.
    ///
    /// The score is determined by considering the completion string character-by-character.
    /// Start with a total score of 0. Then, for each character, multiply the total score by 5
    /// and then increase the total score by the point value given for the character in the
    /// following table:
    /// - ): 1 point.
    /// - ]: 2 points.
    /// - }: 3 points.
    /// - >: 4 points.
    ///
    /// Autocomplete tools are an odd bunch: the winner is found by sorting all of the scores and
    /// then taking the middle score. (There will always be an odd number of scores to consider.)
    fn solve_part2(&self, input: &Self::Input) -> Self::Output2 {
        let mut scores = input
            .iter()
            .filter_map(|chunks| match syntax_error(chunks) {
                SyntaxError::Incomplete(open_chunks) => Some(open_chunks),
                SyntaxError::Corrupted(_) => None,
            })
            .map(|open_chunks| {
                open_chunks.into_iter().rev().map(|chunk| match chunk {
                    '(' => ')',
                    '[' => ']',
                    '{' => '}',
                    '<' => '>',
                    _ => unreachable!(),
                })
            })
            .map(|closing_chunks| {
                closing_chunks.fold(0_i64, |score, chunk| match chunk {
                    ')' => 5 * score + 1,
                    ']' => 5 * score + 2,
                    '}' => 5 * score + 3,
                    '>' => 5 * score + 4,
                    _ => unreachable!(),
                })
            })
            .collect::<Vec<i64>>();

        let median_index = scores.len() / 2; // 0-based
        scores.select_nth_unstable(median_index);
        scores[median_index]
    }

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        BufReader::new(r).lines().flatten().collect::<Vec<String>>()
    }
}
