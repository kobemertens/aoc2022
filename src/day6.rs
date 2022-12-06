use std::collections::HashSet;

use crate::common::Day;

pub struct Day6;

fn check_window(window: &[u8]) -> bool {
    let set: HashSet<u8> = window.iter().copied().collect();
    return set.len() == window.len();
}

fn part(input: &str, window_size: usize) -> usize {
    input
        .as_bytes()
        .windows(window_size)
        .map(check_window)
        .zip(0..)
        .find(|&(b, _)| b)
        .unwrap()
        .1
        + window_size
}

impl<'a> Day<'a> for Day6 {
    type Input = &'a str;
    type Output = usize;

    fn day_number() -> usize {
        6
    }

    fn part1(input: &Self::Input) -> Self::Output {
        part(input, 4)
    }

    fn part2(input: &Self::Input) -> Self::Output {
        part(input, 14)
    }

    fn parse(input: &'a str) -> Self::Input {
        input
    }
}
