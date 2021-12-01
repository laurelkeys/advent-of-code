// Ref.: https://github.com/noirotm/advent-of-code-2019/blob/master/src/solver.rs

use std::{fmt::Display, fs::File, io, path::Path};

pub enum SolverYear {
    Aoc2020,
    Aoc2021,
}

pub trait Solver {
    type Input;
    type Output1: Display;
    type Output2: Display;

    fn solve_part1(&self, input: &Self::Input) -> Self::Output1;
    fn solve_part2(&self, input: &Self::Input) -> Self::Output2;
    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input;

    fn load_input<P: AsRef<Path>>(&self, p: P) -> io::Result<Self::Input> {
        let f = File::open(p)?;

        Ok(self.parse_input(f))
    }

    fn solve(&self, year: SolverYear, day: u8) {
        let input_file_path = match year {
            SolverYear::Aoc2020 => format!("input/2020/day{:02}.txt", day),
            SolverYear::Aoc2021 => format!("input/2021/day{:02}.txt", day),
        };

        let input = self
            .load_input(input_file_path)
            .expect("unable to open input file");

        println!("[Day {}] Answer 1: {}", day, self.solve_part1(&input));
        println!("[Day {}] Answer 2: {}", day, self.solve_part2(&input));
    }
}
