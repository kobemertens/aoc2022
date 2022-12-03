use crate::common::Day;

pub struct Day2;

fn part1(inp: (u8, u8)) -> usize {
    match inp {
        (b'A', b'X') => 4,
        (b'A', b'Y') => 8,
        (b'A', b'Z') => 3,
        (b'B', b'X') => 1,
        (b'B', b'Y') => 5,
        (b'B', b'Z') => 9,
        (b'C', b'X') => 7,
        (b'C', b'Y') => 2,
        (b'C', b'Z') => 6,
        _ => panic!("Invalid input"),
    }
}

fn part2(inp: (u8, u8)) -> usize {
    match inp {
        (b'A', b'X') => 3,
        (b'A', b'Y') => 4,
        (b'A', b'Z') => 8,
        (b'B', b'X') => 1,
        (b'B', b'Y') => 5,
        (b'B', b'Z') => 9,
        (b'C', b'X') => 2,
        (b'C', b'Y') => 6,
        (b'C', b'Z') => 7,
        _ => panic!("Invalid input"),
    }
}

impl<'a> Day<'a> for Day2 {
    type Input = Vec<(u8, u8)>;
    type Output = usize;

    fn day_number() -> usize {
        2
    }

    fn part1(input: &Self::Input) -> Self::Output {
        input.iter().copied().map(part1).sum()
    }

    fn part2(input: &Self::Input) -> Self::Output {
        input.iter().copied().map(part2).sum()
    }

    fn parse(input: &'a str) -> Self::Input {
        input[0..input.len() - 1]
            .split('\n')
            .map(|line| {
                let mut bytes = line.as_bytes().iter().copied();
                (bytes.next().unwrap(), bytes.skip(1).next().unwrap())
            })
            .collect()
    }
}
