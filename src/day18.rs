//! --- Day 18: Operation Order ---

use io::BufRead;

use crate::solver::Solver;
use core::panic;
use std::io::{self, BufReader};

/// https://adventofcode.com/2020/day/18
pub struct Day18;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Op {
    Add, // +
    Mul, // *
}

impl From<char> for Op {
    fn from(c: char) -> Self {
        match c {
            '+' => Op::Add,
            '*' => Op::Mul,
            _ => panic!(),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Token {
    Number(i64),
    BinOp(Op),
    LParen,
    RParen,
}

use Op::*;
use Token::*;

fn lex(expr: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    for token in expr.chars() {
        match token {
            '0'..='9' => {
                let number = token as i64 - '0' as i64;
                if let Some(&Number(n)) = tokens.last() {
                    tokens.pop();
                    tokens.push(Number(number + 10 * n));
                } else {
                    tokens.push(Number(number));
                }
            }
            '(' => tokens.push(LParen),
            ')' => tokens.push(RParen),
            '+' => tokens.push(BinOp(Add)),
            '*' => tokens.push(BinOp(Mul)),
            _ => assert_eq!(token, ' '),
        }
    }

    tokens
}

fn parse<P>(expr: &str, precedence: P) -> Vec<Token>
where
    P: Fn(Op) -> usize,
{
    let mut result = Vec::new();
    let mut partial = Vec::new();

    'parsing: for token in lex(expr) {
        match token {
            Token::Number(_) => result.push(token),
            Token::BinOp(op) => {
                while let Some(top) = partial.last().cloned() {
                    match top {
                        BinOp(top_op) => {
                            if precedence(top_op) < precedence(op) {
                                break;
                            }
                            result.push(partial.pop().unwrap())
                        }
                        LParen => break,
                        _ => unreachable!(),
                    }
                }
                partial.push(token);
            }
            Token::LParen => partial.push(token),
            Token::RParen => {
                while let Some(top) = partial.pop() {
                    if top == LParen {
                        continue 'parsing;
                    }
                    result.push(top)
                }
                unreachable!()
            }
        }
    }

    while let Some(top) = partial.pop() {
        assert!(top != LParen);
        result.push(top)
    }

    result
}

fn eval<P>(expr: &str, precedence: P) -> i64
where
    P: Fn(Op) -> usize,
{
    let mut stack = Vec::new();

    for token in parse(expr, precedence) {
        match token {
            Number(n) => stack.push(n),
            BinOp(op) => {
                let lhs = stack.pop().unwrap();
                let rhs = stack.pop().unwrap();
                stack.push(match op {
                    Add => lhs + rhs,
                    Mul => lhs * rhs,
                })
            }
            _ => unreachable!(),
        }
    }

    stack.pop().unwrap_or(0)
}

impl Solver for Day18 {
    type Input = Vec<String>;
    type Output1 = i64;
    type Output2 = i64;

    fn solve_part1(&self, input: &Self::Input) -> Self::Output1 {
        input.iter().map(|expr| eval(expr, |_: Op| 0)).sum() // equal precedence
    }

    fn solve_part2(&self, input: &Self::Input) -> Self::Output2 {
        input
            .iter()
            .map(|expr| {
                eval(expr, |op: Op| match op {
                    Op::Add => 1, // higher precedence
                    Op::Mul => 0, // lower precedence
                })
            })
            .sum()
    }

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        BufReader::new(r).lines().flatten().collect()
    }
}
