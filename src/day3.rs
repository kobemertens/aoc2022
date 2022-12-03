use std::collections::HashSet;

use crate::common::Day;

pub struct Day3;

fn parse_line(line: &str) -> Vec<u8> {
    line.chars()
        .map(|x| {
            let offset = if x.is_uppercase() { 38 } else { 96 };
            let index = u32::from(x).saturating_sub(offset);
            index.try_into().unwrap()
        })
        .collect()
}

fn common_item(bp: &Backpack) -> u8 {
    let first_half: HashSet<u8> = bp[0..bp.len() / 2].iter().copied().collect();
    let second_half: HashSet<u8> = bp[bp.len() / 2..bp.len()].iter().copied().collect();
    first_half
        .intersection(&second_half)
        .copied()
        .next()
        .unwrap()
}

fn common_item_3(bp1: &Backpack, bp2: &Backpack, bp3: &Backpack) -> u8 {
    let bp1_set: HashSet<u8> = bp1.iter().copied().collect();
    let bp2_set: HashSet<u8> = bp2.iter().copied().collect();
    let bp3_set: HashSet<u8> = bp3.iter().copied().collect();

    let inter: HashSet<u8> = bp1_set.intersection(&bp2_set).copied().collect();
    inter.intersection(&bp3_set).copied().next().unwrap()
}

type Backpack = Vec<u8>;

impl<'a> Day<'a> for Day3 {
    type Input = Vec<Backpack>;
    type Output = usize;

    fn day_number() -> usize {
        3
    }

    fn part1(input: &Self::Input) -> Self::Output {
        input.iter().map(common_item).map(|x| usize::from(x)).sum()
    }

    fn part2(input: &Self::Input) -> Self::Output {
        input
            .chunks(3)
            .map(|x| common_item_3(&x[0], &x[1], &x[2]))
            .map(|x| usize::from(x))
            .sum()
    }

    fn parse(input: &'a str) -> Self::Input {
        input.lines().map(parse_line).collect()
    }
}
