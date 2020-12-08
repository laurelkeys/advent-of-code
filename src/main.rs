use std::{env, fs::read_dir, io, path::Path};

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod solver;

use solver::Solver;

fn main() {
    if let Some(test_file_path) = env::args().nth(2) {
        println!(" === TESTING === ");

        let solver07 = day07::Day07 {};

        let input = solver07
            .load_input(test_file_path)
            .expect("unable to open test file");

        println!("[Day 7] Answer 1: {}", solver07.solve_part1(&input));
        println!("[Day 7] Answer 2: {}", solver07.solve_part2(&input));
        println!(" === TESTING === ");
        return;
    }

    // @Fixme: remove before commit ^^^^^^^^

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
        5 => day05::Day05 {}.solve(day), // 998, 676
        6 => day06::Day06 {}.solve(day), // 6249, 3103
        7 => day07::Day07 {}.solve(day), // 185, 89084
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
