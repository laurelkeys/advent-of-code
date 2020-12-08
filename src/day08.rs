//! --- Day 8: Handheld Halting ---

use crate::solver::Solver;
use std::{
    collections::HashSet,
    io::{self, BufRead, BufReader},
    str::FromStr,
};

/// https://adventofcode.com/2020/day/8
pub struct Day08;

/// Each instruction consists of an operation and an argument.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Instruction(Operation, i32);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Operation {
    /// Increases or decreases a single global value called
    /// the accumulator by the value given in the argument.
    Acc,
    /// Jumps to a new instruction relative to itself,
    /// using the argument value for the offset.
    Jmp,
    /// Stands for no operation - it does nothing.
    Nop,
}

/// Represents the output of a program execution, with the final
/// value of the accumulator if it terminates, or its value immediately
/// before any instruction is executed a second time if it loops infinitely.
pub struct Output {
    accumulator: i32,
    terminates: bool,
}

fn execute(program: &[Instruction]) -> Output {
    let mut accumulator = 0;

    // The program is supposed to terminate by attempting to execute
    // an instruction immediately after the last instruction in the file.
    let terminal = program.len() as i32;

    let mut current = 0;
    let mut executed = HashSet::new();

    let terminates = loop {
        if current == terminal {
            break true;
        }

        if executed.contains(&current) {
            break false;
        }

        let next = match program.get(current as usize).unwrap() {
            Instruction(Operation::Nop, _) => current + 1,
            Instruction(Operation::Jmp, offset) => current + offset,
            Instruction(Operation::Acc, value) => {
                accumulator += value;
                current + 1
            }
        };

        executed.insert(current);
        current = next;
    };

    Output {
        accumulator,
        terminates,
    }
}

impl Solver for Day08 {
    type Input = Vec<Instruction>;
    type Output1 = i32;
    type Output2 = i32;

    fn solve_part1(&self, input: &Self::Input) -> Self::Output1 {
        // Immediately before any instruction is executed a second time,
        // what value is in the accumulator?
        execute(input).accumulator
    }

    fn solve_part2(&self, input: &Self::Input) -> Self::Output2 {
        // Somewhere in the program, either a jmp is supposed to be a nop,
        // or a nop is supposed to be a jmp.
        //
        // By changing exactly one jmp or nop, you can repair the boot code
        // and make it terminate correctly.
        //
        // What is the value of the accumulator after the program terminates?
        if let Output {
            accumulator,
            terminates: true,
        } = execute(input)
        {
            return accumulator;
        }

        // @Cleanup: is this clone really necessary?
        let mut modified_input = input.clone();

        for (i, instruction) in input.iter().enumerate() {
            match instruction {
                Instruction(Operation::Nop, argument) => {
                    modified_input[i] = Instruction(Operation::Jmp, *argument);
                    if let Output {
                        accumulator,
                        terminates: true,
                    } = execute(&modified_input)
                    {
                        return accumulator;
                    }
                    modified_input[i] = *instruction;
                }
                Instruction(Operation::Jmp, offset) => {
                    modified_input[i] = Instruction(Operation::Nop, *offset);
                    if let Output {
                        accumulator,
                        terminates: true,
                    } = execute(&modified_input)
                    {
                        return accumulator;
                    }
                    modified_input[i] = *instruction;
                }
                _ => {}
            }
        }

        unreachable!()
    }

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        BufReader::new(r)
            .lines()
            .flatten()
            .map(|instruction| {
                let (op, arg) = {
                    let mut instruction = instruction.splitn(2, ' ');
                    let op = instruction.next().unwrap();
                    let arg = instruction.next().unwrap();
                    (Operation::from_str(op), arg.parse::<i32>())
                };
                Instruction(op.unwrap(), arg.unwrap())
            })
            .collect()
    }
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "acc" => Ok(Operation::Acc),
            "jmp" => Ok(Operation::Jmp),
            "nop" => Ok(Operation::Nop),
            _ => Err(()),
        }
    }
}
