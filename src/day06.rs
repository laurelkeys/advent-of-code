//! --- Day 6: Custom Customs ---

use crate::solver::Solver;
use std::io;

/// https://adventofcode.com/2020/day/6
pub struct Day06;

const QUESTION_COUNT: usize = 26; // 'a'..='z'

#[derive(Copy, Clone)]
pub struct Answer([bool; QUESTION_COUNT]);

impl Solver for Day06 {
    type Input = Vec<Vec<Answer>>;
    type Output1 = usize;
    type Output2 = usize;

    fn solve_part1(&self, input: &Self::Input) -> Self::Output1 {
        input
            .iter()
            .map(|group_answers| Answer::union(group_answers).unwrap().yes_count())
            .sum()
    }

    fn solve_part2(&self, input: &Self::Input) -> Self::Output2 {
        input
            .iter()
            .map(|group_answers| Answer::intersection(group_answers).unwrap().yes_count())
            .sum()
    }

    fn parse_input<R: io::Read>(&self, mut r: R) -> Self::Input {
        let mut input = String::new();
        r.read_to_string(&mut input).unwrap();

        // Each group's answers are separated by a blank line, and
        // within each group, each person's answers are on a single line.
        input
            .trim_end() // remove trailing newline
            .split("\n\n")
            .map(|group_answers| {
                group_answers
                    .split('\n')
                    .map(|person_answer| Answer::new(person_answer))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
    }
}

impl Answer {
    fn new(person_answer: &str) -> Self {
        Answer(('a'..='z').enumerate().fold(
            [false; QUESTION_COUNT],
            |mut answers, (i, question)| {
                if person_answer.contains(question) {
                    answers[i] = true;
                }
                answers
            },
        ))
    }

    fn yes_count(&self) -> usize {
        self.0.iter().filter(|&&answer| answer).count()
    }

    fn merge(&mut self, other: &Answer) {
        for i in 0..QUESTION_COUNT {
            self.0[i] |= other.0[i];
        }
    }

    fn intersect(&mut self, other: &Answer) {
        for i in 0..QUESTION_COUNT {
            self.0[i] &= other.0[i];
        }
    }

    fn union(answers: &[Answer]) -> Option<Answer> {
        match answers.get(0) {
            Some(first_answer) => {
                let mut merged_answer = *first_answer;
                for answer in &answers[1..] {
                    merged_answer.merge(answer);
                }
                Some(merged_answer)
            }
            None => None,
        }
    }

    fn intersection(answers: &[Answer]) -> Option<Answer> {
        match answers.get(0) {
            Some(first_answer) => {
                let mut intersected_answer = *first_answer;
                for answer in &answers[1..] {
                    intersected_answer.intersect(answer);
                }
                Some(intersected_answer)
            }
            None => None,
        }
    }
}
