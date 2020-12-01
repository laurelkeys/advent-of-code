// Ref.: https://github.com/noirotm/advent-of-code-2019/blob/master/src/solver.rs

use std::{fmt::Debug, fs::File, io, path::Path};

fn input_file(day: i32) -> String {
    format!("input/day{:02}.txt", day)
}

pub trait Solver {
    type Input;
    type Output1: Debug;
    type Output2: Debug;

    fn solve_1st(&self, input: &Self::Input) -> Self::Output1;
    fn solve_2nd(&self, input: &Self::Input) -> Self::Output2;
    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input;

    fn load_input<P: AsRef<Path>>(&self, p: P) -> io::Result<Self::Input> {
        let f = File::open(p)?;

        Ok(self.parse_input(f))
    }

    fn solve(&self, day: i32) {
        let input = self
            .load_input(input_file(day))
            .expect("unable to open input file");

        println!("Solution 1: {:?}", self.solve_1st(&input));
        println!("Solution 2: {:?}", self.solve_2nd(&input));
    }
}
