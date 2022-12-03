use std::fmt::Debug;

pub trait Day<'a> {
    type Input: 'a;
    type Output: Debug;
    fn day_number() -> usize;
    fn parse(input: &'a str) -> Self::Input;
    fn part1(input: &Self::Input) -> Self::Output;
    fn part2(input: &Self::Input) -> Self::Output;
}