use crate::common::Day;

pub struct Day12;

type Grid = Vec<Vec<usize>>;
type Pos = (usize, usize);

#[derive(Clone, Copy, Debug)]
enum LookupEntry {
    Calculcated(usize),
    NotCalculcated,
    UnReachable,
}

fn possible_nbs(grid: &Grid, pos: Pos) -> Vec<Pos> {
    let nbs = [
        (pos.0 + 1, pos.1),
        (pos.0, pos.1 + 1),
        (pos.0.wrapping_sub(1), pos.1),
        (pos.0, pos.1.wrapping_sub(1)),
    ];
    nbs.iter()
        .copied()
        .filter(|(r, c)| r < &grid.len() && c < &grid[0].len())
        .filter(|(r, c)| {
            let pos_height = grid[pos.0][pos.1];
            let dest_height = grid[*r][*c];
            pos_height >= dest_height || pos_height.abs_diff(dest_height) == 1
        })
        .collect()
}

fn shortest_path(
    grid: &mut Grid,
    start: Pos,
    dest: Pos,
    lookup: &mut Vec<Vec<LookupEntry>>,
) -> Option<usize> {
    // println!("{:?}", lookup);
    if start == dest {
        return Some(0);
    }

    if let LookupEntry::Calculcated(x) = lookup[start.0][start.1] {
        return Some(x);
    } else if let LookupEntry::UnReachable = lookup[start.0][start.1] {
        return None;
    }

    // assert!(grid[start.0][start.1] != usize::MAX);
    let nbs = possible_nbs(grid, start);
    // println!("{:?}", nbs);
    // println!("Position: {:?}, neighbours: {:?}", start, nbs);

    let temp = grid[start.0][start.1];
    grid[start.0][start.1] = usize::MAX; // make sure we dont go back

    let mut shortest = None;
    for nb in nbs {
        let possible = shortest_path(grid, nb, dest, lookup);
        // Might be possible to do this cleaner
        if let Some(possible) = possible {
            match shortest {
                None => shortest = Some(possible),
                Some(shortest_val) => {
                    if possible < shortest_val {
                        shortest = Some(possible);
                    }
                }
            }
        }
    }
    println!("found solution for {:?}, {:?}", start, shortest);
    grid[start.0][start.1] = temp;
    lookup[start.0][start.1] = match shortest {
        Some(x) => LookupEntry::Calculcated(x + 1),
        None => LookupEntry::NotCalculcated,
    };
    shortest.map(|x| x + 1)
}

fn find_shortest_path(grid: &mut Grid) -> usize {
    let mut lookup = vec![vec![LookupEntry::NotCalculcated; grid[0].len()]; grid.len()];
    let shortest = shortest_path(grid, (0, 0), (20, 72), &mut lookup).unwrap();
    // lookup.iter().for_each(
        // |x| println!("{:?}", x)
    // );
    shortest
}

impl<'a> Day<'a> for Day12 {
    type Input = Grid;
    type Output = usize;

    fn day_number() -> usize {
        12
    }

    fn part1(input: &Self::Input) -> Self::Output {
        // println!("{:?}", input);
        find_shortest_path(&mut input.clone())
    }

    fn part2(input: &Self::Input) -> Self::Output {
        0
    }

    fn parse(input: &'a str) -> Self::Input {
        let grid: Self::Input = input
            .lines()
            .map(|x| {
                x.chars()
                    .map(|y| {
                        match y {
                            'S' => usize::MAX,
                            'E' => 'z' as usize - 'a' as usize,
                            y => y as usize - 'a' as usize, // lowest is 1
                        }
                    })
                    .collect()
            })
            .collect();
        grid
    }
}
