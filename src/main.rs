use std::{env, ffi::OsStr, fs::read_dir, io, path::Path};

#[path = "./2020/mod.rs"]
mod aoc2020;

#[path = "./2021/mod.rs"]
mod aoc2021;

mod solver;
use solver::{
    Solver,
    SolverYear::{self, Aoc2020, Aoc2021},
};

const YEAR: SolverYear = Aoc2021;

fn main() {
    let day: u8 = env::args()
        .nth(1)
        .and_then(|day| day.parse().ok())
        .or_else(|| latest_day(YEAR).ok())
        .expect("failed to parse day");

    match YEAR {
        Aoc2020 => solve_aoc_2020(day),
        Aoc2021 => solve_aoc_2021(day),
    }
}

fn solve_aoc_2021(day: u8) {
    match day {
        1 => aoc2021::day01::Day01 {}.solve(Aoc2021, day), // 1532, 1571
        2 => aoc2021::day02::Day02 {}.solve(Aoc2021, day), // 1459206, 1320534480
        3 => aoc2021::day03::Day03 {}.solve(Aoc2021, day), // 2583164, 2784375
        4 => aoc2021::day04::Day04 {}.solve(Aoc2021, day), // 58412, 10030
        5 => aoc2021::day05::Day05 {}.solve(Aoc2021, day), // 5124, 19771
        6 => aoc2021::day06::Day06 {}.solve(Aoc2021, day), // 349549, 1589590444365
        7 => aoc2021::day07::Day07 {}.solve(Aoc2021, day), // 336120, 96864235
        8 => aoc2021::day08::Day08 {}.solve(Aoc2021, day), // 512, 1091165
        9 => aoc2021::day09::Day09 {}.solve(Aoc2021, day), // 439, 900900
        10 => aoc2021::day10::Day10 {}.solve(Aoc2021, day), // 392421, 2769449099
        11 => aoc2021::day11::Day11 {}.solve(Aoc2021, day), //
        _ => eprintln!("Day {} hasn't been solved yet 😅", day),
    }
}

fn solve_aoc_2020(day: u8) {
    match day {
        1 => aoc2020::day01::Day01 {}.solve(Aoc2020, day), // 691771, 232508760
        2 => aoc2020::day02::Day02 {}.solve(Aoc2020, day), // 546, 275
        3 => aoc2020::day03::Day03 {}.solve(Aoc2020, day), // 209, 1574890240
        4 => aoc2020::day04::Day04 {}.solve(Aoc2020, day), // 260, 153
        5 => aoc2020::day05::Day05 {}.solve(Aoc2020, day), // 998, 676
        6 => aoc2020::day06::Day06 {}.solve(Aoc2020, day), // 6249, 3103
        7 => aoc2020::day07::Day07 {}.solve(Aoc2020, day), // 185, 89084
        8 => aoc2020::day08::Day08 {}.solve(Aoc2020, day), // 1489, 1539
        9 => aoc2020::day09::Day09 {}.solve(Aoc2020, day), // 23278925, 4011064
        10 => aoc2020::day10::Day10 {}.solve(Aoc2020, day), // 1917, 113387824750592
        11 => aoc2020::day11::Day11 {}.solve(Aoc2020, day), // 2273, 2064
        12 => aoc2020::day12::Day12 {}.solve(Aoc2020, day), // 508, 30761
        13 => aoc2020::day13::Day13 {}.solve(Aoc2020, day), // 3215, 1001569619313439
        14 => aoc2020::day14::Day14 {}.solve(Aoc2020, day), // 12408060320841, 4466434626828
        15 => aoc2020::day15::Day15 {}.solve(Aoc2020, day), // 763, 1876406
        16 => aoc2020::day16::Day16 {}.solve(Aoc2020, day), // 19060, 953713095011
        17 => aoc2020::day17::Day17 {}.solve(Aoc2020, day), // 223, 1884
        18 => aoc2020::day18::Day18 {}.solve(Aoc2020, day), // 650217205854, 20394514442037
        19 => aoc2020::day19::Day19 {}.solve(Aoc2020, day), // 136, 256
        20 => aoc2020::day20::Day20 {}.solve(Aoc2020, day), // 84116744709593, 1957
        21 => aoc2020::day21::Day21 {}.solve(Aoc2020, day), // 1930, "spcqmzfg,rpf,dzqlq,pflk,bltrbvz,xbdh,spql,bltzkxx"
        22 => aoc2020::day22::Day22 {}.solve(Aoc2020, day), // 35370, 36246
        23 => aoc2020::day23::Day23 {}.solve(Aoc2020, day), // "45798623", 235551949822
        24 => aoc2020::day24::Day24 {}.solve(Aoc2020, day), // 495, 4012
        25 => aoc2020::day25::Day25 {}.solve(Aoc2020, day), // 4126980
        _ => eprintln!("That's all there is (no day {}).. see you next year!", day),
    }
}

fn latest_day(year: SolverYear) -> io::Result<u8> {
    fn parse_day(input_file_name: &str) -> Option<u8> {
        input_file_name[3..5].parse().ok() // e.g. maps "day25.txt" to 25
    }

    let path = match year {
        Aoc2020 => Path::new(".").join("input").join("2020"),
        Aoc2021 => Path::new(".").join("input").join("2021"),
    };

    let days = read_dir(path)?
        .flatten()
        .filter_map(|entry| {
            entry
                .path()
                .file_name()
                .and_then(OsStr::to_str)
                .and_then(parse_day)
        })
        .collect::<Vec<_>>();

    Ok(*days.iter().max().unwrap())
}
