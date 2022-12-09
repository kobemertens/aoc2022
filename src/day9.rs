use std::collections::HashSet;

use crate::common::Day;

type Pos = (isize, isize);
type Dir = (isize, isize);

type Rope = Vec<Pos>;

#[derive(Debug, Clone, Copy)]
pub struct Instr {
    dir: Dir,
    steps: usize,
}

fn grid_distance(pos1: Pos, pos2: Pos) -> usize {
    std::cmp::max(pos1.0.abs_diff(pos2.0), pos1.1.abs_diff(pos2.1))
}

fn follow_head(head: Pos, tail: Pos) -> Pos {
    let diff = (head.0 - tail.0, head.1 - tail.1);
    let new_diff = match diff {
        (2, 2) => (1, 1),
        (2, -2) => (1, -1),
        (-2, -2) => (-1, -1),
        (-2, 2) => (-1, 1),
        (2, c) => (1, c),
        (-2, c) => (-1, c),
        (r, 2) => (r, 1),
        (r, -2) => (r, -1),
        _ => panic!("Invalid gap: {:?}", diff),
    };
    (tail.0 + new_diff.0, tail.1 + new_diff.1)
}

fn _is_valid_rope(rope: &Rope) -> bool {
    rope.iter()
        .zip(rope.iter().skip(1))
        .all(|(&x, &y)| grid_distance(x, y) < 2)
}

fn move_rope(dir: Dir, rope: &mut Rope, visited: &mut HashSet<Pos>) {
    let new_head = (rope[0].0 + dir.0, rope[0].1 + dir.1);
    rope[0] = new_head;
    for i in 1..rope.len() {
        if grid_distance(rope[i], rope[i - 1]) > 1 {
            rope[i] = follow_head(rope[i - 1], rope[i]);
        } else {
            break;
        }
    }
    visited.insert(*rope.last().unwrap());
}

fn run_instr(instr: &Instr, rope: &mut Rope, visited: &mut HashSet<Pos>) {
    for _ in 0..instr.steps {
        move_rope(instr.dir, rope, visited);
    }
}

fn part(input: &Vec<Instr>, rope_len: usize) -> usize {
    let mut rope = vec![(0, 0); rope_len];
    let mut visited: HashSet<Pos> = HashSet::new();
    for instr in input {
        run_instr(instr, &mut rope, &mut visited);
    }
    visited.len()
}

pub struct Day9;

impl<'a> Day<'a> for Day9 {
    type Input = Vec<Instr>;
    type Output = usize;

    fn day_number() -> usize {
        9
    }

    fn part1(input: &Self::Input) -> Self::Output {
        part(input, 2)
    }

    fn part2(input: &Self::Input) -> Self::Output {
        part(input, 10)
    }

    fn parse(input: &'a str) -> Self::Input {
        input
            .lines()
            .map(|x| {
                let mut parts = x.split(' ');
                let dir = match parts.next().unwrap() {
                    "U" => (-1, 0),
                    "D" => (1, 0),
                    "R" => (0, 1),
                    "L" => (0, -1),
                    c => panic!("Unexpected instruction character: {}", c),
                };
                let steps = parts.next().unwrap().parse::<usize>().unwrap();
                Instr { dir, steps }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::day9::{follow_head, grid_distance, Pos, _is_valid_rope};

    use super::move_rope;

    #[test]
    fn dist_test() {
        let pos1 = (0, 0);
        let pos2 = (1, 1);
        let pos3 = (2, 2);
        assert_eq!(grid_distance(pos1, pos2), 1);
        assert_eq!(grid_distance(pos1, pos3), 2);
        assert_eq!(grid_distance(pos1, pos1), 0);
        assert_eq!(grid_distance((-8, 3), (-10, 1)), 2);
    }

    #[test]
    fn move_rope_test() {
        let mut rope = vec![(0, 0); 3];
        let mut set = HashSet::new();
        move_rope((0, 1), &mut rope, &mut set);
        assert_eq!(rope, vec![(0, 1), (0, 0), (0, 0)]);
        move_rope((0, 1), &mut rope, &mut set);
        assert_eq!(rope, vec![(0, 2), (0, 1), (0, 0)]);
        move_rope((0, 1), &mut rope, &mut set);
        assert_eq!(rope, vec![(0, 3), (0, 2), (0, 1)]);
        move_rope((-1, 0), &mut rope, &mut set);
        assert_eq!(rope, vec![(-1, 3), (0, 2), (0, 1)]);
        move_rope((0, 1), &mut rope, &mut set);
        assert_eq!(rope, vec![(-1, 4), (-1, 3), (-1, 2)]);
        assert!(_is_valid_rope(&rope));
        let set_ref: HashSet<Pos> = vec![(0, 0), (0, 1), (-1, 2)].iter().copied().collect();
        assert_eq!(set, set_ref);
    }

    #[test]
    fn follow_head_test() {
        assert_eq!(follow_head((1, 2), (0, 0)), (1, 1));
        assert_eq!(follow_head((2, 4), (1, 2)), (2, 3));
        assert_eq!(follow_head((-1, 1), (1, 2)), (0, 1));
    }
}
