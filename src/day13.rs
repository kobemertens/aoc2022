use crate::common::Day;

pub struct Day13;

pub enum Value {
    Literal(usize),
    List(Vec<Value>),
}

enum Animal {
    Dog,
    Cat,
    Horse(char),
}

type Pair = (Value, Value);

fn parse_number() {

}

fn parse_list() {

}

fn parse_value(inp: &str) -> (Option<Value>, &str) {
    let mut chars = inp.chars().peekable();
    let next_char = chars.peek();
    if next_char == None {
        return (None, inp);
    } else if let Some(c) = next_char {
        
    }
}

impl<'a> Day<'a> for Day13 {
    type Input = Vec<Pair>;
    type Output = usize;

    fn day_number() -> usize {
        13
    }

    fn part1(input: &Self::Input) -> Self::Output {
        0
    }

    fn part2(input: &Self::Input) -> Self::Output {
        0
    }

    fn parse(input: &'a str) -> Self::Input {
        input
            .split("\n\n")
            .map(|x| {
                let mut lines = x.lines();
                (
                    parse_value(lines.next().unwrap()),
                    parse_value(lines.next().unwrap()),
                )
            })
            .collect()
    }
}
