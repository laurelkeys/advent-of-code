use std::{env, fs::read_dir, io, path::Path};

mod day01;
mod day02;
mod day03;
mod day04;
mod solver;

use solver::Solver;

fn main() {
    let day: u8 = env::args()
        .nth(1)
        .and_then(|day| day.parse().ok())
        .or_else(|| latest_day().ok())
        .expect("failed to parse day");

    match day {
        1 => day01::Day01 {}.solve(day), // 691771, 232508760
        2 => day02::Day02 {}.solve(day), // 546, 275
        3 => day03::Day03 {}.solve(day), // 209, 1574890240
        4 => day04::Day04 {}.solve(day), // 260, 153
        _ => eprintln!("Day {} hasn't been solved yet 😅", day),
    }
}

fn latest_day() -> io::Result<u8> {
    fn parse_day(input_file_name: &str) -> Option<u8> {
        input_file_name[3..5].parse().ok() // e.g. maps "day25.txt" to 25
    }

    let days = read_dir(Path::new(".").join("input"))?
        .flatten()
        .filter_map(|entry| {
            entry
                .path()
                .file_name()
                .and_then(|file_name| file_name.to_str())
                .and_then(|file_name| parse_day(file_name))
        })
        .collect::<Vec<_>>();

    Ok(*days.iter().max().unwrap())
}
