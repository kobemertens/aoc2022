use crate::common::Day;

pub struct Day4;

type Range = (u32, u32);
type Pair = (Range, Range);

fn parse_range(line: &str) -> Range {
    let mut parts = line.split('-');
    (
        parts.next().unwrap().parse::<u32>().unwrap(),
        parts.next().unwrap().parse::<u32>().unwrap(),
    )
}

fn parse_line(line: &str) -> Pair {
    let mut parts = line.split(',');
    (
        parse_range(parts.next().unwrap()),
        parse_range(parts.next().unwrap()),
    )
}

impl<'a> Day<'a> for Day4 {
    type Input = Vec<Pair>;
    type Output = usize;

    fn day_number() -> usize {
        4
    }

    fn part1(input: &Self::Input) -> Self::Output {
        input
            .iter()
            .copied()
            .filter(|((x1, y1), (x2, y2))| (x1 >= x2 && y1 <= y2) || (x2 >= x1 && y2 <= y1))
            .count()
    }

    fn part2(input: &Self::Input) -> Self::Output {
        input
            .iter()
            .copied()
            .filter(|((x1, y1), (x2, y2))| x1 <= y2 && y1 >= x2)
            .count()
    }

    fn parse(input: &'a str) -> Self::Input {
        input[0..input.len() - 1]
            .split('\n')
            .map(parse_line)
            .collect()
    }
}
