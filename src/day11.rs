use core::panic;

use crate::common::Day;

pub struct Day11;

pub type Item = usize;

#[derive(Clone, Debug)]
pub enum Operation {
    ProdConst(usize),
    AddConst(usize),
    ProdOld,
    AddOld,
}

#[derive(Clone, Debug)]
pub struct Test {
    n: usize,
    if_true: usize,
    if_false: usize,
}

#[derive(Clone, Debug)]
pub struct Monkey {
    id: usize,
    items: Vec<Item>,
    op: Operation,
    test: Test,
    insp_count: usize,
}
fn do_round2(monkeys: &mut Vec<Monkey>) {
    for i in 0..monkeys.len() {
        println!("Monkey {}:", i);
        let mut to_add: Vec<(usize, Item)> = vec![];
        let monkey = &mut monkeys[i];
        for item in &monkey.items {
            println!("  Monkey inspects item with worry level of {}", item);
            let mut worry_lvl: usize = match monkey.op {
                Operation::AddConst(x) => item + x,
                Operation::ProdConst(x) => item * x,
                Operation::AddOld => item + item,
                Operation::ProdOld => item * item,
            };
            worry_lvl %= 9699690;
            println!("    changes worry level to {}", worry_lvl);

            if worry_lvl % monkey.test.n == 0 {
                println!("    Current worry level is divisible by {}.", monkey.test.n);
                to_add.push((monkey.test.if_true, worry_lvl));
            } else {
                println!(
                    "    Current worry level is not divisible by {}.",
                    monkey.test.n
                );
                to_add.push((monkey.test.if_false, worry_lvl));
            }
        }
        monkey.insp_count += monkey.items.len();
        monkey.items = vec![];
        to_add
            .iter()
            .copied()
            .for_each(|(idx, item)| monkeys[idx].items.push(item));
    }
}

fn do_round(monkeys: &mut Vec<Monkey>) {
    for i in 0..monkeys.len() {
        println!("Monkey {}:", i);
        let mut to_add: Vec<(usize, Item)> = vec![];
        let monkey = &mut monkeys[i];
        for item in &monkey.items {
            println!("  Monkey inspects item with worry level of {}", item);
            let mut worry_lvl: usize = match monkey.op {
                Operation::AddConst(x) => item + x,
                Operation::ProdConst(x) => item * x,
                Operation::AddOld => item + item,
                Operation::ProdOld => item * item,
            };
            worry_lvl /= 3;
            // println!("    changes worry level to {}", worry_lvl);

            if worry_lvl % monkey.test.n == 0 {
                // println!("    Current worry level is divisible by {}.", monkey.test.n);
                to_add.push((monkey.test.if_true, worry_lvl));
            } else {
                // println!(
                    // "    Current worry level is not divisible by {}.",
                    // monkey.test.n
                // );
                to_add.push((monkey.test.if_false, worry_lvl));
            }
        }
        monkey.insp_count += monkey.items.len();
        monkey.items = vec![];
        to_add
            .iter()
            .copied()
            .for_each(|(idx, item)| monkeys[idx].items.push(item));
    }
}

impl<'a> Day<'a> for Day11 {
    type Input = Vec<Monkey>;
    type Output = usize;

    fn day_number() -> usize {
        11
    }

    fn part1(input: &Self::Input) -> Self::Output {
        let monkeys = &mut input.clone();
        for _ in 0..20 {
            do_round(monkeys);
        }
        let mut counts: Vec<usize> = monkeys.iter().map(|monkey| monkey.insp_count).collect();
        counts.sort_by(|x, y| y.cmp(x));
        counts[0] * counts[1]
    }

    fn part2(input: &Self::Input) -> Self::Output {
        let monkeys = &mut input.clone();
        for _ in 0..10000 {
            do_round2(monkeys);
        }
        let mut counts: Vec<usize> = monkeys.iter().map(|monkey| monkey.insp_count).collect();
        counts.sort_by(|x, y| y.cmp(x));
        counts[0] * counts[1]
    }

    fn parse(input: &'a str) -> Self::Input {
        input
            .split("\n\n")
            .enumerate()
            .map(|(id, part)| {
                let mut lines = part.lines().skip(1);
                let items = lines
                    .next()
                    .unwrap()
                    .strip_prefix("  Starting items: ")
                    .unwrap()
                    .split(", ")
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect();
                let mut operation_it = lines
                    .next()
                    .unwrap()
                    .strip_prefix("  Operation: new = old ")
                    .unwrap()
                    .split(" ");

                let operation_operator = operation_it.next().unwrap();
                println!("{}", operation_operator);
                let operation = match operation_it.next().unwrap() {
                    "old" => match operation_operator {
                        "+" => Operation::AddOld,
                        "*" => Operation::ProdOld,
                        _ => panic!("Invalid input"),
                    },
                    n_str => {
                        let n = n_str.parse::<usize>().unwrap();
                        match operation_operator {
                            "+" => Operation::AddConst(n),
                            "*" => Operation::ProdConst(n),
                            _ => panic!("Invalid input"),
                        }
                    }
                };

                let test_n = lines
                    .next()
                    .unwrap()
                    .strip_prefix("  Test: divisible by ")
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();
                let test_if_true = lines
                    .next()
                    .unwrap()
                    .strip_prefix("    If true: throw to monkey ")
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();
                let test_if_false = lines
                    .next()
                    .unwrap()
                    .strip_prefix("    If false: throw to monkey ")
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();

                Monkey {
                    id,
                    items,
                    op: operation,
                    test: Test {
                        n: test_n,
                        if_true: test_if_true,
                        if_false: test_if_false,
                    },
                    insp_count: 0,
                }
            })
            .collect()
    }
}
