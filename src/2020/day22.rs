//! --- Day 22: Crab Combat ---

use crate::solver::Solver;
use std::{
    collections::{hash_map::DefaultHasher, HashSet, VecDeque},
    hash::{Hash, Hasher},
    io,
};

/// https://adventofcode.com/2020/day/22
pub struct Day22;

fn hash(player1: &VecDeque<u8>, player2: &VecDeque<u8>) -> u64 {
    let mut hasher = DefaultHasher::new();
    (player1, player2).hash(&mut hasher);
    hasher.finish()
}

impl Solver for Day22 {
    type Input = (VecDeque<u8>, VecDeque<u8>);
    type Output1 = usize;
    type Output2 = usize;

    fn solve_part1(&self, input: &Self::Input) -> Self::Output1 {
        let (mut player1, mut player2) = input.clone();
        let card_count = player1.len() + player2.len();

        while !(player1.is_empty() || player2.is_empty()) {
            let card1 = player1.pop_front().unwrap();
            let card2 = player2.pop_front().unwrap();

            if card1 > card2 {
                player1.push_back(card1);
                player1.push_back(card2);
            } else {
                player2.push_back(card2);
                player2.push_back(card1);
            }
        }

        if player1.is_empty() { player2 } else { player1 }
            .iter()
            .enumerate()
            .map(|(i, &card)| (card_count - i) * card as usize)
            .sum()
    }

    fn solve_part2(&self, input: &Self::Input) -> Self::Output2 {
        let (mut player1, mut player2) = input.clone();
        let card_count = player1.len() + player2.len();

        #[rustfmt::skip]
        fn recursive_combat_winner(player1: &mut VecDeque<u8>, player2: &mut VecDeque<u8>) -> u8 {
            let mut rounds = HashSet::new();

            while !(player1.is_empty() || player2.is_empty()) {
                if !rounds.insert(hash(player1, player2)) {
                    return 1; // this exact round had already been played
                }

                let card1 = player1.pop_front().unwrap();
                let card2 = player2.pop_front().unwrap();

                let winner = if player1.len() < card1 as usize
                             || player2.len() < card2 as usize
                {
                    if card1 > card2 { 1 } else { 2 }
                } else {
                    recursive_combat_winner(
                        &mut player1.iter().take(card1 as usize).cloned().collect(),
                        &mut player2.iter().take(card2 as usize).cloned().collect(),
                    )
                };

                if winner == 1 {
                    player1.push_back(card1);
                    player1.push_back(card2);
                } else {
                    player2.push_back(card2);
                    player2.push_back(card1);
                }
            }

            if player1.is_empty() { 2 } else { 1 }
        }

        let winner = recursive_combat_winner(&mut player1, &mut player2);

        if winner == 1 { player1 } else { player2 }
            .iter()
            .enumerate()
            .map(|(i, &card)| (card_count - i) * card as usize)
            .sum()
    }

    fn parse_input<R: io::Read>(&self, mut r: R) -> Self::Input {
        let mut input = String::new();
        r.read_to_string(&mut input).unwrap();
        let mut input = input.trim_end().split("\n\n");

        let mut parse_player = || {
            input
                .next()
                .unwrap()
                .lines()
                .skip(1)
                .map(|card| card.parse::<u8>().unwrap())
                .collect::<VecDeque<_>>()
        };

        (parse_player(), parse_player())
    }
}
