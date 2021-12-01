//! --- Day 17: Conway Cubes ---

use crate::solver::Solver;
use std::{
    collections::{HashMap, HashSet},
    io::{self, BufRead, BufReader},
};

/// https://adventofcode.com/2020/day/17
pub struct Day17;

// @Todo: rewrite this with const generics... no time to make it pretty right now.

struct Pocket {
    // Represent the set of active cubes in each z-plane by their (x, y) coordinates.
    // @Note: since there's z-axis symmetry in the problem, we can store just the
    // planes at positive z coordinates, and index into `z_planes` using `.abs()`.
    z_planes: HashMap<i32, HashSet<(i32, i32)>>,
}

impl Pocket {
    fn new(starting_region: HashSet<(i32, i32)>) -> Self {
        let mut z_planes = HashMap::new();
        z_planes.insert(0, starting_region);
        Pocket { z_planes }
    }

    fn active_count(&self) -> usize {
        self.z_planes.values().map(HashSet::len).sum::<usize>()
    }

    /// Execute one cycle, where all cubes simultaneously change their state according
    /// to the following rules:
    /// - If a cube is active and exactly 2 or 3 of its neighbors are also active,
    ///   the cube remains active. Otherwise, the cube becomes inactive.
    /// - If a cube is inactive but exactly 3 of its neighbors are active,
    ///   the cube becomes active. Otherwise, the cube remains inactive.
    fn cycle(&mut self) {
        let mut active_neighbors: HashMap<i32, HashMap<(i32, i32), usize>> = HashMap::new();

        for (z, plane) in self.z_planes.iter() {
            for (x, y) in plane {
                for (nx, ny, nz) in neighbors((*x, *y, *z)) {
                    *active_neighbors
                        .entry(nz)
                        .or_insert_with(HashMap::new)
                        .entry((nx, ny))
                        .or_insert(0) += 1;
                }
            }
        }

        let mut new_z_planes: HashMap<i32, HashSet<(i32, i32)>> = HashMap::new();

        for (&z, neighbors) in active_neighbors.iter() {
            for (&xy, &count) in neighbors {
                match self.z_planes.get(&z) {
                    Some(plane) => match plane.contains(&xy) {
                        true => {
                            if (2..=3).contains(&count) {
                                new_z_planes
                                    .entry(z)
                                    .or_insert_with(HashSet::new)
                                    .insert(xy);
                            }
                        }
                        false => {
                            if count == 3 {
                                new_z_planes
                                    .entry(z)
                                    .or_insert_with(HashSet::new)
                                    .insert(xy);
                            }
                        }
                    },
                    None => {
                        // All cubes in this plane were inactive.
                        if count == 3 {
                            new_z_planes
                                .entry(z)
                                .or_insert_with(HashSet::new)
                                .insert(xy);
                        }
                    }
                }
            }
        }

        self.z_planes = new_z_planes;
    }
}

impl Solver for Day17 {
    type Input = HashSet<(i32, i32)>;
    type Output1 = usize;
    type Output2 = usize;

    fn solve_part1(&self, input: &Self::Input) -> Self::Output1 {
        let mut pocket = Pocket::new(input.clone());

        for _ in 1..=6 {
            pocket.cycle();
        }

        pocket.active_count()
    }

    fn solve_part2(&self, input: &Self::Input) -> Self::Output2 {
        // @Cleanup: no time to refactor, just inline everything... aaaagh!
        let mut w_planes = HashMap::new();

        w_planes.insert(
            0,
            input
                .iter()
                .map(|&(x, y)| (x, y, 0))
                .collect::<HashSet<(i32, i32, i32)>>(),
        );

        for _ in 1..=6 {
            let mut active_neighbors = HashMap::new();

            for (w, plane) in w_planes.iter() {
                for (x, y, z) in plane {
                    for (nx, ny, nz, nw) in neighbors4d((*x, *y, *z, *w)) {
                        *active_neighbors
                            .entry(nw)
                            .or_insert_with(HashMap::new)
                            .entry((nx, ny, nz))
                            .or_insert(0) += 1;
                    }
                }
            }

            let mut new_w_planes = HashMap::new();

            for (&w, neighbors) in active_neighbors.iter() {
                for (&xyz, &count) in neighbors {
                    match w_planes.get(&w) {
                        Some(plane) => match plane.contains(&xyz) {
                            true => {
                                if (2..=3).contains(&count) {
                                    new_w_planes
                                        .entry(w)
                                        .or_insert_with(HashSet::new)
                                        .insert(xyz);
                                }
                            }
                            false => {
                                if count == 3 {
                                    new_w_planes
                                        .entry(w)
                                        .or_insert_with(HashSet::new)
                                        .insert(xyz);
                                }
                            }
                        },
                        None => {
                            // All cubes in this plane were inactive.
                            if count == 3 {
                                new_w_planes
                                    .entry(w)
                                    .or_insert_with(HashSet::new)
                                    .insert(xyz);
                            }
                        }
                    }
                }
            }

            w_planes = new_w_planes;
        }

        w_planes.values().map(HashSet::len).sum::<usize>()
    }

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        BufReader::new(r)
            .lines()
            .flatten()
            .enumerate()
            .flat_map(|(y, line)| {
                line.char_indices()
                    .filter_map(|(x, cube)| match cube {
                        '#' => Some((x as i32, y as i32)), // active
                        '.' => None,                       // inactive
                        _ => unreachable!(),
                    })
                    .collect::<HashSet<_>>()
            })
            .collect()
    }
}

type Xyz = (i32, i32, i32);
type Xyzw = (i32, i32, i32, i32);

#[rustfmt::skip]
fn neighbors((x, y, z): Xyz) -> Vec<Xyz> {
    [
        (-1, -1,  1), (0, -1,  1), (1, -1,  1),
        (-1,  0,  1), (0,  0,  1), (1,  0,  1),
        (-1,  1,  1), (0,  1,  1), (1,  1,  1),

        (-1, -1,  0), (0, -1,  0), (1, -1,  0),
        (-1,  0,  0),              (1,  0,  0),
        (-1,  1,  0), (0,  1,  0), (1,  1,  0),

        (-1, -1, -1), (0, -1, -1), (1, -1, -1),
        (-1,  0, -1), (0,  0, -1), (1,  0, -1),
        (-1,  1, -1), (0,  1, -1), (1,  1, -1),
    ]
    .iter()
    .map(|(dx, dy, dz)| (x + dx, y + dy, z + dz))
    .collect()
}

fn neighbors4d((x, y, z, w): Xyzw) -> Vec<Xyzw> {
    let mut neighbors = Vec::with_capacity(80);
    for dx in -1..=1 {
        for dy in -1..=1 {
            for dz in -1..=1 {
                for dw in -1..=1 {
                    if !(dx == 0 && dy == 0 && dz == 0 && dw == 0) {
                        neighbors.push((x + dx, y + dy, z + dz, w + dw));
                    }
                }
            }
        }
    }
    neighbors
}
