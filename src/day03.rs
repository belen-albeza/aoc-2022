use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

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

type Rucksack = Vec<Item>;

trait Compartments {
    fn common_item(&self) -> Item;
}

impl Compartments for Rucksack {
    fn common_item(&self) -> Item {
        let left = self[0..self.len() / 2].to_vec();
        let right = &self[self.len() / 2..self.len()].to_vec();
        left.into_iter()
            .fold(None, |res, item| {
                if right.contains(&item) {
                    Some(item)
                } else {
                    res
                }
            })
            .unwrap()
    }
}

#[aoc_generator(day3)]
pub fn parse_input(input: &str) -> Vec<Rucksack> {
    input.lines().map(|x| x.chars().collect()).collect()
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &[Rucksack]) -> u32 {
    input
        .into_iter()
        .map(|rucksack| rucksack.common_item().priority())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_rucksack_common_item() {
        let rucksack: Rucksack = "vJrwpWtwJgWrhcsFMMfFFhFp".chars().collect();
        assert_eq!(rucksack.common_item(), 'p');
    }

    #[test]
    pub fn test_parse_input() {
        let input = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#;
        assert_eq!(
            parse_input(input),
            vec![
                "vJrwpWtwJgWrhcsFMMfFFhFp".chars().collect::<Vec<char>>(),
                "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"
                    .chars()
                    .collect::<Vec<char>>(),
                "PmmdzqPrVvPwwTWBwg".chars().collect::<Vec<char>>(),
                "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"
                    .chars()
                    .collect::<Vec<char>>(),
                "ttgJtRGJQctTZtZT".chars().collect::<Vec<char>>(),
                "CrZsJsPPZsGzwwsLwLmpwMDw".chars().collect::<Vec<char>>(),
            ]
        )
    }

    #[test]
    fn test_solve_part1() {
        let input = [
            "vJrwpWtwJgWrhcsFMMfFFhFp".chars().collect::<Vec<char>>(),
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"
                .chars()
                .collect::<Vec<char>>(),
            "PmmdzqPrVvPwwTWBwg".chars().collect::<Vec<char>>(),
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"
                .chars()
                .collect::<Vec<char>>(),
            "ttgJtRGJQctTZtZT".chars().collect::<Vec<char>>(),
            "CrZsJsPPZsGzwwsLwLmpwMDw".chars().collect::<Vec<char>>(),
        ];
        assert_eq!(solve_part1(&input), 157);
    }
}
