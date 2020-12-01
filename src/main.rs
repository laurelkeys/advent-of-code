use std::env;

mod day01;
mod solver;

use solver::Solver;

// @Todo: automatically default to the latest day in input/.
const DEFAULT_DAY: i32 = 1;

fn main() {
    let day = env::args()
        .nth(1)
        .unwrap_or_else(|| DEFAULT_DAY.to_string())
        .parse()
        .unwrap_or(DEFAULT_DAY);

    match day {
        1 => day01::Day01 {}.solve(day),
        _ => eprintln!("Day {} hasn't been solved yet :(", day),
    }
}
