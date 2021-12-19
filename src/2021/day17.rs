//! --- Day 17: Trick Shot ---

use crate::solver::Solver;
use std::io::{self, BufRead, BufReader};

/// https://adventofcode.com/2021/day/17
pub struct Day17;

/// The probe launcher on your submarine can fire the probe with any integer
/// velocity in the x (forward) and y (upward, or downward if negative) directions.
///
/// The probe's x,y position starts at 0,0. Then, it will follow some trajectory by
/// moving in steps. On each step, these changes occur in the following order:
/// - The probe's x position increases by its x velocity.
/// - The probe's y position increases by its y velocity.
/// - Due to drag, the probe's x velocity changes by 1 toward the value 0; that is,
///   it decreases by 1 if it is greater than 0, increases by 1 if it is less than
///   0, or does not change if it is already 0.
/// - Due to gravity, the probe's y velocity decreases by 1.
///
/// For the probe to successfully make it into the trench, the probe must be on some
/// trajectory that causes it to be within a target area after any step.
fn max_y_after_any_step(
    (x0, xf): (i32, i32),
    (y0, yf): (i32, i32),
    mut vel: (i32, i32),
) -> Option<i32> {
    let mut pos = (0, 0);
    let mut max_y = 0;

    while pos.1 >= y0 {
        if (x0..=xf).contains(&pos.0) && (y0..=yf).contains(&pos.1) {
            return Some(max_y);
        }

        pos.0 += vel.0;
        pos.1 += vel.1;

        max_y = max_y.max(pos.1);

        vel.0 -= vel.0.signum();
        vel.1 -= 1;
    }

    None
}

impl Solver for Day17 {
    type Input = ((i32, i32), (i32, i32));
    type Output1 = i32;
    type Output2 = usize;

    /// Find the initial velocity that causes the probe to reach the highest y position
    /// and still eventually be within the target area after any step.
    /// What is the highest y position it reaches on this trajectory?
    fn solve_part1(&self, input: &Self::Input) -> Self::Output1 {
        let ((x0, xf), (y0, yf)) = *input;
        let max_y_abs = Ord::max(y0.abs(), yf.abs());

        ((0..).find(|vx| (vx * vx + vx) / 2 >= x0).unwrap()..=xf)
            .flat_map(|vx| (-max_y_abs..=max_y_abs).map(move |vy| (vx, vy)))
            .filter_map(|vel| max_y_after_any_step((x0, xf), (y0, yf), vel))
            .max_by_key(|y| *y)
            .unwrap()
    }

    /// How many distinct initial velocity values cause the probe to be within the
    /// target area after any step?
    fn solve_part2(&self, input: &Self::Input) -> Self::Output2 {
        let ((x0, xf), (y0, yf)) = *input;
        let max_y_abs = Ord::max(y0.abs(), yf.abs());

        ((0..).find(|vx| (vx * vx + vx) / 2 >= x0).unwrap()..=xf)
            .flat_map(|vx| (-max_y_abs..=max_y_abs).map(move |vy| (vx, vy)))
            .filter_map(|vel| max_y_after_any_step((x0, xf), (y0, yf), vel))
            .count()
    }

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        let input = BufReader::new(r).lines().flatten().next().unwrap();
        let (xs, ys) = input
            .strip_prefix("target area: x=")
            .and_then(|input| input.split_once(", y="))
            .unwrap();

        fn zip_ok<T, E>(lhs: Result<T, E>, rhs: Result<T, E>) -> Option<(T, T)> {
            lhs.ok().zip(rhs.ok())
        }

        (
            xs.split_once("..")
                .and_then(|(x0, xf)| zip_ok(x0.parse(), xf.parse()))
                .unwrap(),
            ys.split_once("..")
                .and_then(|(y0, yf)| zip_ok(y0.parse(), yf.parse()))
                .unwrap(),
        )
    }
}
