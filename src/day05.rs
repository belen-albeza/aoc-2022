use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use lazy_static::lazy_static;
use regex::Regex;

pub type Crate = char;

pub type Stack = Vec<Crate>;

pub type Cargo = Vec<Stack>;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Move {
    amount: u64,
    from: usize,
    to: usize,
}

#[derive(Debug, PartialEq, Clone)]
struct Ship {
    cargo: Cargo,
}

impl Ship {
    fn top_cargo(&self) -> String {
        self.cargo
            .iter()
            .map(|stack| stack.last().unwrap_or(&' '))
            .collect()
    }

    fn move_crate(&mut self, from: usize, to: usize) {
        let cargo_crate = self.cargo[from].pop().unwrap();
        self.cargo[to].push(cargo_crate);
    }

    fn operate_crane(&mut self, moves: &[Move]) {
        for crane_move in moves {
            for _ in 0..crane_move.amount {
                self.move_crate(crane_move.from, crane_move.to);
            }
        }
    }
}

#[aoc_generator(day5)]
pub fn parse_input(input: &str) -> (Cargo, Vec<Move>) {
    let mut chunks = input.split("\n\n");
    let cargo = parse_cargo(chunks.next().unwrap());
    let moves = parse_moves(chunks.next().unwrap());
    (cargo, moves)
}

fn parse_cargo(input: &str) -> Cargo {
    let mut cargo = Cargo::new();

    for line in input.lines().rev() {
        for i in 0..((line.len() + 1) / 4) {
            if let None = cargo.get(i) {
                cargo.push(vec![]);
            }
            if let Some(x) = line.chars().nth((i * 4) + 1) {
                if x.is_alphabetic() {
                    cargo[i].push(x);
                }
            }
        }
    }

    cargo
}

fn parse_moves(input: &str) -> Vec<Move> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\d+").unwrap();
    }

    input
        .lines()
        .into_iter()
        .map(|line| {
            let numbers: Vec<u64> = RE
                .find_iter(line)
                .map(|x| x.as_str().parse::<u64>().unwrap())
                .collect();
            Move {
                amount: numbers[0],
                from: (numbers[1] - 1) as usize,
                to: (numbers[2] - 1) as usize,
            }
        })
        .collect()
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &(Cargo, Vec<Move>)) -> String {
    let mut ship = Ship {
        cargo: input.0.clone(),
    };
    ship.operate_crane(&input.1);
    ship.top_cargo()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ship_top_cargo() {
        let ship = Ship {
            cargo: vec![vec!['A', 'B'], vec!['C'], vec!['D']],
        };
        assert_eq!(ship.top_cargo(), "BCD");
    }

    #[test]
    fn test_ship_move_crane() {
        let mut ship = Ship {
            cargo: vec![vec!['A', 'B'], vec!['C'], vec![]],
        };

        ship.move_crate(0, 1);

        assert_eq!(ship.cargo, [vec!['A'], vec!['C', 'B'], vec![]]);
    }

    #[test]
    fn test_solve_part1() {
        let input = (
            vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']],
            vec![
                Move {
                    amount: 1,
                    from: 1,
                    to: 0,
                },
                Move {
                    amount: 3,
                    from: 0,
                    to: 2,
                },
                Move {
                    amount: 2,
                    from: 1,
                    to: 0,
                },
                Move {
                    amount: 1,
                    from: 0,
                    to: 1,
                },
            ],
        );
        assert_eq!(solve_part1(&input), "CMZ");
    }

    #[test]
    fn test_parse_input() {
        let input = r#"    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#;
        assert_eq!(
            parse_input(input),
            (
                vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P'],],
                vec![
                    Move {
                        amount: 1,
                        from: 1,
                        to: 0
                    },
                    Move {
                        amount: 3,
                        from: 0,
                        to: 2
                    },
                    Move {
                        amount: 2,
                        from: 1,
                        to: 0
                    },
                    Move {
                        amount: 1,
                        from: 0,
                        to: 1
                    },
                ]
            )
        )
    }
}
