//! --- Day 14: Docking Data ---

use crate::solver::Solver;
use std::{
    collections::HashMap,
    io::{self, BufRead, BufReader},
};

/// https://adventofcode.com/2020/day/14
pub struct Day14;

#[derive(Copy, Clone)]
pub struct Mask {
    clear: usize, // 1s where the mask had '0's
    set: usize,   // 1s where the mask had '1's
}

#[derive(Copy, Clone)]
pub struct Mem {
    addr: usize,
    value: usize,
}

pub enum Instruction {
    Mask(Mask),
    Write(Mem),
}

impl Solver for Day14 {
    type Input = Vec<Instruction>;
    type Output1 = usize;
    type Output2 = usize;

    fn solve_part1(&self, input: &Self::Input) -> Self::Output1 {
        let mut memory = HashMap::new();
        let mut mask = Mask::new();

        for instr in input {
            match instr {
                Instruction::Mask(new_mask) => mask = *new_mask,
                Instruction::Write(Mem { addr, value }) => {
                    memory.insert(*addr, mask.overwrite(*value));
                }
            }
        }

        memory.values().sum()
    }

    fn solve_part2(&self, input: &Self::Input) -> Self::Output2 {
        let mut memory = HashMap::new();
        let mut mask = Mask::new();

        for instr in input {
            match instr {
                Instruction::Mask(new_mask) => mask = *new_mask,
                Instruction::Write(Mem { addr, value }) => {
                    for floating_addr in mask.decode(*addr) {
                        memory.insert(floating_addr, *value);
                    }
                }
            }
        }

        memory.values().sum()
    }

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        use regex::Regex; // https://docs.rs/regex/1.4.2/regex/#syntax

        let mask_re = Regex::new(r"mask = (?P<bitmask>[01X]{36})").unwrap();
        let mem_re = Regex::new(r"mem\[(?P<address>\d+)\] = (?P<value>\d+)").unwrap();

        BufReader::new(r)
            .lines()
            .flatten()
            .map(|line| {
                if mem_re.is_match(&line) {
                    let captures = mem_re.captures(&line).unwrap();
                    let address = captures.name("address").unwrap().as_str();
                    let value = captures.name("value").unwrap().as_str();

                    Instruction::Write(Mem {
                        addr: address.parse::<usize>().unwrap(),
                        value: value.parse::<usize>().unwrap(),
                    })
                } else {
                    let captures = mask_re.captures(&line).unwrap();
                    let bitmask = captures.name("bitmask").unwrap().as_str();

                    Instruction::Mask(bitmask.char_indices().fold(
                        Mask::new(),
                        |mut mask, (i, bit)| {
                            match bit {
                                '0' => mask.clear |= 1 << (35 - i),
                                '1' => mask.set |= 1 << (35 - i),
                                _ => {}
                            };
                            mask
                        },
                    ))
                }
            })
            .collect()
    }
}

impl Mask {
    fn new() -> Self {
        Mask { clear: 0, set: 0 }
    }

    fn overwrite(&self, value: usize) -> usize {
        (value & !self.clear) | self.set
    }

    fn decode(&self, addr: usize) -> Vec<usize> {
        let mut floating_bits = Vec::new();

        {
            let mut bit = 0;
            let mut xs = !(self.clear | self.set); // 1s where the mask had 'X's
            xs &= !(!0 << 36); // limit to the first 36 bits
            while xs > 0 {
                if (xs & 1) == 1 {
                    floating_bits.push(bit);
                }
                xs >>= 1;
                bit += 1;
            }
        }

        (0..(1 << floating_bits.len()))
            .map(|mut x_bit| {
                let mut addr = addr | self.set;
                for bit in &floating_bits {
                    match x_bit & 1 {
                        0 => addr &= !(1 << bit),
                        1 => addr |= 1 << bit,
                        _ => unreachable!(),
                    }
                    x_bit >>= 1;
                }
                addr
            })
            .collect()
    }
}

/*
use std::fmt::Display;

impl Display for Mask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            format!("{:036b}", self.clear)
                .chars()
                .zip(format!("{:036b}", self.set).chars())
                .map(|(clear, set)| match (clear, set) {
                    ('1', '0') => '0',
                    (_, '1') => '1',
                    _ => 'X',
                })
                .collect::<String>()
        )
    }
}
*/
