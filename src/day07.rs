//! --- Day 7: Handy Haversacks ---

use crate::solver::Solver;
use std::{
    collections::{HashMap, HashSet},
    io::{self, BufRead, BufReader},
};

/// https://adventofcode.com/2020/day/7
pub struct Day07;

#[derive(Default)]
/// Map of each bag to its rules.
pub struct Bags(HashMap<String, BagRule>);

#[derive(Default)]
pub struct BagRule {
    /// Map of each "contained" bag to the amount of it that this bag can carry.
    pub contents: HashMap<String, usize>,
    /// Set of all bags which can directly hold this bag inside.
    pub containers: HashSet<String>,
}

const MY_BAG: &str = "shiny gold";

impl Solver for Day07 {
    type Input = Bags;
    type Output1 = usize;
    type Output2 = usize;

    fn solve_part1(&self, input: &Self::Input) -> Self::Output1 {
        // How many bag colors can eventually contain at least one shiny gold bag?
        if let Some(my_containers) = input.0.get(MY_BAG).map(|my_bag| &my_bag.containers) {
            let mut my_containers = my_containers.clone();
            loop {
                let mut more_containers = HashSet::new();

                for container in &my_containers {
                    if let Some(its_containers) = input
                        .0
                        .get(container)
                        .map(|my_container| &my_container.containers)
                    {
                        for its_container in its_containers {
                            if !my_containers.contains(its_container) {
                                more_containers.insert(its_container.clone());
                            }
                        }
                    }
                }

                if more_containers.is_empty() {
                    break my_containers.len();
                }

                my_containers.extend(more_containers);
            }
        } else {
            0
        }
    }

    fn solve_part2(&self, input: &Self::Input) -> Self::Output2 {
        // How many individual bags are required inside your single shiny gold bag?
        input.max_amount_inside(MY_BAG)
    }

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        use regex::Regex; // https://docs.rs/regex/1.4.2/regex/#syntax

        let re = Regex::new(r" bags?(?: contain|,|\.) *").unwrap();

        let mut bags = Bags::default();

        BufReader::new(r).lines().flatten().for_each(|line| {
            let mut line = re.split(&line).collect::<Vec<&str>>().into_iter();

            // Read the first bag (which contains the remaining ones).
            let container_bag = line.next().unwrap().to_string();

            for content in line.filter(|&content| !content.is_empty() && content != "no other") {
                let mut split = content.splitn(2, ' ');

                let max_amount = split.next().unwrap().parse::<usize>().unwrap();
                let contained_bag = split.next().unwrap().to_string();

                // @Cleanup: can we use &str instead of String to remove some clones?
                bags.add_content_to(container_bag.clone(), contained_bag.clone(), max_amount);
                bags.add_container_of(contained_bag, container_bag.clone());
            }
        });

        bags
    }
}

impl Bags {
    fn add_content_to(&mut self, bag: String, contained_bag: String, max_amount: usize) {
        self.0
            .entry(bag)
            .or_insert_with(BagRule::default)
            .contents
            .insert(contained_bag, max_amount);
    }

    fn add_container_of(&mut self, bag: String, container_bag: String) {
        self.0
            .entry(bag)
            .or_insert_with(BagRule::default)
            .containers
            .insert(container_bag);
    }

    fn max_amount_inside(&self, bag: &str) -> usize {
        match self.0.get(bag) {
            Some(bag) => bag
                .contents
                .iter()
                .fold(0, |acc, (contained_bag, max_amount)| {
                    acc + max_amount * (1 + self.max_amount_inside(contained_bag))
                }),
            None => 0,
        }
    }
}

/* Example:
    light red     bags contain  1 bright white bag ,  2 muted yellow  bags.
    dark orange   bags contain  3 bright white bags,  4 muted yellow  bags.
    bright white  bags contain  1 shiny gold                          bag .
    muted yellow  bags contain  2 shiny gold   bags,  9 faded blue    bags.
    shiny gold    bags contain  1 dark olive   bag ,  2 vibrant plum  bags.
    dark olive    bags contain  3 faded blue   bags,  4 dotted black  bags.
    vibrant plum  bags contain  5 faded blue   bags,  6 dotted black  bags.
    faded blue    bags contain    no other                            bags.
    dotted black  bags contain    no other                            bags.
*/
