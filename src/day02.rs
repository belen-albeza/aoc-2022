use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

use std::convert::From;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Outcome {
    Win,
    Loss,
    Draw,
}

impl Outcome {
    fn score(&self) -> u64 {
        match self {
            Self::Win => 6,
            Self::Draw => 3,
            Self::Loss => 0,
        }
    }
}

impl From<&str> for Outcome {
    fn from(raw: &str) -> Self {
        match raw {
            "X" => Self::Loss,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => panic!("Unrecognized outcome: {}", raw),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl Choice {
    fn versus(&self, other: Choice) -> Outcome {
        match (self, other) {
            (Choice::Rock, Choice::Rock) => Outcome::Draw,
            (Choice::Rock, Choice::Paper) => Outcome::Loss,
            (Choice::Rock, Choice::Scissors) => Outcome::Win,
            (Choice::Paper, Choice::Rock) => Outcome::Win,
            (Choice::Paper, Choice::Paper) => Outcome::Draw,
            (Choice::Paper, Choice::Scissors) => Outcome::Loss,
            (Choice::Scissors, Choice::Rock) => Outcome::Loss,
            (Choice::Scissors, Choice::Paper) => Outcome::Win,
            (Choice::Scissors, Choice::Scissors) => Outcome::Draw,
        }
    }

    fn score(&self) -> u64 {
        match self {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        }
    }

    pub fn score_versus(&self, other: Choice) -> u64 {
        let outcome = self.versus(other);
        self.score() + outcome.score()
    }
}

impl From<&str> for Choice {
    fn from(raw: &str) -> Self {
        match raw {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => panic!("Unrecognized choice: {}", raw),
        }
    }
}

impl From<RoundWithOutcome> for Choice {
    fn from(round: RoundWithOutcome) -> Self {
        match round {
            (Self::Rock, Outcome::Win) => Self::Paper,
            (Self::Rock, Outcome::Draw) => Self::Rock,
            (Self::Rock, Outcome::Loss) => Self::Scissors,
            (Self::Paper, Outcome::Win) => Self::Scissors,
            (Self::Paper, Outcome::Draw) => Self::Paper,
            (Self::Paper, Outcome::Loss) => Self::Rock,
            (Self::Scissors, Outcome::Win) => Self::Rock,
            (Self::Scissors, Outcome::Draw) => Self::Scissors,
            (Self::Scissors, Outcome::Loss) => Self::Paper,
        }
    }
}

type RoundWithChoices = (Choice, Choice);
type RoundWithOutcome = (Choice, Outcome);

#[aoc_generator(day2, part1)]
pub fn parse_input_part1(input: &str) -> Vec<RoundWithChoices> {
    input
        .lines()
        .into_iter()
        .map(|line| match line.split(' ').collect::<Vec<&str>>()[..] {
            [opponent, player] => (Choice::from(player), Choice::from(opponent)),
            _ => panic!("Each round must contain exactly two plays"),
        })
        .collect()
}

#[aoc_generator(day2, part2)]
pub fn parse_input_part2(input: &str) -> Vec<RoundWithOutcome> {
    input
        .lines()
        .into_iter()
        .map(|line| match line.split(' ').collect::<Vec<&str>>()[..] {
            [opponent, outcome] => (Choice::from(opponent), Outcome::from(outcome)),
            _ => panic!("Each round must contain exactly two plays"),
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[RoundWithChoices]) -> u64 {
    input.into_iter().map(|x| (x.0).score_versus(x.1)).sum()
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[RoundWithOutcome]) -> u64 {
    input
        .into_iter()
        .map(|&x| (Choice::from(x), x.0))
        .map(|x| (x.0).score_versus(x.1))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input_part1() {
        let input = r#"A Y
B X
C Z"#;
        assert_eq!(
            parse_input_part1(input),
            vec![
                (Choice::Paper, Choice::Rock),
                (Choice::Rock, Choice::Paper),
                (Choice::Scissors, Choice::Scissors),
            ]
        )
    }

    #[test]
    fn test_parse_input_part2() {
        let input = r#"A Y
B X
C Z"#;
        assert_eq!(
            parse_input_part2(input),
            vec![
                (Choice::Rock, Outcome::Draw),
                (Choice::Paper, Outcome::Loss),
                (Choice::Scissors, Outcome::Win),
            ]
        )
    }

    #[test]
    fn test_solve_part1() {
        let input = vec![
            (Choice::Paper, Choice::Rock),
            (Choice::Rock, Choice::Paper),
            (Choice::Scissors, Choice::Scissors),
        ];

        assert_eq!(solve_part1(&input), 15);
    }

    #[test]
    fn test_solve_part2() {
        let input = vec![
            (Choice::Rock, Outcome::Draw),
            (Choice::Paper, Outcome::Loss),
            (Choice::Scissors, Outcome::Win),
        ];

        assert_eq!(solve_part2(&input), 12);
    }
}
