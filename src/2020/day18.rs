//! --- Day 18: Operation Order ---

use crate::solver::Solver;
use std::io::{self, BufRead, BufReader};

/// https://adventofcode.com/2020/day/18
pub struct Day18;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Op {
    Add,
    Mul,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Token {
    Number(i64),
    BinOp(Op),
    LParen,
    RParen,
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

use Op::*;
use Token::*;

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

fn parse<P>(expr: &str, precedence: P) -> Vec<Token>
where
    P: Fn(Op) -> usize,
{
    let mut tokens = Vec::new();
    let mut stack = Vec::new();

    'parsing: for token in lex(expr) {
        match token {
            Token::Number(_) => tokens.push(token),
            Token::BinOp(op) => {
                while let Some(top) = stack.last().cloned() {
                    match top {
                        BinOp(top_op) => {
                            if precedence(top_op) >= precedence(op) {
                                tokens.push(stack.pop().unwrap())
                            } else {
                                break;
                            }
                        }
                        LParen => break,
                        _ => unreachable!(),
                    }
                }
                stack.push(token);
            }
            Token::LParen => stack.push(token),
            Token::RParen => loop {
                match stack.pop() {
                    Some(LParen) => continue 'parsing,
                    Some(top) => tokens.push(top),
                    _ => unreachable!(),
                }
            },
        }
    }

    while let Some(top) = stack.pop() {
        assert!(top != LParen);
        tokens.push(top)
    }

    tokens
}

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
