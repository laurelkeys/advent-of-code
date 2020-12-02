use std::{env, fs::read_dir, path::Path};

mod day01;
mod day02;
mod solver;

use solver::Solver;

fn main() {
    let default_day = latest_day();

    let day = env::args()
        .nth(1)
        .unwrap_or_else(|| default_day.to_string())
        .parse()
        .unwrap_or(default_day);

    println!("Solutions for day {}:", day);

    match day {
        1 => day01::Day01 {}.solve(day), // 691771, 232508760
        2 => day02::Day02 {}.solve(day), // 546, 275
        _ => eprintln!("Day {} hasn't been solved yet :(", day),
    }
}

fn latest_day() -> i32 {
    let mut days = read_dir(Path::new(".").join("input"))
        .unwrap()
        .flatten()
        .filter(|entry| entry.path().is_file())
        .flat_map(|file| file.file_name().into_string())
        .flat_map(|file_name| file_name[3..5].parse::<i32>())
        .collect::<Vec<_>>();
    days.sort_unstable();
    *days.last().unwrap()
}
