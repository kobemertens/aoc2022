use std::vec;

use crate::common::Day;

pub struct Day8;

type Grid = Vec<Vec<usize>>;
type Visibilities = Vec<Vec<bool>>;
type Dir = (i32, i32);
type Pos = (usize, usize);

fn look_into_direction(grid: &Grid, vis: &mut Visibilities, start_pos: Pos, dir: Dir) {
    let mut highest: Option<usize> = None;
    let (mut row, mut col) = start_pos;
    let height = grid.len();
    let width = grid[0].len();
    while row < height && col < width {
        if highest == None || grid[row][col] > highest.unwrap() {
            vis[row][col] = true;
            highest = Some(grid[row][col]);
        }

        let new_row = (row as i32 + dir.0);
        let new_col = (col as i32 + dir.1);

        if new_row >= 0 && new_col >= 0 {
            row = new_row as usize;
            col = new_col as usize;
        } else {
            break;
        }
    }
}

fn scenic_score(grid: &Grid, tree_pos: Pos) -> usize {
    let dirs: Vec<(isize, isize)> = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut scores = vec![];
    let tree_height = grid[tree_pos.0][tree_pos.1];
    let height = grid.len();
    let width = grid[0].len();

    for (dr, dc) in dirs {
        let mut dist = 1;
        let mut dir_score = 0;
        loop {
            let nr: isize = (tree_pos.0 as isize) + dr * dist;
            let nc: isize = (tree_pos.1 as isize) + dc * dist;
            if nr < 0
                || nr >= height.try_into().unwrap()
                || nc < 0
                || nc >= width.try_into().unwrap()
            {
                scores.push(dir_score);
                break;
            }
            let other_tree_height = grid[nr as usize][nc as usize];

            dir_score += 1;
            dist += 1;
            if other_tree_height >= tree_height {
                scores.push(dir_score);
                break;
            }
        }
    }

    scores.iter().product()
}

impl<'a> Day<'a> for Day8 {
    type Input = Grid;
    type Output = usize;

    fn day_number() -> usize {
        8
    }

    fn part1(input: &Self::Input) -> Self::Output {
        let grid = input;
        let mut vis = vec![vec![false; grid[0].len()]; grid.len()];
        let width = grid[0].len();
        let height = grid.len();
        // loopt over de rows
        for i in 0..input.len() {
            look_into_direction(&grid, &mut vis, (i, 0), (0, 1));
            look_into_direction(&grid, &mut vis, (i, width - 1), (0, -1));
        }
        // loopt over de cols
        for i in 0..input[0].len() {
            look_into_direction(&grid, &mut vis, (0, i), (1, 0));
            look_into_direction(&grid, &mut vis, (height - 1, i), (-1, 0));
        }

        vis.iter().map(|x| x.iter().filter(|&&x| x).count()).sum()
    }

    fn part2(input: &Self::Input) -> Self::Output {
        let grid = input;
        let mut max = 0;
        for r in 1..grid.len() - 1 {
            for c in 1..grid[0].len() - 1 {
                let score = scenic_score(&grid, (r, c));
                if score > max {
                    max = score;
                }
            }
        }

        max
    }

    fn parse(input: &'a str) -> Self::Input {
        input
            .lines()
            .map(|x| {
                x.chars()
                    .map(|x| String::from(x).parse::<usize>().unwrap())
                    .collect()
            })
            .collect()
    }
}
