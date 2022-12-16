use std::{collections::HashSet, thread::current};

use crate::common::Day;

pub struct Day15;

pub type Pos = (isize, isize);
pub type Coverage = HashSet<Pos>;
pub type LineSeg = (Pos, Pos);

#[derive(Debug)]
pub struct Sensor {
    pos: Pos,
    beacon: Pos,
}

impl Sensor {
    fn get_coverage(&self) -> HashSet<Pos> {
        let mut coverage = HashSet::new();
        let dist = manhatthan(self.pos, self.beacon);
        for inner_dist in 0..=isize::try_from(dist).unwrap() {
            for i in 0..=inner_dist {
                coverage.insert((self.pos.0 + inner_dist - i, self.pos.1 + i));
                coverage.insert((self.pos.0 - inner_dist + i, self.pos.1 + i));
                coverage.insert((self.pos.0 - inner_dist + i, self.pos.1 - i));
                coverage.insert((self.pos.0 + inner_dist - i, self.pos.1 - i));
            }
        }
        assert!(coverage.iter().all(|&x| manhatthan(x, self.pos) <= dist));
        coverage
    }

    fn get_intersection(&self, y_c: isize) -> Option<LineSeg> {
        let r: isize = isize::try_from(manhatthan(self.pos, self.beacon)).unwrap();
        if self.pos.1 + r >= y_c && self.pos.1 - r <= y_c {
            Some((
                (
                    -r + isize::try_from(y_c.abs_diff(self.pos.1)).unwrap() + self.pos.0,
                    y_c,
                ),
                (
                    r - isize::try_from(y_c.abs_diff(self.pos.1)).unwrap() + self.pos.0,
                    y_c,
                ),
            ))
        } else {
            None
        }
    }

    fn can_sense_pos(&self, pos: Pos) -> bool {
        let radius = manhatthan(self.pos, self.beacon);
        let dist = manhatthan(self.pos, pos);
        dist <= radius
    }
}

fn manhatthan(pos1: Pos, pos2: Pos) -> usize {
    pos1.0.abs_diff(pos2.0) + pos1.1.abs_diff(pos2.1)
}

fn parse_pos(inp: &str) -> Pos {
    let mut coords = inp.split(", ");
    (
        coords
            .next()
            .unwrap()
            .split("=")
            .skip(1)
            .next()
            .unwrap()
            .parse::<isize>()
            .unwrap(),
        coords
            .next()
            .unwrap()
            .split("=")
            .skip(1)
            .next()
            .unwrap()
            .parse::<isize>()
            .unwrap(),
    )
}

fn coverage(sensors: &Vec<Sensor>) -> Coverage {
    let mut coverage = HashSet::new();
    for sensor in sensors {
        coverage = coverage.union(&sensor.get_coverage()).copied().collect();
    }
    coverage
}

fn simplify_linesegs(lines: &Vec<LineSeg>) -> Vec<LineSeg> {
    let mut out = vec![];
    let mut sorted = lines.clone();
    sorted.sort_by(|x, y| x.0 .0.cmp(&y.0 .0));
    let mut current_seg = sorted[0];
    for line_seg in sorted.iter().skip(1) {
        if line_seg.0 .0 - 1 <= current_seg.1 .0 {
            if line_seg.1 .0 > current_seg.1 .0 {
                current_seg = (current_seg.0, line_seg.1);
            }
        } else {
            out.push(current_seg);
            current_seg = *line_seg;
        }
    }
    out.push(current_seg);

    out
}

impl<'a> Day<'a> for Day15 {
    type Input = Vec<Sensor>;
    type Output = usize;

    fn day_number() -> usize {
        15
    }

    fn part1(input: &Self::Input) -> Self::Output {
        let y_c = 2000000;
        let beacons_on_line: HashSet<Pos> = input
            .iter()
            .map(|sensor| sensor.beacon)
            .filter(|(_, y)| *y == y_c)
            .collect();
        println!("beacons{:?}", beacons_on_line);
        let intersections: Vec<LineSeg> = input
            .iter()
            .filter_map(|x| x.get_intersection(y_c))
            .collect();
        let simplified = simplify_linesegs(&intersections);
        simplified
            .iter()
            .map(|(x, y)| {
                let dist = manhatthan(*x, *y);
                println!("distance{:?}", dist);
                dist
            })
            .sum::<usize>()
            + 1
            - beacons_on_line.len()
    }

    fn part2(input: &Self::Input) -> Self::Output {
        for y in 0..=4000000 {
            let intersections: Vec<LineSeg> = input
                .iter()
                .filter_map(|x| x.get_intersection(y))
                .collect();
            let simplified = simplify_linesegs(&intersections);
            if simplified.len() > 1 {
                // Some DIY required cuz I aint got time for this
                println!("simpl{:?}", simplified);
            }
        }
        0
    }

    fn parse(input: &'a str) -> Self::Input {
        input
            .lines()
            .map(|line| {
                let stripped = line.strip_prefix("Sensor at ").unwrap();
                let colon_idx = stripped.find(":").unwrap();
                let sensor_pos = parse_pos(&stripped[0..colon_idx]);
                let beacon_pos = parse_pos(
                    stripped[colon_idx..stripped.len()]
                        .strip_prefix(": closest beacon is at ")
                        .unwrap(),
                );
                Sensor {
                    pos: sensor_pos,
                    beacon: beacon_pos,
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::day15::Sensor;

    use super::simplify_linesegs;

    #[test]
    fn test_coverage() {
        let sensor = Sensor {
            pos: (0, 0),
            beacon: (2, 0),
        };
        assert_eq!(
            sensor.get_coverage(),
            vec![
                (0, 0),
                (1, 0),
                (0, 1),
                (-1, 0),
                (0, -1),
                (2, 0),
                (0, 2),
                (-2, 0),
                (0, -2),
                (1, 1),
                (-1, 1),
                (1, -1),
                (-1, -1),
            ]
            .iter()
            .copied()
            .collect()
        );
    }

    #[test]
    fn test_intersection() {
        let sensor = Sensor {
            pos: (0, 0),
            beacon: (2, 0),
        };
        assert_eq!(sensor.get_intersection(0), Some(((-2, 0), (2, 0))));
        assert_eq!(sensor.get_intersection(1), Some(((-1, 1), (1, 1))));
        assert_eq!(sensor.get_intersection(2), Some(((0, 2), (0, 2))));
        assert_eq!(sensor.get_intersection(3), None);
    }

    #[test]
    fn test_simplify_linesegs() {
        let line_segs = vec![
            ((0, 0), (10, 0)),
            ((5, 0), (8, 0)),
            ((6, 0), (14, 0)),
            ((20, 0), (30, 0)),
        ];
        assert_eq!(
            simplify_linesegs(&line_segs),
            vec![((0, 0), (14, 0)), ((20, 0), (30, 0))]
        );
        let line_segs2 = vec![
            ((-2, 10), (2, 10)),
            ((2, 10), (14, 10)),
            ((2, 10), (2, 10)),
            ((12, 10), (12, 10)),
            ((14, 10), (18, 10)),
            ((16, 10), (24, 10)),
        ];
        assert_eq!(simplify_linesegs(&line_segs2), vec![((-2, 10), (24, 10))]);
    }
}
