use std::collections::HashSet;

use crate::common::Day;

pub struct Day14;

pub type Pos = (usize, usize);

#[derive(Clone, Debug)]
pub struct Cave {
    blocks: HashSet<Pos>,
    lowest: usize,
}

impl Cave {
    fn drop_sand_unit(&mut self) -> bool {
        let mut sand_pos = (0, 500);
        loop {
            if sand_pos.0 > self.lowest {
                return true;
            }
            sand_pos = (sand_pos.0 + 1, sand_pos.1);
            if self.blocks.contains(&sand_pos) {
                sand_pos = (sand_pos.0, sand_pos.1 - 1);
                if self.blocks.contains(&sand_pos) {
                    sand_pos = (sand_pos.0, sand_pos.1 + 2);
                    if self.blocks.contains(&sand_pos) {
                        self.blocks.insert((sand_pos.0 - 1, sand_pos.1 - 1));
                        return false;
                    }
                }
            }
        }
    }

    fn drop_sand_part2(&mut self) -> bool {
        if self.blocks.contains(&(0, 500)) {
            return false;
        }

        let mut sand_pos = (0, 500);
        loop {
            sand_pos = (sand_pos.0 + 1, sand_pos.1);
            if sand_pos.0 == self.lowest + 2 {
                let insert_pos = (sand_pos.0 - 1, sand_pos.1);
                self.blocks.insert(insert_pos);
                return true;
            }
            if self.blocks.contains(&sand_pos) {
                sand_pos = (sand_pos.0, sand_pos.1 - 1);
                if self.blocks.contains(&sand_pos) {
                    sand_pos = (sand_pos.0, sand_pos.1 + 2);
                    if self.blocks.contains(&sand_pos) || sand_pos.0 == self.lowest + 2 {
                        let insert_pos = (sand_pos.0 - 1, sand_pos.1 - 1);
                        self.blocks.insert(insert_pos);
                        return true;
                    }
                }
            }
        }
    }
}

impl<'a> Day<'a> for Day14 {
    type Input = Cave;
    type Output = usize;

    fn day_number() -> usize {
        14
    }

    fn part1(input: &Self::Input) -> Self::Output {
        let mut sand_count = 0;
        let cave = &mut input.clone();
        while !cave.drop_sand_unit() {
            sand_count += 1;
        }
        sand_count
    }

    fn part2(input: &Self::Input) -> Self::Output {
        let mut sand_count = 0;
        let cave = &mut input.clone();
        while cave.drop_sand_part2() {
            sand_count += 1;
        }
        sand_count
    }

    fn parse(input: &'a str) -> Self::Input {
        let blocks: HashSet<Pos> = input
            .lines()
            .flat_map(|line| {
                let points = line.split(" -> ").map(|point_str| {
                    let mut x = point_str.split(",");
                    let yx = (
                        x.next().unwrap().parse::<usize>().unwrap(),
                        x.next().unwrap().parse::<usize>().unwrap(),
                    );
                    (yx.1, yx.0)
                });
                points.clone().zip(points.clone().skip(1)).flat_map(
                    |((start_r, start_c), (end_r, end_c))| {
                        if start_r == end_r {
                            let out: Vec<Pos> = (std::cmp::min(start_c, end_c)
                                ..=std::cmp::max(start_c, end_c))
                                .map(|c| (start_r, c))
                                .collect();
                            return out;
                        } else {
                            let out: Vec<Pos> = (std::cmp::min(start_r, end_r)
                                ..=std::cmp::max(start_r, end_r))
                                .map(|r| (r, start_c))
                                .collect();
                            return out;
                        }
                    },
                )
            })
            .collect();

        let lowest = blocks.iter().map(|&(r, c)| r).max().unwrap();

        Cave { blocks, lowest }
    }
}
