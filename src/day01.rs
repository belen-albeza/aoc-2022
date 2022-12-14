use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

type Elf = Vec<u32>;

trait SnacksCarrier {
    fn total_calories(&self) -> u32;
}

impl SnacksCarrier for Elf {
    fn total_calories(&self) -> u32 {
        self.iter().sum()
    }
}

#[aoc_generator(day1)]
pub fn parse_input(input: &str) -> Vec<Elf> {
    let mut res: Vec<Elf> = vec![];
    let mut buffer: Elf = vec![];

    for line in input.lines().map(|x| x.trim()).into_iter() {
        if line.is_empty() {
            res.push(buffer);
            buffer = vec![];
        } else {
            buffer.push(line.parse::<u32>().unwrap())
        }
    }

    return res;
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[Elf]) -> u32 {
    input
        .into_iter()
        .map(|x| x.total_calories())
        .max()
        .unwrap_or(0)
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[Elf]) -> u32 {
    let mut calories_by_elf: Vec<u32> = input.into_iter().map(|x| x.total_calories()).collect();

    const N: u32 = 3;
    let mut top_calories: Vec<u32> = vec![];

    for _ in 0..N {
        let max = calories_by_elf.clone().into_iter().max().unwrap_or(0);
        top_calories.push(max);
        calories_by_elf.remove(calories_by_elf.iter().position(|&x| x == max).unwrap());
    }

    top_calories.into_iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day1_part1() {
        let input: Vec<Elf> = vec![
            vec![1000, 2000, 3000],
            vec![4000],
            vec![5000, 6000],
            vec![7000, 8000, 9000],
            vec![10000],
        ];
        assert_eq!(solve_part1(&input), 24000);
    }

    #[test]
    fn test_day1_part2() {
        let input: Vec<Elf> = vec![
            vec![1000, 2000, 3000],
            vec![4000],
            vec![5000, 6000],
            vec![7000, 8000, 9000],
            vec![10000],
        ];
        assert_eq!(solve_part2(&input), 45000);
    }
}
