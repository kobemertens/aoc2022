use crate::common::Day;

pub struct Day2;

#[derive(Debug, PartialEq)]
pub enum Shape {
    Rock,
    Paper,
    Scissors,
}

type Turn = (Shape, Shape);

fn get_my_points(turn: &Turn) -> usize {
    let shape_score = match turn.1 {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3,
    };

    let outcome_score = match turn {
        (Shape::Rock, Shape::Paper) => 6,
        (Shape::Paper, Shape::Scissors) => 6,
        (Shape::Scissors, Shape::Rock) => 6,
        (x, y) => {
            if x == y {
                3
            } else {
                0
            }
        }
    };

    outcome_score + shape_score
}

fn my_move(opponent: &Shape, outcome: &Shape) -> usize {
    match (opponent, outcome) {
        (Shape::Rock, Shape::Rock) => 3,
        (Shape::Rock, Shape::Paper) => 4,
        (Shape::Rock, Shape::Scissors) => 8,

        (Shape::Paper, Shape::Rock) => 1,
        (Shape::Paper, Shape::Paper) => 5,
        (Shape::Paper, Shape::Scissors) => 9,

        (Shape::Scissors, Shape::Rock) => 2,
        (Shape::Scissors, Shape::Paper) => 6,
        (Shape::Scissors, Shape::Scissors) => 7,
    }
}

fn parse_turn(line: &str) -> Turn {
    let mut parts = line.split(' ');
    let first = match parts.next().unwrap() {
        "A" => Shape::Rock,
        "B" => Shape::Paper,
        "C" => Shape::Scissors,
        _ => panic!("Should never happen"),
    };

    let second = match parts.next().unwrap() {
        "X" => Shape::Rock,
        "Y" => Shape::Paper,
        "Z" => Shape::Scissors,
        _ => panic!("Should never happen"),
    };

    (first, second)
}

impl<'a> Day<'a> for Day2 {
    type Input = Vec<Turn>;
    type Output = usize;

    fn day_number() -> usize {
        2
    }

    fn part1(input: &Self::Input) -> Self::Output {
        input.iter().map(get_my_points).sum()
    }

    fn part2(input: &Self::Input) -> Self::Output {
        input.iter().map(|(x, y)| my_move(x, y)).sum()
    }

    fn parse(input: &'a str) -> Self::Input {
        input[0..input.len()-1].split('\n').map(parse_turn).collect()
    }
}
