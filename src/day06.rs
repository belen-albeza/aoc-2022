use aoc_runner_derive::aoc;
use ringbuffer::{ConstGenericRingBuffer, RingBuffer, RingBufferExt, RingBufferWrite};
use std::collections::HashSet;

const BUFFER_SIZE: usize = 4;

trait ElvenBuffer {
    fn is_start_of_packet(&self) -> bool;
}

impl ElvenBuffer for ConstGenericRingBuffer<char, BUFFER_SIZE> {
    fn is_start_of_packet(&self) -> bool {
        let set: HashSet<char> = HashSet::from_iter(self.to_vec());
        let are_all_items_unique = set.len() == self.len();
        self.is_full() && are_all_items_unique
    }
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &str) -> usize {
    let mut buffer = ConstGenericRingBuffer::<char, BUFFER_SIZE>::new();

    for (i, item) in input.chars().enumerate() {
        buffer.push(item);
        if buffer.is_start_of_packet() {
            return i + 1;
        }
    }

    input.len()
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
}
