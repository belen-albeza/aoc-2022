use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

use itertools::Itertools;
use std::convert::From;

type Item = char;

trait Prioritizable {
    fn priority(&self) -> u32;
}

impl Prioritizable for Item {
    fn priority(&self) -> u32 {
        match self {
            'a'..='z' => (*self as u32) - 96,
            'A'..='Z' => (*self as u32) - 38,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Rucksack {
    items: Vec<Item>,
}

impl From<&str> for Rucksack {
    fn from(raw: &str) -> Self {
        Self {
            items: raw.chars().collect(),
        }
    }
}

impl Rucksack {
    fn compartments(&self) -> (Rucksack, Rucksack) {
        let left = self.items[0..self.items.len() / 2].to_owned();
        let right = self.items[self.items.len() / 2..self.items.len()].to_owned();
        return (Rucksack { items: left }, Rucksack { items: right });
    }

    fn common_items(&self, other: &Rucksack) -> Vec<Item> {
        self.items.iter().fold(vec![], |mut res: Vec<Item>, &item| {
            if other.items.contains(&item) {
                res.push(item);
            }
            res
        })
    }

    pub fn common_item_in_compartments(&self) -> Item {
        let pockets = self.compartments();
        let common = pockets.0.common_items(&pockets.1);
        common[0]
    }

    pub fn common_item_among(list: &[Rucksack]) -> Item {
        let first = list[0].clone();
        let others: Vec<Rucksack> = list[1..list.len()].iter().map(|x| x.clone()).collect();

        let common = others.into_iter().fold(first, |res, rucksack| Rucksack {
            items: rucksack.common_items(&res).to_owned(),
        });

        common.items[0]
    }
}

type Group = Vec<Rucksack>;

#[aoc_generator(day3, part1)]
pub fn parse_input_part1(input: &str) -> Vec<Rucksack> {
    input.lines().map(|x| Rucksack::from(x)).collect()
}

#[aoc_generator(day3, part2)]
pub fn parse_input_part2(input: &str) -> Vec<Group> {
    let rucksacks = parse_input_part1(input);
    rucksacks
        .into_iter()
        .chunks(3)
        .into_iter()
        .map(|x| x.collect())
        .collect()
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &[Rucksack]) -> u32 {
    input
        .into_iter()
        .map(|rucksack| rucksack.common_item_in_compartments().priority())
        .sum()
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &[Group]) -> u32 {
    input
        .into_iter()
        .map(|group| Rucksack::common_item_among(&group).priority())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_rucksack_common_item_in_compartments() {
        let rucksack = Rucksack {
            items: "vJrwpWtwJgWrhcsFMMfFFhFp".chars().collect(),
        };
        assert_eq!(rucksack.common_item_in_compartments(), 'p');
    }

    #[test]
    pub fn test_common_item_among() {
        let rucksacks = vec![
            Rucksack::from("vJrwpWtwJgWrhcsFMMfFFhFp"),
            Rucksack::from("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"),
            Rucksack::from("PmmdzqPrVvPwwTWBwg"),
        ];
        assert_eq!(Rucksack::common_item_among(&rucksacks), 'r');
    }

    #[test]
    pub fn test_parse_input_part1() {
        let input = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#;
        assert_eq!(
            parse_input_part1(input),
            vec![
                Rucksack {
                    items: "vJrwpWtwJgWrhcsFMMfFFhFp".chars().collect()
                },
                Rucksack {
                    items: "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".chars().collect(),
                },
                Rucksack {
                    items: "PmmdzqPrVvPwwTWBwg".chars().collect(),
                },
                Rucksack {
                    items: "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn".chars().collect(),
                },
                Rucksack {
                    items: "ttgJtRGJQctTZtZT".chars().collect(),
                },
                Rucksack {
                    items: "CrZsJsPPZsGzwwsLwLmpwMDw".chars().collect(),
                },
            ]
        )
    }

    #[test]
    fn test_solve_part1() {
        let input = [
            Rucksack::from("vJrwpWtwJgWrhcsFMMfFFhFp"),
            Rucksack::from("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"),
            Rucksack::from("PmmdzqPrVvPwwTWBwg"),
            Rucksack::from("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"),
            Rucksack::from("ttgJtRGJQctTZtZT"),
            Rucksack::from("CrZsJsPPZsGzwwsLwLmpwMDw"),
        ];
        assert_eq!(solve_part1(&input), 157);
    }

    #[test]
    fn test_parse_input_part2() {
        let input = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#;
        assert_eq!(
            parse_input_part2(&input),
            vec![
                [
                    Rucksack::from("vJrwpWtwJgWrhcsFMMfFFhFp"),
                    Rucksack::from("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"),
                    Rucksack::from("PmmdzqPrVvPwwTWBwg"),
                ],
                [
                    Rucksack::from("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"),
                    Rucksack::from("ttgJtRGJQctTZtZT"),
                    Rucksack::from("CrZsJsPPZsGzwwsLwLmpwMDw"),
                ],
            ]
        );
    }

    #[test]
    fn test_solve_part2() {
        let input = vec![
            vec![
                Rucksack::from("vJrwpWtwJgWrhcsFMMfFFhFp"),
                Rucksack::from("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"),
                Rucksack::from("PmmdzqPrVvPwwTWBwg"),
            ],
            vec![
                Rucksack::from("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"),
                Rucksack::from("ttgJtRGJQctTZtZT"),
                Rucksack::from("CrZsJsPPZsGzwwsLwLmpwMDw"),
            ],
        ];

        assert_eq!(solve_part2(&input), 70)
    }
}
