//! --- Day 20: Monster Messages ---

use crate::solver::Solver;
use std::{collections::HashMap, io, str::FromStr};

/// https://adventofcode.com/2020/day/20
pub struct Day20;

#[derive(Debug, PartialEq, Eq)]
pub struct Tile {
    image: [[char; 10]; 10],
}

impl Solver for Day20 {
    type Input = HashMap<usize, Tile>;
    type Output1 = usize;
    type Output2 = usize;

    fn solve_part1(&self, input: &Self::Input) -> Self::Output1 {
        let mut tile_matches = HashMap::new();

        for (id, tile) in input.iter() {
            // @Cleanup: ignore already matched margins.
            let margins = tile.margins();

            'matching: for (other_id, other_tile) in input.iter() {
                if other_id == id {
                    continue 'matching;
                }

                for margin in margins.iter() {
                    for other_margin in other_tile.margins().iter() {
                        if *margin == *other_margin {
                            tile_matches
                                .entry(id)
                                .or_insert_with(Vec::new)
                                .push(other_id);
                            continue 'matching;
                        }
                    }
                }
            }
        }

        // What do you get if you multiply together the IDs of the four corner tiles?
        tile_matches
            .iter()
            .filter_map(|(&&id, matches)| {
                match matches.len() {
                    2 => Some(id), // corner
                    _ => None,
                }
            })
            .product()
    }

    fn solve_part2(&self, input: &Self::Input) -> Self::Output2 {
        todo!()
    }

    fn parse_input<R: io::Read>(&self, mut r: R) -> Self::Input {
        let mut input = String::new();
        r.read_to_string(&mut input).unwrap();
        let input = input.trim_end().split("\n\n");

        input
            .map(|image_tile| {
                let mut image_tile = image_tile.splitn(2, '\n');
                let id = image_tile
                    .next()
                    .unwrap()
                    .trim_start_matches("Tile ")
                    .trim_end_matches(':')
                    .parse()
                    .unwrap();
                let tile = image_tile.next().unwrap().parse().unwrap();

                (id, tile)
            })
            .collect()
    }
}

impl FromStr for Tile {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut image = [[' '; 10]; 10];
        for (i, row) in s.lines().enumerate() {
            for (j, c) in row.char_indices() {
                image[i][j] = c;
            }
        }

        if image
            .iter()
            .all(|row| row.iter().all(|&c| c == '.' || c == '#'))
        {
            Ok(Tile { image })
        } else {
            Err(())
        }
    }
}

// @Cleanup: replace this with a flip! macro
#[rustfmt::skip]
fn flip(mut margin: [char; 10]) -> [char; 10] {
    margin.swap(0, 9);
    margin.swap(1, 8);
    margin.swap(2, 7);
    margin.swap(3, 6);
    margin.swap(4, 5);
    margin
}

impl Tile {
    #[rustfmt::skip]
    fn margins(&self) -> [[char; 10]; 8] {
        [
            self.top(), self.bottom(), self.left(), self.right(),
            flip(self.top()), flip(self.bottom()), flip(self.left()), flip(self.right()),
        ]
    }

    fn top(&self) -> [char; 10] {
        self.image[0]
    }
    fn bottom(&self) -> [char; 10] {
        self.image[10 - 1]
    }

    // @Cleanup: create a row! macro
    #[rustfmt::skip]
    fn left(&self) -> [char; 10] {
        [
            self.image[0][0], self.image[1][0], self.image[2][0], self.image[3][0], self.image[4][0],
            self.image[5][0], self.image[6][0], self.image[7][0], self.image[8][0], self.image[9][0],
        ]
    }
    #[rustfmt::skip]
    fn right(&self) -> [char; 10] {
        [
            self.image[0][9], self.image[1][9], self.image[2][9], self.image[3][9], self.image[4][9],
            self.image[5][9], self.image[6][9], self.image[7][9], self.image[8][9], self.image[9][9],
        ]
    }
}
