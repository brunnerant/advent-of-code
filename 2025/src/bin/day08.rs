use std::cmp::Reverse;

use aoc2025::union_find::{UnionFind, connected_components};
use itertools::Itertools;
use ordered_float::OrderedFloat;

const INPUT: &str = include_str!("../../assets/day08.txt");

#[derive(Debug)]
struct Point {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Point {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    pub fn distance(&self, other: &Point) -> f32 {
        let dx = (self.x - other.x) as f32;
        let dy = (self.y - other.y) as f32;
        let dz = (self.z - other.z) as f32;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

type Input = Vec<Point>;

fn parse(input: &str) -> Input {
    input
        .lines()
        .flat_map(|l| l.split(','))
        .map(|x| x.parse::<i32>().unwrap())
        .tuples()
        .map(|(x, y, z)| Point::new(x, y, z))
        .collect()
}

fn part1(points: &Input, n: usize) -> usize {
    let mut edges: Vec<_> = (0..points.len()).tuple_combinations().collect();
    edges.sort_by_key(|&(a, b)| OrderedFloat(points[a].distance(&points[b])));

    let mut cc = connected_components(points.len(), edges.into_iter().take(n));
    cc.sort_by_key(|c| Reverse(c.len()));
    cc.into_iter().take(3).map(|c| c.len()).product()
}

fn part2(points: &Input) -> usize {
    let mut edges: Vec<_> = (0..points.len()).tuple_combinations().collect();
    edges.sort_by_key(|&(a, b)| OrderedFloat(points[a].distance(&points[b])));
    let mut uf = UnionFind::new(points.len());
    for (a, b) in edges {
        uf.union(a, b);
        if uf.group_size(a) == points.len() {
            return points[a].x as usize * points[b].x as usize;
        }
    }
    0
}

pub fn main() {
    let input = parse(INPUT);
    println!("Day 08 - Part 1: {}", part1(&input, 1000));
    println!("Day 08 - Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use crate::{parse, part1, part2};

    #[test]
    fn test_part1() {
        let input = parse(
            "162,817,812\n57,618,57\n906,360,560\n592,479,940\n352,342,300\n466,668,158\n542,29,236\n\
             431,825,988\n739,650,466\n52,470,668\n216,146,977\n819,987,18\n117,168,530\n805,96,715\n\
             346,949,466\n970,615,88\n941,993,340\n862,61,35\n984,92,344\n425,690,689",
        );
        assert_eq!(part1(&input, 10), 40);
    }

    #[test]
    fn test_part2() {
        let input = parse(
            "162,817,812\n57,618,57\n906,360,560\n592,479,940\n352,342,300\n466,668,158\n542,29,236\n\
             431,825,988\n739,650,466\n52,470,668\n216,146,977\n819,987,18\n117,168,530\n805,96,715\n\
             346,949,466\n970,615,88\n941,993,340\n862,61,35\n984,92,344\n425,690,689",
        );
        assert_eq!(part2(&input), 25272);
    }
}
