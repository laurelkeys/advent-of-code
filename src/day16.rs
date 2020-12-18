//! --- Day 16: Ticket Translation ---

use crate::solver::Solver;
use lazy_static::lazy_static;
use std::{io, ops::RangeInclusive};

/// https://adventofcode.com/2020/day/16
pub struct Day16;

type Value = usize;
type Ticket = Vec<Value>;

type Field = (String, (RangeInclusive<usize>, RangeInclusive<usize>));

/// Returns true iff `value` is in the valid range of `field`.
fn is_valid(value: &Value, field: &Field) -> bool {
    let f_range = &field.1;
    f_range.0.contains(value) || f_range.1.contains(value)
}

/// Returns true iff `value` is valid for at least one field of `fields`.
fn has_valid(value: &Value, fields: &[Field]) -> bool {
    fields.iter().any(|f| is_valid(value, f))
}

impl Solver for Day16 {
    type Input = (Vec<Field>, Ticket, Vec<Ticket>);
    type Output1 = usize;
    type Output2 = usize;

    fn solve_part1(&self, input: &Self::Input) -> Self::Output1 {
        let (fields, _, nearby_tickets) = input;

        // Add together all invalid values.
        nearby_tickets
            .iter()
            .flat_map(|t| t.iter().filter(|v| !has_valid(v, fields)))
            .sum()
    }

    fn solve_part2(&self, input: &Self::Input) -> Self::Output2 {
        let (fields, my_ticket, nearby_tickets) = input;

        // Discard tickets with invalid values.
        let mut tickets = vec![my_ticket];
        tickets.extend(
            nearby_tickets
                .iter()
                .filter(|t| t.iter().all(|v| has_valid(v, fields))),
        );

        // For each ticket value, store the possible field indices as a bitmask.
        let mut possible_fields_per_value = (0..my_ticket.len())
            .map(|v_i| {
                (0..fields.len()).fold(0u64, |possible_fields, f_i| {
                    let is_possible = tickets.iter().all(|t| is_valid(&t[v_i], &fields[f_i]));
                    possible_fields | ((is_possible as u64) << f_i)
                })
            })
            .collect::<Vec<u64>>();

        // For each ticket value, store its corresponding field index.
        let mut correct_field_per_value = vec![None; my_ticket.len()];
        while let Some(v_i) = possible_fields_per_value
            .iter()
            .position(|possible_fields| possible_fields.count_ones() == 1)
        {
            let correct_field = possible_fields_per_value[v_i];
            let f_i = correct_field.trailing_zeros() as usize;
            correct_field_per_value[v_i] = Some(f_i);
            for possible_fields in possible_fields_per_value.iter_mut() {
                *possible_fields &= !correct_field;
            }
        }
        /* assert!(correct_field_per_value.iter().all(|f_i| f_i.is_some())); */

        // Multiply together the values in my ticket of "departure" fields.
        my_ticket
            .iter()
            .enumerate()
            .filter(|&(v_i, _)| {
                let f_i = correct_field_per_value[v_i].unwrap();
                fields[f_i].0.starts_with("departure")
            })
            .map(|(_, v)| *v)
            .product()
    }

    fn parse_input<R: io::Read>(&self, mut r: R) -> Self::Input {
        let mut input = String::new();
        r.read_to_string(&mut input).unwrap();
        let mut input = input.trim_end().split("\n\n");

        let fields = input.next().unwrap().lines();
        let my_ticket = input.next().unwrap().lines();
        let nearby_tickets = input.next().unwrap().lines();

        (
            fields.map(parse_field).collect(),
            my_ticket.skip(1).map(parse_ticket).next().unwrap(),
            nearby_tickets.skip(1).map(parse_ticket).collect(),
        )
    }
}

fn parse_field(rule: &str) -> Field {
    use regex::Regex; // https://docs.rs/regex/1.4.2/regex/#syntax

    lazy_static! {
        static ref FIELD_RE: Regex = Regex::new(r"([a-z ]+): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();
    };

    let captures = FIELD_RE.captures(rule).unwrap();
    let field = captures[1].to_string();
    let range = |start: usize, end: usize| {
        captures[start].parse().unwrap()..=captures[end].parse().unwrap()
    };

    (field, (range(2, 3), range(4, 5)))
}

fn parse_ticket(ticket: &str) -> Ticket {
    ticket
        .split(',')
        .map(|value| value.parse().unwrap())
        .collect()
}
