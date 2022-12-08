use itertools::Itertools;

use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use grid::*;

type Tree = u64;

trait Forest {
    fn is_tree_visible(&self, col: usize, row: usize) -> bool;
    fn scenic_score_for_tree(&self, col: usize, row: usize) -> u64;
    fn tree_col_chunk(&self, col: usize, chunk: &mut dyn Iterator<Item = usize>) -> Vec<Tree>;
    fn tree_row_chunk(&self, row: usize, chunk: &mut dyn Iterator<Item = usize>) -> Vec<Tree>;
}

impl Forest for Grid<Tree> {
    fn is_tree_visible(&self, col: usize, row: usize) -> bool {
        let height = self.get(row, col).unwrap();

        let is_visible =
            |trees: Vec<Tree>| trees.into_iter().filter(|tree| tree >= height).count() == 0;

        let from_north = is_visible(self.tree_col_chunk(col, &mut (0..row)));
        let from_south = is_visible(self.tree_col_chunk(col, &mut (row + 1..self.rows())));
        let from_west = is_visible(self.tree_row_chunk(row, &mut (0..col)));
        let from_east = is_visible(self.tree_row_chunk(row, &mut (col + 1..self.cols())));

        from_north || from_south || from_west || from_east
    }

    fn scenic_score_for_tree(&self, col: usize, row: usize) -> u64 {
        let height = self.get(row, col).unwrap();

        let distance = |trees: Vec<u64>| {
            let max_distance = trees.len();
            trees
                .into_iter()
                .enumerate()
                .find(|(_, tree)| tree >= height)
                .map(|(i, tree)| (i + 1, tree))
                .map_or(max_distance, |x| x.0)
        };

        let north_distance = distance(self.tree_col_chunk(col, &mut (0..row).rev()));
        let south_distance = distance(self.tree_col_chunk(col, &mut ((row + 1)..self.rows())));
        let west_distance = distance(self.tree_row_chunk(row, &mut (0..col).rev()));
        let east_distance = distance(self.tree_row_chunk(row, &mut ((col + 1)..self.cols())));

        ((north_distance) * (south_distance) * (west_distance) * (east_distance)) as u64
    }

    fn tree_col_chunk(&self, col: usize, chunk: &mut dyn Iterator<Item = usize>) -> Vec<Tree> {
        chunk.map(|i| *self.get(i, col).unwrap()).collect()
    }

    fn tree_row_chunk(&self, row: usize, chunk: &mut dyn Iterator<Item = usize>) -> Vec<Tree> {
        chunk.map(|i| *self.get(row, i).unwrap()).collect()
    }
}

#[aoc_generator(day8)]
pub fn parse_input(input: &str) -> Grid<Tree> {
    let (data, n_cols) = input
        .lines()
        .into_iter()
        .fold((vec![], 0), |(mut data, _), line| {
            let cells: Vec<u64> = line
                .chars()
                .into_iter()
                .map(|x| x.to_string().parse::<u64>().unwrap())
                .collect();
            data.extend(cells);
            (data, line.len())
        });

    Grid::from_vec(data, n_cols)
}

#[aoc(day8, part1)]
pub fn solve_part1(forest: &Grid<Tree>) -> u64 {
    (0..forest.cols())
        .cartesian_product(0..forest.rows())
        .filter(|(x, y)| forest.is_tree_visible(*x, *y))
        .count() as u64
}

#[aoc(day8, part2)]
pub fn solve_part2(forest: &Grid<Tree>) -> u64 {
    (0..forest.cols())
        .cartesian_product(0..forest.rows())
        .map(|(x, y)| forest.scenic_score_for_tree(x, y))
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn raw_input() -> &'static str {
        r#"30373
25512
65332
33549
35390"#
    }

    fn input() -> Grid<Tree> {
        grid![
            [3, 0, 3, 7, 3]
            [2, 5, 5, 1, 2]
            [6, 5, 3, 3, 2]
            [3, 3, 5, 4, 9]
            [3, 5, 3, 9, 0]
        ]
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(parse_input(raw_input()), input());
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(&input()), 21);
    }

    #[test]
    fn test_forest_is_tree_visible() {
        let forest = input();
        assert_eq!(forest.is_tree_visible(0, 0), true);
        assert_eq!(forest.is_tree_visible(4, 4), true);
        assert_eq!(forest.is_tree_visible(1, 1), true);
        assert_eq!(forest.is_tree_visible(2, 1), true);
        assert_eq!(forest.is_tree_visible(3, 1), false);
        assert_eq!(forest.is_tree_visible(1, 2), true);
        assert_eq!(forest.is_tree_visible(2, 2), false);
        assert_eq!(forest.is_tree_visible(3, 2), true);
        assert_eq!(forest.is_tree_visible(2, 3), true);
        assert_eq!(forest.is_tree_visible(1, 3), false);
        assert_eq!(forest.is_tree_visible(3, 3), false);
    }

    #[test]
    fn test_forest_scenic_score() {
        let forest = input();
        assert_eq!(forest.scenic_score_for_tree(2, 1), 4);
        assert_eq!(forest.scenic_score_for_tree(2, 3), 8);
    }
}
