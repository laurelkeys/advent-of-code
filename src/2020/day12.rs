//! --- Day 12: Rain Risk ---

use crate::solver::Solver;
use std::{
    io::{self, BufRead, BufReader},
    ops::{AddAssign, Mul},
};

/// https://adventofcode.com/2020/day/12
pub struct Day12;

#[derive(Copy, Clone)]
pub struct Point(i32, i32);

#[derive(Copy, Clone)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Ship {
    position: Point,
    direction: Direction,
}

pub enum Instruction {
    TurnLeft(usize),
    TurnRight(usize),
    MoveForward(usize),
    Move(Direction, usize),
}

impl Solver for Day12 {
    type Input = Vec<Instruction>;
    type Output1 = i32;
    type Output2 = i32;

    fn solve_part1(&self, input: &Self::Input) -> Self::Output1 {
        let mut ship = Ship {
            direction: Direction::East,
            position: Point(0, 0),
        };

        for instruction in input {
            match *instruction {
                Instruction::TurnLeft(angle) => ship.turn(-(angle as i32)),
                Instruction::TurnRight(angle) => ship.turn(angle as i32),
                Instruction::MoveForward(value) => ship.position += ship.direction * value,
                Instruction::Move(direction, value) => ship.position += direction * value,
            }
        }

        ship.position.0.abs() + ship.position.1.abs()
    }

    fn solve_part2(&self, input: &Self::Input) -> Self::Output2 {
        let mut ship = Ship {
            direction: Direction::East,
            position: Point(0, 0),
        };

        let mut waypoint = Point(10, 1);

        for instruction in input {
            match *instruction {
                Instruction::TurnLeft(angle) => waypoint.turn(-(angle as i32)),
                Instruction::TurnRight(angle) => waypoint.turn(angle as i32),
                Instruction::MoveForward(times) => ship.position += waypoint * times,
                Instruction::Move(direction, value) => waypoint += direction * value,
            }
        }

        ship.position.0.abs() + ship.position.1.abs()
    }

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        BufReader::new(r)
            .lines()
            .flatten()
            .map(|line| {
                let (action, value) = line.split_at(1);
                let value = value.parse::<usize>().unwrap();
                match action {
                    "L" => Instruction::TurnLeft(value),
                    "R" => Instruction::TurnRight(value),
                    "F" => Instruction::MoveForward(value),
                    "N" => Instruction::Move(Direction::North, value),
                    "S" => Instruction::Move(Direction::South, value),
                    "E" => Instruction::Move(Direction::East, value),
                    "W" => Instruction::Move(Direction::West, value),
                    _ => unreachable!(),
                }
            })
            .collect()
    }
}

fn mod4(x: i32) -> i32 {
    ((x % 4) + 4) % 4
}

impl Ship {
    fn turn(&mut self, angle: i32) {
        use Direction::*;
        self.direction = [North, East, South, West][mod4(
            match self.direction {
                North => 0,
                East => 1,
                South => 2,
                West => 3,
            } + angle / 90,
        ) as usize];
    }
}

impl Point {
    fn turn(&mut self, angle: i32) {
        *self = match mod4(angle / 90) {
            1 => Point(self.1, -self.0),
            2 => Point(-self.0, -self.1),
            3 => Point(-self.1, self.0),
            _ => unreachable!(),
        };
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl Mul<usize> for Point {
    type Output = Point;

    fn mul(self, rhs: usize) -> Self::Output {
        let val = rhs as i32;
        Point(self.0 * val, self.1 * val)
    }
}

impl Mul<usize> for Direction {
    type Output = Point;

    fn mul(self, rhs: usize) -> Self::Output {
        let val = rhs as i32;
        match self {
            Direction::North => Point(0, val),
            Direction::East => Point(val, 0),
            Direction::South => Point(0, -val),
            Direction::West => Point(-val, 0),
        }
    }
}
