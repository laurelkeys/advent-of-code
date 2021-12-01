//! --- Day 4: Passport Processing ---

use crate::solver::Solver;
use std::{
    io::{self, BufRead, BufReader},
    ops::RangeInclusive,
};

/// https://adventofcode.com/2020/day/4
pub struct Day04;

impl Solver for Day04 {
    type Input = Vec<String>;
    type Output1 = usize;
    type Output2 = usize;

    fn solve_part1(&self, input: &Self::Input) -> Self::Output1 {
        input
            .iter()
            .filter(|passport| has_all_fields(passport))
            .count()
    }

    fn solve_part2(&self, input: &Self::Input) -> Self::Output2 {
        input
            .iter()
            .filter(|passport| has_all_fields_valid(passport))
            .count()
    }

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        let (mut passport_list, passport) = BufReader::new(r).lines().flatten().fold(
            (vec![], vec![]),
            |(mut passport_list, mut passport), line| {
                if line.is_empty() {
                    // Different passports are separated by blank lines.
                    passport_list.push(passport.join(" "));
                    (passport_list, vec![])
                } else {
                    // Passport fields are separated by spaces or newlines.
                    passport.push(line);
                    (passport_list, passport)
                }
            },
        );
        passport_list.push(passport.join(" "));
        passport_list
    }
}

fn has_all_fields(passport: &str) -> bool {
    let mut byr_ok = false;
    let mut iyr_ok = false;
    let mut eyr_ok = false;
    let mut hgt_ok = false;
    let mut hcl_ok = false;
    let mut ecl_ok = false;
    let mut pid_ok = false;

    // Each passport is represented as a sequence of `key:value` pairs.
    for key_value in passport.split_ascii_whitespace() {
        let mut key_value = key_value.split(':');
        let key = key_value.next().unwrap();

        match key {
            "byr" => byr_ok = true,
            "iyr" => iyr_ok = true,
            "eyr" => eyr_ok = true,
            "hgt" => hgt_ok = true,
            "hcl" => hcl_ok = true,
            "ecl" => ecl_ok = true,
            "pid" => pid_ok = true,
            "cid" => {} // optional
            _ => unreachable!(),
        }
    }

    byr_ok && iyr_ok && eyr_ok && hgt_ok && hcl_ok && ecl_ok && pid_ok
}

fn has_all_fields_valid(passport: &str) -> bool {
    let mut byr_ok = false;
    let mut iyr_ok = false;
    let mut eyr_ok = false;
    let mut hgt_ok = false;
    let mut hcl_ok = false;
    let mut ecl_ok = false;
    let mut pid_ok = false;

    fn is_valid_year(year: &str, digit_count: usize, range: RangeInclusive<usize>) -> bool {
        year.len() == digit_count && range.contains(&year.parse::<usize>().unwrap())
    }

    // Each passport is represented as a sequence of `key:value` pairs.
    for key_value in passport.split_ascii_whitespace() {
        let mut key_value = key_value.split(':');
        let key = key_value.next().unwrap();
        let value = key_value.next().unwrap();

        match key {
            // byr (Birth Year) - four digits; at least 1920 and at most 2002.
            "byr" => byr_ok = is_valid_year(value, 4, 1920..=2002),

            // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
            "iyr" => iyr_ok = is_valid_year(value, 4, 2010..=2020),

            // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
            "eyr" => eyr_ok = is_valid_year(value, 4, 2020..=2030),

            // hgt (Height) - a number followed by either cm or in:
            //   - If cm, the number must be at least 150 and at most 193.
            //   - If in, the number must be at least 59 and at most 76.
            "hgt" => {
                let (height, unit) = (&value[..value.len() - 2], &value[value.len() - 2..]);
                hgt_ok = match unit {
                    "cm" => (150..=193).contains(&height.parse::<usize>().unwrap()),
                    "in" => (59..=76).contains(&height.parse::<usize>().unwrap()),
                    _ => false,
                }
            }

            // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
            "hcl" => {
                hcl_ok = value.starts_with('#')
                    && value.len() == 7
                    && value[1..]
                        .chars()
                        .all(|c| matches!(c, '0'..='9' | 'a'..='f'))
            }

            // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
            "ecl" => {
                ecl_ok = matches!(value, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth")
            }

            // pid (Passport ID) - a nine-digit number, including leading zeroes.
            "pid" => pid_ok = value.len() == 9 && value.parse::<usize>().is_ok(),

            // cid (Country ID) - ignored, missing or not.
            "cid" => {}

            _ => unreachable!(),
        }
    }

    byr_ok && iyr_ok && eyr_ok && hgt_ok && hcl_ok && ecl_ok && pid_ok
}

/*
struct Passport {
    /// Birth Year
    byr: Option<usize>,
    /// Issue Year
    iyr: Option<usize>,
    /// Expiration Year
    eyr: Option<usize>,
    /// Height (in `cm` or `in`)
    hgt: Option<usize>,
    /// Hair Color (in hex, e.g. `#fffffd`)
    hcl: Option<[char; 6]>,
    /// Eye Color
    ecl: Option<[char; 3]>,
    /// Passport ID
    pid: Option<usize>,
    /// Country ID (optional, meaning it's a North Pole Credential)
    cid: Option<Option<usize>>,
}
*/
