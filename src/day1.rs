use crate::common::Day;

pub struct Day1;

type Elf = Vec<u32>;

impl<'a> Day<'a> for Day1 {
    type Input = Vec<Elf>;
    type Output = u32;

    fn day_number() -> usize {
        1
    }

    fn part1(input: &Self::Input) -> Self::Output {
        input.iter().map(|x| x.iter().sum()).max().unwrap()
    }

    fn part2(input: &Self::Input) -> Self::Output {
        let mut maximums: Vec<u32> = input.iter().map(|x| x.iter().sum()).collect();
        maximums.sort_by(|a, b| b.cmp(a));
        maximums.iter().take(3).sum()
    }

    fn parse(input: &'a str) -> Self::Input {
        input[0..input.len()-1]
            .split("\n\n")
            .map(|x| x.split('\n').map(|x| x.parse::<u32>().unwrap()).collect())
            .collect()
    }
}
