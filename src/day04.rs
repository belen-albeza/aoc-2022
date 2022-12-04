use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

type Section = std::ops::RangeInclusive<usize>;
type Pair = (Section, Section);

trait Assignment {
    fn is_redundant(&self) -> bool;
    fn has_overlap(&self) -> bool;
}

impl Assignment for Pair {
    fn is_redundant(&self) -> bool {
        (self.0.start() >= self.1.start() && self.0.end() <= self.1.end())
            || (self.1.start() >= self.0.start() && self.1.end() <= self.0.end())
    }

    fn has_overlap(&self) -> bool {
        (self.0.contains(self.1.start()) || self.0.contains(self.1.end()))
            || (self.1.contains(self.0.start()) || self.1.contains(self.0.end()))
    }
}

#[aoc_generator(day4)]
pub fn parse_input(input: &str) -> Vec<Pair> {
    input
        .lines()
        .into_iter()
        .map(|x| {
            let pairs: Vec<Section> = x
                .split(",")
                .map(|pair| {
                    let mut tokens = pair.split("-");
                    let start = tokens.next().unwrap().parse::<usize>().unwrap();
                    let end = tokens.next().unwrap().parse::<usize>().unwrap();

                    start..=end
                })
                .collect();
            (pairs[0].clone(), pairs[1].clone())
        })
        .collect()
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &[Pair]) -> u64 {
    input.into_iter().filter(|x| x.is_redundant()).count() as u64
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &[Pair]) -> u64 {
    input.into_iter().filter(|x| x.has_overlap()).count() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#;
        assert_eq!(
            parse_input(input),
            vec![
                (2..=4, 6..=8),
                (2..=3, 4..=5),
                (5..=7, 7..=9),
                (2..=8, 3..=7),
                (6..=6, 4..=6),
                (2..=6, 4..=8),
            ]
        )
    }

    #[test]
    pub fn test_solve_part1() {
        let input = vec![
            (2..=4, 6..=8),
            (2..=3, 4..=5),
            (5..=7, 7..=9),
            (2..=8, 3..=7),
            (6..=6, 4..=6),
            (2..=6, 4..=8),
        ];
        assert_eq!(solve_part1(&input), 2);
    }

    #[test]
    pub fn test_solve_part2() {
        let input = vec![
            (2..=4, 6..=8),
            (2..=3, 4..=5),
            (5..=7, 7..=9),
            (2..=8, 3..=7),
            (6..=6, 4..=6),
            (2..=6, 4..=8),
        ];
        assert_eq!(solve_part2(&input), 4);
    }
}
