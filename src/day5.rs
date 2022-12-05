use crate::common::Day;

pub struct Day5;

#[derive(Debug, Clone, Copy)]
pub struct Move {
    from: usize,
    to: usize,
    amount: usize,
}

type Stack = Vec<char>;

fn parse_stacks(stacks_string: &str) -> Vec<Stack> {
    let mut stacks: Vec<Stack> = vec![vec![]; 9];
    let lines = stacks_string.lines();
    for line in lines {
        if line.chars().nth(1).unwrap() == '1' {
            break;
        }
        for (c, i) in line.chars().skip(1).step_by(4).zip(0..) {
            if c == ' ' {
                continue;
            } else {
                stacks[i].insert(0, c);
            }
        }
    }
    stacks
}

fn parse_moves(moves_string: &str) -> Vec<Move> {
    moves_string[0..moves_string.len() - 1]
        .split('\n')
        .map(|x| {
            let mut words = x.split(' ');
            Move {
                amount: words.nth(1).unwrap().parse::<usize>().unwrap(),
                from: words.nth(1).unwrap().parse::<usize>().unwrap() - 1,
                to: words.nth(1).unwrap().parse::<usize>().unwrap() - 1,
            }
        })
        .collect()
}

fn exec_move(stacks: &mut Vec<Stack>, move_instr: &Move) {
    let mut temp: Stack = vec![];
    for _ in 0..move_instr.amount {
        let el = stacks[move_instr.from].pop().unwrap();
        temp.push(el);
    }

    for i in 0..move_instr.amount {
        stacks[move_instr.to].push(temp[i]);
    }
}

fn exec_move_(stacks: &mut Vec<Stack>, move_instr: &Move) {
    let mut temp: Stack = vec![];
    for _ in 0..move_instr.amount {
        let el = stacks[move_instr.from].pop().unwrap();
        temp.push(el);
    }

    for _ in 0..move_instr.amount {
        stacks[move_instr.to].push(temp.pop().unwrap());
    }
}

impl<'a> Day<'a> for Day5 {
    type Input = (Vec<Stack>, Vec<Move>);
    type Output = String;

    fn day_number() -> usize {
        5
    }

    fn part1(input: &Self::Input) -> Self::Output {
        let mut stacks = input.0.clone();
        for move_instr in &input.1 {
            exec_move(&mut stacks, &move_instr);
        }
        let mut res = String::from("");
        for stack in stacks {
            res.push(*stack.last().unwrap());
        }
        res
    }

    fn part2(input: &Self::Input) -> Self::Output {
        let mut stacks = input.0.clone();
        for move_instr in &input.1 {
            exec_move_(&mut stacks, &move_instr);
        }
        let mut res = String::from("");
        for stack in stacks {
            res.push(*stack.last().unwrap());
        }
        res
    }

    fn parse(input: &'a str) -> Self::Input {
        let mut parts = input.split("\n\n");
        let init_stack_string = parts.next().unwrap();
        let move_strings = parts.next().unwrap();
        (parse_stacks(init_stack_string), parse_moves(move_strings))
    }
}
