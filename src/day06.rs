use aoc_runner_derive::aoc;
use itertools::Itertools;
use std::collections::HashSet;

const PACKET_BUFFER_SIZE: usize = 4;
const MESSAGE_BUFFER_SIZE: usize = 14;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RingBuffer<T: Copy + PartialEq, const SIZE: usize> {
    buffer: [Option<T>; SIZE],
    write_ptr: usize,
}

impl<const SIZE: usize, T: Copy + PartialEq> RingBuffer<T, SIZE> {
    pub fn new() -> Self {
        Self {
            buffer: [None; SIZE],
            write_ptr: 0,
        }
    }

    pub fn is_full(&self) -> bool {
        !self.buffer.iter().contains(&None)
    }

    pub fn len(&self) -> usize {
        if self.is_full() {
            SIZE
        } else {
            self.write_ptr
        }
    }

    pub fn to_vec(&self) -> Vec<T> {
        let res = if self.is_full() {
            let head = self.buffer[self.write_ptr..].to_owned();
            let tail = self.buffer[..self.write_ptr].to_owned();
            [head, tail].concat()
        } else {
            self.buffer.to_vec()
        };

        res.iter().filter_map(|&x| x).collect()
    }

    pub fn push(&mut self, x: T) {
        self.buffer[self.write_ptr] = Some(x);
        self.write_ptr = (self.write_ptr + 1) % SIZE;
    }
}

trait ElvenBuffer<const T: usize> {
    fn is_start_of_marker(&self) -> bool;
}

impl<const T: usize> ElvenBuffer<T> for RingBuffer<char, T> {
    fn is_start_of_marker(&self) -> bool {
        let set: HashSet<char> = HashSet::from_iter(self.to_vec());
        let are_all_items_unique = set.len() == self.len();
        self.is_full() && are_all_items_unique
    }
}

fn get_start_of_marker_for_signal<const T: usize>(
    signal: &str,
    buffer: &mut RingBuffer<char, T>,
) -> usize {
    for (i, item) in signal.chars().enumerate() {
        buffer.push(item);
        if buffer.is_start_of_marker() {
            return i + 1;
        }
    }

    signal.len()
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &str) -> usize {
    let mut buffer = RingBuffer::<char, PACKET_BUFFER_SIZE>::new();
    get_start_of_marker_for_signal(input, &mut buffer)
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &str) -> usize {
    let mut buffer = RingBuffer::<char, MESSAGE_BUFFER_SIZE>::new();
    get_start_of_marker_for_signal(input, &mut buffer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(solve_part1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(solve_part1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(solve_part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(solve_part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(solve_part2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(solve_part2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(solve_part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(solve_part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}
