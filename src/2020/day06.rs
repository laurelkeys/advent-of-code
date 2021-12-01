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
            .map(|group_answers| Answer::union_of(group_answers).unwrap().yes_count())
            .sum()
    }

    fn solve_part2(&self, input: &Self::Input) -> Self::Output2 {
        input
            .iter()
            .map(|group_answers| Answer::intersection_of(group_answers).unwrap().yes_count())
            .sum()
    }

    fn parse_input<R: io::Read>(&self, mut r: R) -> Self::Input {
        let mut input = String::new();
        r.read_to_string(&mut input).unwrap();

        // Each group's answers are separated by a blank line, and
        // within each group, each person's answers are on a single line.
        input
            .split("\n\n")
            .map(|group_answers| {
                group_answers
                    .split_whitespace()
                    .map(|person_answer| Answer::new(person_answer))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
    }
}

impl Answer {
    fn new(person_answer: &str) -> Self {
        let mut answer = [false; QUESTION_COUNT];
        for (i, question) in ('a'..='z').enumerate() {
            answer[i] = person_answer.contains(question);
        }
        Answer(answer)
    }

    fn yes_count(&self) -> usize {
        self.0.iter().filter(|&&answer| answer).count()
    }

    fn union_of(answers: &[Answer]) -> Option<Answer> {
        Self::collapse(answers, |a, b| a || b)
    }

    fn intersection_of(answers: &[Answer]) -> Option<Answer> {
        Self::collapse(answers, |a, b| a && b)
    }

    fn collapse<F>(answers: &[Answer], f: F) -> Option<Answer>
    where
        F: Fn(bool, bool) -> bool,
    {
        match answers {
            [first_answer, answers @ ..] => Some(answers.iter().fold(
                *first_answer,
                |mut collapsed_answer, answer| {
                    for i in 0..QUESTION_COUNT {
                        collapsed_answer.0[i] = f(collapsed_answer.0[i], answer.0[i]);
                    }
                    collapsed_answer
                },
            )),
            _ => None,
        }
    }
}
