//! --- Day 8: Handheld Halting ---

use crate::solver::Solver;
use std::{
    collections::HashSet,
    io::{self, BufRead, BufReader},
};

/// https://adventofcode.com/2020/day/8
pub struct Day08;

#[derive(Copy, Clone, Debug)]
/// Each instruction consists of an operation (opcode) and an argument.
pub enum Instruction {
    /// Increases or decreases a single global value called
    /// the accumulator, by the value given in the argument.
    Acc(i32),
    /// Jumps to a new instruction relative to itself,
    /// using the argument value for the offset.
    Jmp(i32),
    /// Stands for no operation - it does nothing.
    Nop(i32),
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

    let mut current = 0; // program counter
    let mut visited = HashSet::new();

    let terminates = loop {
        if current == terminal {
            break true;
        }

        visited.insert(current);

        current += match program[current as usize] {
            Instruction::Acc(value) => {
                accumulator += value;
                1
            }
            Instruction::Jmp(offset) => offset,
            Instruction::Nop(_) => 1,
        };

        if visited.contains(&current) {
            break false;
        }
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
        let mut modified_input = input.clone();

        for (i, instruction) in input.iter().enumerate() {
            modified_input[i] = match *instruction {
                Instruction::Jmp(offset) => Instruction::Nop(offset),
                Instruction::Nop(arg) if arg > 1 => Instruction::Jmp(arg),
                _ => continue,
            };

            if let Output {
                accumulator,
                terminates: true,
            } = execute(&modified_input)
            {
                return accumulator;
            }

            modified_input[i] = *instruction; // swap back the original instruction
        }

        unreachable!()
    }

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        BufReader::new(r)
            .lines()
            .flatten()
            .map(|line| {
                let mut instruction = line.splitn(2, ' ');
                let op = instruction.next().unwrap();
                let arg = instruction.next().unwrap().parse::<i32>().unwrap();
                match op {
                    "acc" => Instruction::Acc(arg),
                    "jmp" => Instruction::Jmp(arg),
                    "nop" => Instruction::Nop(arg),
                    _ => unreachable!(),
                }
            })
            .collect()
    }
}
