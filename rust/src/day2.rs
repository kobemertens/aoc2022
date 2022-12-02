use crate::common::Day;

pub struct Day2;

#[derive(Debug, PartialEq)]
pub enum Item {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
pub enum Outcome {
    Loss,
    Draw,
    Win,
}

fn map_item_to_outcome(item: &Item) -> Outcome {
    match item {
        Item::Rock => Outcome::Loss,
        Item::Paper => Outcome::Draw,
        Item::Scissors => Outcome::Win,
    }
}

type Turn = (Item, Item);

fn outcome_to_score(outcome: &Outcome) -> usize {
    match outcome {
        Outcome::Loss => 0,
        Outcome::Draw => 3,
        Outcome::Win => 6,
    }
}

fn get_my_points(turn: &Turn) -> usize {
    let shape_score = match turn.1 {
        Item::Rock => 1,
        Item::Paper => 2,
        Item::Scissors => 3,
    };

    let outcome_score = match turn {
        (Item::Rock, Item::Paper) => 6,
        (Item::Paper, Item::Scissors) => 6,
        (Item::Scissors, Item::Rock) => 6,
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

fn my_move(opponent: &Item, outcome: &Outcome) -> usize {
    match (opponent, outcome) {
        (Item::Rock, Outcome::Loss) => 3,
        (Item::Rock, Outcome::Draw) => 4,
        (Item::Rock, Outcome::Win) => 8,

        (Item::Paper, Outcome::Loss) => 1,
        (Item::Paper, Outcome::Draw) => 5,
        (Item::Paper, Outcome::Win) => 9,

        (Item::Scissors, Outcome::Loss) => 2,
        (Item::Scissors, Outcome::Draw) => 6,
        (Item::Scissors, Outcome::Win) => 7,
    }
}

fn parse_turn(line: &str) -> Turn {
    let mut parts = line.split(' ');
    let first = match parts.next().unwrap() {
        "A" => Item::Rock,
        "B" => Item::Paper,
        "C" => Item::Scissors,
        _ => panic!("Should never happen"),
    };

    let second = match parts.next().unwrap() {
        "X" => Item::Rock,
        "Y" => Item::Paper,
        "Z" => Item::Scissors,
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
        input
            .iter()
            .map(|(x, y)| my_move(x, &map_item_to_outcome(y)))
            .sum()
    }

    fn parse(input: &'a str) -> Self::Input {
        input.split('\n').map(parse_turn).collect()
    }
}
