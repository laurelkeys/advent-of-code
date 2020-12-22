use std::{env, fs::read_dir, io, path::Path};

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod solver;

use solver::Solver;

fn main() {
    let day: u8 = env::args()
        .nth(1)
        .and_then(|day| day.parse().ok())
        .or_else(|| latest_day().ok())
        .expect("failed to parse day");

    match day {
        1 => day01::Day01 {}.solve(day),  // 691771, 232508760
        2 => day02::Day02 {}.solve(day),  // 546, 275
        3 => day03::Day03 {}.solve(day),  // 209, 1574890240
        4 => day04::Day04 {}.solve(day),  // 260, 153
        5 => day05::Day05 {}.solve(day),  // 998, 676
        6 => day06::Day06 {}.solve(day),  // 6249, 3103
        7 => day07::Day07 {}.solve(day),  // 185, 89084
        8 => day08::Day08 {}.solve(day),  // 1489, 1539
        9 => day09::Day09 {}.solve(day),  // 23278925, 4011064
        10 => day10::Day10 {}.solve(day), // 1917, 113387824750592
        11 => day11::Day11 {}.solve(day), // 2273, 2064
        12 => day12::Day12 {}.solve(day), // 508, 30761
        13 => day13::Day13 {}.solve(day), // 3215, 1001569619313439
        14 => day14::Day14 {}.solve(day), // 12408060320841, 4466434626828
        15 => day15::Day15 {}.solve(day), // 763, 1876406
        16 => day16::Day16 {}.solve(day), // 19060, 953713095011
        17 => day17::Day17 {}.solve(day), // 223, 1884
        18 => day18::Day18 {}.solve(day), // 650217205854, 20394514442037
        19 => day19::Day19 {}.solve(day), // 136, 256
        20 => day20::Day20 {}.solve(day), // 84116744709593, 1957
        21 => day21::Day21 {}.solve(day), // 1930, "spcqmzfg,rpf,dzqlq,pflk,bltrbvz,xbdh,spql,bltzkxx"
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
