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

fn allergen_possibilities(foods: &[Food]) -> HashMap<String, HashSet<String>> {
    let mut possibilities: HashMap<String, HashSet<String>> = HashMap::new();

    for food in foods {
        for allergen in &food.allergens {
            match possibilities.get_mut(allergen) {
                Some(ingredients) => {
                    ingredients.retain(|ingredient| food.ingredients.contains(ingredient));
                }
                None => {
                    possibilities.insert(allergen.clone(), food.ingredients.clone());
                }
            }
        }
    }

    possibilities
}

impl Solver for Day21 {
    type Input = (Vec<Food>, HashSet<String>);
    type Output1 = usize;
    type Output2 = String;

    fn solve_part1(&self, input: &Self::Input) -> Self::Output1 {
        let (foods, all_ingredients) = input;

        // Determine which ingredients cannot possibly contain any of the allergens in your list.
        // How many times do any of those ingredients appear?
        let mut ingredients_without_allergens = all_ingredients.clone();
        for (_, ingredients_with_allergen) in allergen_possibilities(&foods).iter() {
            ingredients_without_allergens
                .retain(|ingredient| !ingredients_with_allergen.contains(ingredient));
        }

        // dbg!(&allergen_possibilities);

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
        let (foods, _) = input;

        // For each allergen, map the ingredients which possibly contain it.
        let mut allergen_possibilities = allergen_possibilities(&foods);

        // For each allergen, map the ingredient which actually contains it.
        let mut allergen_ingredient = HashMap::new();

        while !allergen_possibilities.is_empty() {
            let (allergen, ingredient) = allergen_possibilities
                .iter()
                .find(|(_, ingredients_with_allergen)| ingredients_with_allergen.len() == 1)
                .map(|(allergen, ingredient)| (allergen, ingredient.iter().next().unwrap()))
                .map(|(allergen, ingredient)| (allergen.clone(), ingredient.clone()))
                .unwrap();

            allergen_possibilities.remove(&allergen);
            for (_, ingredients_with_allergen) in allergen_possibilities.iter_mut() {
                ingredients_with_allergen.remove(&ingredient);
            }

            allergen_ingredient.insert(allergen, ingredient);
        }

        // Arrange the ingredients alphabetically by their allergen and separate them by commas
        // (with no spaces) to produce your canonical dangerous ingredient list.
        let mut allergen_ingredient = allergen_ingredient.into_iter().collect::<Vec<_>>();
        allergen_ingredient.sort_by_key(|(allergen, _)| allergen.clone());

        allergen_ingredient
            .into_iter()
            .map(|(_, ingredient)| ingredient)
            .collect::<Vec<_>>()
            .join(",")
    }

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        let mut all_ingredients = HashSet::new();

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

                Food {
                    ingredients,
                    allergens,
                }
            })
            .collect::<Vec<_>>();

        (foods, all_ingredients)
    }
}
