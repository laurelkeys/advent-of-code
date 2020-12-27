//! --- Day 25: Combo Breaker ---

use crate::solver::Solver;
use std::io::{self, BufRead, BufReader};

/// https://adventofcode.com/2020/day/25
pub struct Day25;

impl Solver for Day25 {
    type Input = (usize, usize);
    type Output1 = usize;
    type Output2 = String;

    fn solve_part1(&self, input: &Self::Input) -> Self::Output1 {
        let (card_pub_key, door_pub_key) = *input;

        fn loop_size(pub_key: usize) -> usize {
            let mut loop_size = 0;
            let mut value = 1;
            while pub_key != value {
                value *= 7; // subject number
                value %= 20201227;
                loop_size += 1;
            }
            loop_size
        }

        let card_loop_size = loop_size(card_pub_key);
        let door_loop_size = loop_size(door_pub_key);

        fn encryption_key(subject_number: usize, loop_size: usize) -> usize {
            let mut value = 1;
            for _ in 0..loop_size {
                value *= subject_number;
                value %= 20201227;
            }
            value
        }

        let handshake = encryption_key(card_pub_key, door_loop_size);
        assert_eq!(handshake, encryption_key(door_pub_key, card_loop_size));

        handshake
    }

    fn solve_part2(&self, _input: &Self::Input) -> Self::Output2 {
        "Merry Christmas!".to_string()
    }

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        let mut pub_keys = BufReader::new(r)
            .lines()
            .flatten()
            .take(2)
            .map(|pub_key| pub_key.parse().unwrap());

        (pub_keys.next().unwrap(), pub_keys.next().unwrap())
    }
}
