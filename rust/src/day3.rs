use std::collections::HashSet;

use crate::common::{self, Day};

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
    let first_half: HashSet<u8> = bp[0..bp.len() / 2].iter().map(|&x| x).collect();
    let second_half: HashSet<u8> = bp[bp.len() / 2..bp.len()].iter().map(|&x| x).collect();
    let intersection = first_half.intersection(&second_half);
    for item in intersection {
        return *item;
    }
    0
}

fn common_item_3(bp1: &Backpack, bp2: &Backpack, bp3: &Backpack) -> u8 {
    let bp1_set: HashSet<u8> = bp1.iter().map(|x| x.to_owned()).collect();
    let bp2_set: HashSet<u8> = bp2.iter().map(|x| x.to_owned()).collect();
    let intersection1: HashSet<u8> = bp1_set
        .intersection(&bp2_set)
        .into_iter()
        .map(|x| x.to_owned())
        .collect();
    let bp3_set: HashSet<u8> = bp3.iter().map(|x| x.to_owned()).collect();
    for item in intersection1.intersection(&bp3_set) {
        return *item;
    }
    0
}

type Backpack = Vec<u8>;

impl<'a> Day<'a> for Day3 {
    type Input = Vec<Backpack>;
    type Output = usize;

    fn day_number() -> usize {
        3
    }

    fn part1(input: &Self::Input) -> Self::Output {
        let mut sum: usize = 0;
        for i in input.iter().map(common_item) {
            sum += usize::from(i);
        }
        sum
    }

    fn part2(input: &Self::Input) -> Self::Output {
        let mut sum: usize = 0;
        for i in (0..input.len()).step_by(3) {
            let common_el = common_item_3(&input[i], &input[i + 1], &input[i + 2]);
            sum += usize::from(common_el);
        }
        sum
    }

    fn parse(input: &'a str) -> Self::Input {
        input.lines().map(parse_line).collect()
    }
}
