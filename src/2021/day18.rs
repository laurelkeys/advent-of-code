//! --- Day 18: Snailfish ---

use crate::solver::Solver;
use std::io::{self, BufRead, BufReader};

/// https://adventofcode.com/2021/day/18
pub struct Day18;

#[derive(Clone, Debug)]
pub enum Number {
    Elem(u8),
    Pair(Box<(Number, Number)>),
}

impl Solver for Day18 {
    type Input = Vec<Number>;
    type Output1 = u32;
    type Output2 = u32;

    /// To add two snailfish numbers, form a pair from the left and right parameters
    /// of the addition operator. There's only one problem: snailfish numbers must
    /// always be reduced, and the process of adding two snailfish numbers can result
    /// in snailfish numbers that need to be reduced.
    ///
    /// The magnitude of a pair is 3 times the magnitude of its left element plus 2
    /// times the magnitude of its right element. The magnitude of a regular number
    /// is just that number.
    ///
    /// Add up all of the snailfish numbers from the homework assignment in the order
    /// they appear. What is the magnitude of the final sum?
    fn solve_part1(&self, input: &Self::Input) -> Self::Output1 {
        let input_sum = input.clone().into_iter().reduce(|acc, number| {
            let sum = Number::Pair(Box::new((acc, number)));
            reduce_number(sum)
        });

        fn magnitude(number: &Number) -> u32 {
            match number {
                Number::Elem(value) => *value as u32,
                Number::Pair(pair) => magnitude(&pair.0) * 3 + magnitude(&pair.1) * 2,
            }
        }

        magnitude(&input_sum.unwrap())
    }

    ///
    fn solve_part2(&self, input: &Self::Input) -> Self::Output2 {
        todo!()
    }

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        BufReader::new(r)
            .lines()
            .flatten()
            .map(|line| {
                let number = parse_number(&mut line.chars());
                assert!(matches!(number, Number::Pair(_)));
                number
            })
            .collect::<Vec<_>>()
    }
}

fn parse_number(it: &mut impl Iterator<Item = char>) -> Number {
    match it.next() {
        Some('[') => {
            let lhs = parse_number(it);
            assert_eq!(it.next(), Some(','));
            let rhs = parse_number(it);
            assert_eq!(it.next(), Some(']'));
            Number::Pair(Box::new((lhs, rhs)))
        }
        Some(value) => {
            assert!(value.is_ascii_digit());
            Number::Elem(value as u8 - b'0')
        }
        _ => unreachable!(),
    }
}

/// To reduce a snailfish number, you must repeatedly do the first action in this
/// list that applies to the snailfish number:
/// - If any pair is nested inside four pairs, the leftmost such pair explodes.
/// - If any regular number is 10 or greater, the leftmost such regular number splits.
///
/// Once no action in the above list applies, the snailfish number is reduced.
/// During reduction, at most one action applies, after which the process returns
/// to the top of the list of actions.
fn reduce_number(mut number: Number) -> Number {
    loop {
        if explode_any_pair_nested_4_times(&mut number) {
            continue;
        }
        if split_any_number_greater_than_9(&mut number) {
            continue;
        }
        break;
    }

    number
}

/// To explode a pair, the pair's left value is added to the first regular number
/// to the left of the exploding pair (if any), and the pair's right value is added
/// to the first regular number to the right of the exploding pair (if any).
/// Exploding pairs will always consist of two regular numbers. Then, the entire
/// exploding pair is replaced with the regular number 0.
fn explode_any_pair_nested_4_times(number: &mut Number) -> bool {
    fn add_left(number: &mut Number, value: u8) {
        match number {
            Number::Elem(curr_value) => *curr_value += value,
            Number::Pair(pair) => add_left(&mut pair.0, value),
        }
    }

    fn add_right(number: &mut Number, value: u8) {
        match number {
            Number::Elem(curr_value) => *curr_value += value,
            Number::Pair(pair) => add_right(&mut pair.1, value),
        }
    }

    enum Exploded {
        No,
        Yes(Option<u8>, Option<u8>),
    }

    fn explode_any_pair_nested_n_times(number: &mut Number, n: u8) -> Exploded {
        match number {
            Number::Elem(_) => Exploded::No,
            Number::Pair(pair) => {
                if n == 0 {
                    match (&pair.0, &pair.1) {
                        (&Number::Elem(lvalue), &Number::Elem(rvalue)) => {
                            *number = Number::Elem(0);
                            Exploded::Yes(Some(lvalue), Some(rvalue))
                        }
                        _ => unreachable!(),
                    }
                } else {
                    let (lhs, rhs) = (&mut pair.0, &mut pair.1);
                    match explode_any_pair_nested_n_times(lhs, n - 1) {
                        Exploded::No => match explode_any_pair_nested_n_times(rhs, n - 1) {
                            Exploded::No => Exploded::No,
                            Exploded::Yes(None, None) => Exploded::Yes(None, None),
                            Exploded::Yes(None, Some(rvalue)) => Exploded::Yes(None, Some(rvalue)),
                            Exploded::Yes(Some(lvalue), None) => {
                                add_right(lhs, lvalue);
                                Exploded::Yes(None, None)
                            }
                            Exploded::Yes(Some(lvalue), Some(rvalue)) => {
                                add_right(lhs, lvalue);
                                Exploded::Yes(None, Some(rvalue))
                            }
                        },
                        Exploded::Yes(None, None) => Exploded::Yes(None, None),
                        Exploded::Yes(None, Some(rvalue)) => {
                            add_left(rhs, rvalue);
                            Exploded::Yes(None, None)
                        }
                        Exploded::Yes(Some(lvalue), None) => Exploded::Yes(Some(lvalue), None),
                        Exploded::Yes(Some(lvalue), Some(rvalue)) => {
                            add_left(rhs, rvalue);
                            Exploded::Yes(Some(lvalue), None)
                        }
                    }
                }
            }
        }
    }

    matches!(
        explode_any_pair_nested_n_times(number, 4),
        Exploded::Yes(_, _) // Exploded::Yes(None, None)
    )
}

/// To split a regular number, replace it with a pair; the left element of the pair
/// should be the regular number divided by two and rounded down, while the right
/// element of the pair should be the regular number divided by two and rounded up.
fn split_any_number_greater_than_9(number: &mut Number) -> bool {
    match number {
        Number::Elem(value) => {
            if *value > 9 {
                let lhs = Number::Elem(*value / 2);
                let rhs = Number::Elem((*value + 1) / 2);
                *number = Number::Pair(Box::new((lhs, rhs)));
                true
            } else {
                false
            }
        }
        Number::Pair(pair) => {
            if split_any_number_greater_than_9(&mut pair.0) {
                true
            } else {
                split_any_number_greater_than_9(&mut pair.1)
            }
        }
    }
}
