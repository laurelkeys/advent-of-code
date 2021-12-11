//! --- Day 8: Seven Segment Search ---

use crate::solver::Solver;
use std::io::{self, BufRead, BufReader};

/// https://adventofcode.com/2021/day/8
pub struct Day08;

const ABCDEFG: [char; 7] = ['a', 'b', 'c', 'd', 'e', 'f', 'g'];

impl Solver for Day08 {
    type Input = Vec<(Vec<String>, Vec<String>)>;
    type Output1 = usize;
    type Output2 = usize;

    /// Each entry consists of ten unique signal patterns, a `|` delimiter, and finally
    /// the four digit output value. Within an entry, the same wire/segment connections
    /// are used (but you don't know what the connections actually are).
    ///
    /// Because the digits 1, 4, 7, and 8 each use a unique number of segments, you should
    /// be able to tell which combinations of signals correspond to those digits.
    ///
    /// In the output values, how many times do digits 1, 4, 7, or 8 appear?
    fn solve_part1(&self, input: &Self::Input) -> Self::Output1 {
        input.iter().fold(0, |acc, (_, output)| {
            acc + output
                .iter()
                .filter(|value| [2, 3, 4, 7_usize].contains(&value.len()))
                .count()
        })
    }

    /// For each entry, determine all of the wire/segment connections and decode the
    /// four-digit output values. What do you get if you add up all of the output values?
    fn solve_part2(&self, input: &Self::Input) -> Self::Output2 {
        fn decode_output(patterns: &[String], output: &[String]) -> usize {
            // In the ten unique signal patterns, the frequency of each letter in "abcdefg"
            // is [8, 6, 8, 7, 4, 9, 7]. Since 6, 4 and 9 are unique, we can find out "bef":
            let freq = patterns.iter().fold([0; 7], |mut freq, pattern| {
                pattern.chars().for_each(|chr| {
                    freq[(chr as u8 - b'a') as usize] += 1;
                });
                freq
            });

            let b = ABCDEFG[freq.iter().position(|&f| f == 6).unwrap()] as u8;
            let e = ABCDEFG[freq.iter().position(|&f| f == 4).unwrap()] as u8;
            let f = ABCDEFG[freq.iter().position(|&f| f == 9).unwrap()] as u8;

            // Digits 1, 4, 7, and 8 each use a unique number of segments:
            let pattern_1 = patterns.iter().find(|p| p.len() == 2).unwrap().as_bytes(); // cf
            let pattern_4 = patterns.iter().find(|p| p.len() == 4).unwrap().as_bytes(); // bcdf
            let pattern_7 = patterns.iter().find(|p| p.len() == 3).unwrap().as_bytes(); // acf
            let pattern_8 = patterns.iter().find(|p| p.len() == 7).unwrap().as_bytes(); // abcdefg

            #[rustfmt::skip]
            let (a, c, d, g) = {
                let a = *pattern_7.iter().find(|&chr| !pattern_1.contains(chr)).unwrap();
                let c = *pattern_7.iter().find(|&chr| ![a, f].contains(chr)).unwrap();
                let d = *pattern_4.iter().find(|&chr| ![b, c, f].contains(chr)).unwrap();
                let g = *pattern_8.iter().find(|&chr| ![a, b, c, d, e, f].contains(chr)).unwrap();
                (a, c, d, g)
            };

            let decode = {
                let mut decode = [' '; 7];
                decode[(a - b'a') as usize] = 'a';
                decode[(b - b'a') as usize] = 'b';
                decode[(c - b'a') as usize] = 'c';
                decode[(d - b'a') as usize] = 'd';
                decode[(e - b'a') as usize] = 'e';
                decode[(f - b'a') as usize] = 'f';
                decode[(g - b'a') as usize] = 'g';
                decode
            };

            output
                .iter()
                .enumerate()
                .map(|(i, encoded)| {
                    let decoded = {
                        let mut decoded = encoded
                            .chars()
                            .map(|chr| decode[(chr as u8 - b'a') as usize])
                            .collect::<Vec<_>>();
                        decoded.sort_unstable();
                        decoded.into_iter().collect::<String>()
                    };

                    let value = match &decoded[..] {
                        "abcefg" => 0,
                        "cf" => 1,
                        "acdeg" => 2,
                        "acdfg" => 3,
                        "bcdf" => 4,
                        "abdfg" => 5,
                        "abdefg" => 6,
                        "acf" => 7,
                        "abcdefg" => 8,
                        "abcdfg" => 9,
                        _ => unreachable!(),
                    };

                    value * 10_usize.pow(3 - i as u32)
                })
                .sum()
        }

        input
            .iter()
            .map(|(patterns, output)| decode_output(patterns, output))
            .sum()
    }

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        BufReader::new(r)
            .lines()
            .flatten()
            .map(|line| {
                let (patterns, output) = line.split_once(" | ").unwrap();

                let patterns = patterns.split(' ').map(String::from).collect::<Vec<_>>();
                assert_eq!(patterns.len(), 10); // all ten unique signal patters

                let output = output.split(' ').map(String::from).collect::<Vec<_>>();
                assert_eq!(output.len(), 4); // four digit output value

                (patterns, output)
            })
            .collect::<Vec<(Vec<String>, Vec<String>)>>()
    }
}

/*
struct SevenSegmentDisplay(u8);

const NUMBER: [SevenSegmentDisplay; 10] = [
    //                    abcdefg
    SevenSegmentDisplay(0b1110111), // 0: abcefg
    SevenSegmentDisplay(0b0010010), // 1: cf        (only digit that uses 2 segments)
    SevenSegmentDisplay(0b1011101), // 2: acdeg
    SevenSegmentDisplay(0b1011011), // 3: acdfg
    SevenSegmentDisplay(0b0111010), // 4: bcdf      (only digit that uses 4 segments)
    SevenSegmentDisplay(0b1101011), // 5: abdfg
    SevenSegmentDisplay(0b1101111), // 6: abdefg
    SevenSegmentDisplay(0b1010010), // 7: acf       (only digit that uses 3 segments)
    SevenSegmentDisplay(0b1111111), // 8: abcdefg   (only digit that uses 7 segments)
    SevenSegmentDisplay(0b1111011), // 9: abcdfg
];

impl SevenSegmentDisplay {
    fn new(a: bool, b: bool, c: bool, d: bool, e: bool, f: bool, g: bool) -> Self {
        SevenSegmentDisplay(
            ((a as u8) << 6)
                & ((b as u8) << 5)
                & ((c as u8) << 4)
                & ((d as u8) << 3)
                & ((e as u8) << 2)
                & ((f as u8) << 1)
                & (g as u8),
        )
    }
}

use std::fmt::Display;

impl Display for SevenSegmentDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let is_on = |bit: u8| (self.0 >> bit) & 1 == 1;
        f.write_str(if is_on(6) { " aaaa \n" } else { " .... \n" })?;
        f.write_str(match (is_on(5), is_on(4)) {
            (true, true) => "b    c\nb    c\n",
            (true, false) => "b    .\nb    .\n",
            (false, true) => ".    c\n.    c\n",
            (false, false) => ".    .\n.    .\n",
        })?;
        f.write_str(if is_on(3) { " dddd \n" } else { " .... \n" })?;
        f.write_str(match (is_on(2), is_on(1)) {
            (true, true) => "e    f\ne    f\n",
            (true, false) => "e    .\ne    .\n",
            (false, true) => ".    f\n.    f\n",
            (false, false) => ".    .\n.    .\n",
        })?;
        f.write_str(if is_on(0) { " gggg \n" } else { " .... \n" })?;
        Ok(())
    }
}
*/
