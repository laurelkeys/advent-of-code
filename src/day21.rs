//! --- Day 21: Allergen Assessment ---

use crate::solver::Solver;
use std::{
    collections::{HashMap, HashSet},
    io::{self, BufRead, BufReader},
};

/// https://adventofcode.com/2020/day/21
pub struct Day21;

#[derive(Clone, Debug)]
pub struct Food {
    ingredients: HashSet<String>,
    allergens: HashSet<String>,
}

impl Solver for Day21 {
    type Input = (Vec<Food>, HashSet<String>, HashSet<String>);
    type Output1 = usize;
    type Output2 = usize;

    fn solve_part1(&self, input: &Self::Input) -> Self::Output1 {
        let (foods, all_ingredients, _) = input;

        // For each allergen, map the ingredients which possibly contain it.
        let mut allergen_possibilities: HashMap<String, HashSet<String>> = HashMap::new();

        for food in foods {
            for allergen in &food.allergens {
                match allergen_possibilities.get_mut(allergen) {
                    Some(ingredients) => {
                        ingredients.retain(|ingredient| food.ingredients.contains(ingredient));
                    }
                    None => {
                        allergen_possibilities.insert(allergen.clone(), food.ingredients.clone());
                    }
                }
            }
        }

        // Determine which ingredients cannot possibly contain any of the allergens in your list.
        // How many times do any of those ingredients appear?
        let mut ingredients_without_allergens = all_ingredients.clone();
        for (_, ingredients_with_allergens) in allergen_possibilities {
            ingredients_without_allergens
                .retain(|ingredient| !ingredients_with_allergens.contains(ingredient));
        }

        foods
            .iter()
            .map(|food| {
                food.ingredients
                    .iter()
                    .filter(|&ingredient| ingredients_without_allergens.contains(ingredient))
                    .count()
            })
            .sum::<usize>()
    }

    fn solve_part2(&self, input: &Self::Input) -> Self::Output2 {
        todo!()
    }

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        let mut all_ingredients = HashSet::new();
        let mut all_allergens = HashSet::new();

        let foods = BufReader::new(r)
            .lines()
            .flatten()
            .map(|line| {
                let mut line = line.trim_end_matches(')').split(" (contains ");

                let ingredients = line
                    .next()
                    .unwrap()
                    .split(' ')
                    .map(String::from)
                    .collect::<HashSet<_>>();

                all_ingredients.extend(ingredients.clone());

                let allergens = match line.next() {
                    None => HashSet::new(),
                    Some(allergens) => allergens
                        .split(", ")
                        .map(String::from)
                        .collect::<HashSet<_>>(),
                };

                all_allergens.extend(allergens.clone());

                Food {
                    ingredients,
                    allergens,
                }
            })
            .collect::<Vec<_>>();

        (foods, all_ingredients, all_allergens)
    }
}
