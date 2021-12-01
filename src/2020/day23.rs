//! --- Day 23: Crab Cups ---

use crate::solver::Solver;
use std::{
    io::{self, BufRead, BufReader},
    iter::{once, FromIterator},
};

/// https://adventofcode.com/2020/day/23
pub struct Day23;

#[derive(Clone, Debug)]
pub struct Cups<T> {
    label: Vec<T>,
    count: usize,
    prev: Vec<usize>,
    next: Vec<usize>,
}

impl Solver for Day23 {
    type Input = Cups<u8>;
    type Output1 = String;
    type Output2 = usize;

    fn solve_part1(&self, input: &Self::Input) -> Self::Output1 {
        let mut cups = input.clone();
        let mut curr = 0; // index of the current cup

        for _ in 1..=100 {
            let (pick_ups, pick_up_labels, next) = {
                let fst = cups.next[curr];
                let snd = cups.next[fst];
                let trd = cups.next[snd];
                (
                    (fst, snd, trd),
                    [cups.label[fst], cups.label[snd], cups.label[trd]],
                    cups.next[trd],
                )
            };

            // Remove the three picked up cups from the circle.
            cups.link(curr, next);

            // Select the destination cup.
            let curr_label = cups.label[curr];
            let mut dest_label = match curr_label - 1 {
                0 => cups.count as u8, // highest label
                l => l,
            };
            while pick_up_labels.contains(&dest_label) {
                dest_label = match dest_label - 1 {
                    0 => cups.count as u8,
                    l => l,
                };
            }
            let dest = cups.find(dest_label).unwrap();

            // Places the pick ups immediately clockwise of the destination cup.
            cups.link(pick_ups.2, cups.next[dest]);
            cups.link(dest, pick_ups.0);

            // Select the new current cup.
            curr = next;
        }

        // Starting after the cup labeled 1, collect the other cups' labels clockwise.
        cups.clockwise_from_label(1)
            .into_iter()
            .skip(1)
            .map(|label| label.to_string())
            .collect::<Vec<_>>()
            .join("")
    }

    fn solve_part2(&self, input: &Self::Input) -> Self::Output2 {
        let mut cups = Cups::from_labels_up_to(&input.label, 1_000_000);
        let mut curr = 0; // index of the current cup

        for _ in 1..=10_000_000 {
            let (pick_ups, pick_up_labels, next) = {
                let fst = cups.next[curr];
                let snd = cups.next[fst];
                let trd = cups.next[snd];
                (
                    (fst, snd, trd),
                    [cups.label[fst], cups.label[snd], cups.label[trd]],
                    cups.next[trd],
                )
            };

            // Remove the three picked up cups from the circle.
            cups.link(curr, next);

            // Select the destination cup.
            let curr_label = cups.label[curr];
            let mut dest_label = match curr_label - 1 {
                0 => cups.count, // highest label
                l => l,
            };
            while pick_up_labels.contains(&dest_label) {
                dest_label = match dest_label - 1 {
                    0 => cups.count,
                    l => l,
                };
            }
            let dest = if dest_label <= 10 {
                cups.find(dest_label).unwrap()
            } else {
                dest_label - 1
            };

            // Places the pick ups immediately clockwise of the destination cup.
            cups.link(pick_ups.2, cups.next[dest]);
            cups.link(dest, pick_ups.0);

            // Select the new current cup.
            curr = next;
        }

        // Determine which two cups will end up immediately clockwise of cup 1.
        // What do you get if you multiply their labels together?
        cups.clockwise_from_label(1)
            .into_iter()
            .skip(1)
            .take(2)
            .product()
    }

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        BufReader::new(r)
            .lines()
            .flatten()
            .next()
            .unwrap()
            .chars()
            .map(|cup| cup as u8 - b'0')
            .collect()
    }
}

impl<T: Copy + Eq> Cups<T> {
    /// Places `this` before `that` (and so, `that` after `this`).
    fn link(&mut self, this: usize, that: usize) {
        self.next[this] = that;
        self.prev[that] = this;
    }

    /// Returns the index of the cup labeled with `value`.
    fn find(&self, value: T) -> Option<usize> {
        self.label.iter().enumerate().find_map(
            |(i, &label)| {
                if label == value {
                    Some(i)
                } else {
                    None
                }
            },
        )
    }

    /// Returns the cup labels in clockwise order, starting with
    /// the cup labeled `value`.
    fn clockwise_from_label(&self, value: T) -> Vec<T> {
        let mut labels = Vec::with_capacity(self.count - 1);
        let mut cup = self.find(value).unwrap();
        let first = cup;
        loop {
            labels.push(self.label[cup]);
            cup = self.next[cup];
            if cup == first {
                break;
            }
        }

        labels
    }
}

impl<T> FromIterator<T> for Cups<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let label = iter.into_iter().collect::<Vec<_>>();
        let count = label.len();

        Cups {
            label,
            count,
            prev: once(count - 1).chain(0..count - 1).collect(),
            next: (1..count).chain(once(0)).collect(),
        }
    }
}

impl Cups<usize> {
    fn from_labels_up_to<T>(labels: &[T], count: usize) -> Self
    where
        T: Copy + Ord + Into<usize>,
    {
        let mut label = Vec::with_capacity(count);

        for value in labels {
            label.push((*value).into());
        }

        let highest_label: usize = (*labels.iter().max().unwrap()).into();
        for value in (highest_label + 1)..=count {
            label.push(value);
        }

        Cups {
            label,
            count,
            prev: once(count - 1).chain(0..count - 1).collect(),
            next: (1..count).chain(once(0)).collect(),
        }
    }
}
