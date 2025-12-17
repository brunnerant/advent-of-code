use std::cmp::Reverse;

use aoc2025::grid::Grid;
use itertools::Itertools;

const INPUT: &str = include_str!("../../assets/day09.txt");

type Input = Vec<(usize, usize)>;

fn parse(input: &str) -> Input {
    input
        .lines()
        .flat_map(|l| l.split(','))
        .map(|n| n.parse().unwrap())
        .tuples()
        .collect()
}

fn length(from: usize, to: usize) -> usize {
    if to > from {
        to - from + 1
    } else {
        from - to + 1
    }
}

fn area(corner1: (usize, usize), corner2: (usize, usize)) -> usize {
    length(corner1.0, corner2.0) * length(corner1.1, corner2.1)
}

fn part1(input: &Input) -> usize {
    input
        .iter()
        .tuple_combinations()
        .map(|(&a, &b)| area(a, b))
        .max()
        .unwrap()
}

fn compress(coords: impl Iterator<Item = usize>) -> Vec<usize> {
    let mut sorted: Vec<_> = coords.collect();
    sorted.sort();
    sorted.dedup();
    sorted
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Tile {
    Vertical,
    Horizontal,
    Inside,
    Empty,
}

fn part2(input: &Input) -> usize {
    // Compute the areas of the point pairs
    let areas: Vec<_> = (0..input.len())
        .tuple_combinations()
        .map(|(i, j)| (i, j, area(input[i], input[j])))
        .sorted_by_key(|(_, _, a)| Reverse(*a))
        .collect();

    // Compress the coordinates of the point pairs
    let xcomp = compress(input.iter().map(|&(x, _)| x));
    let ycomp = compress(input.iter().map(|&(_, y)| y));
    let points: Vec<_> = input
        .iter()
        .map(|(a, b)| {
            (
                xcomp.binary_search(a).unwrap(),
                ycomp.binary_search(b).unwrap(),
            )
        })
        .collect();

    // Fill the edges
    let mut tiles = Grid::fill_with_elem(xcomp.len(), ycomp.len(), Tile::Empty);
    for i in 0..points.len() {
        let j = (i + 1) % points.len();
        let (xi, yi) = points[i];
        let (xj, yj) = points[j];
        if xi == xj {
            for y in yi.min(yj)..=yi.max(yj) {
                tiles[(xi, y)] = Tile::Vertical;
            }
            tiles[(xi, yi.max(yj))] = Tile::Horizontal; // this is to handle edge cases with adjacent walls
        } else if yi == yj {
            for x in xi.min(xj)..=xi.max(xj) {
                if tiles[(x, yi)] == Tile::Empty {
                    tiles[(x, yi)] = Tile::Horizontal;
                }
            }
        }
    }

    // Ray-cast the inside of the shape
    for y in 0..tiles.height {
        let mut num_walls = 0;
        for x in 0..tiles.width {
            if tiles[(x, y)] == Tile::Vertical {
                num_walls += 1;
            }
            if tiles[(x, y)] == Tile::Empty && num_walls % 2 == 1 {
                tiles[(x, y)] = Tile::Inside;
            }
        }
    }

    // Compute space towards the right for each cell
    let mut space = Grid::fill_with_elem(tiles.width, tiles.height, 0);
    for y in 0..tiles.height {
        let last = (tiles.width - 1, y);
        space[last] = if tiles[last] != Tile::Empty { 1 } else { 0 };
        for x in (0..tiles.width - 1).rev() {
            space[(x, y)] = if tiles[(x, y)] != Tile::Empty {
                space[(x + 1, y)] + 1
            } else {
                0
            };
        }
    }

    // Take the biggest square that fits within the tiles
    areas
        .into_iter()
        .find(|&(i, j, _)| {
            let (xi, yi) = points[i];
            let (xj, yj) = points[j];
            let xmin = xi.min(xj);
            let width = length(xi, xj);
            (yi.min(yj)..=yi.max(yj)).all(|y| space[(xmin, y)] >= width)
        })
        .map_or(0, |(_, _, a)| a)
}

pub fn main() {
    let input = parse(INPUT);
    println!("Day 09 - Part 1: {}", part1(&input));
    println!("Day 09 - Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use crate::{parse, part1, part2};

    #[test]
    fn test_part1() {
        let input = parse("7,1\n11,1\n11,7\n9,7\n9,5\n2,5\n2,3\n7,3");
        assert_eq!(part1(&input), 50);
    }

    #[test]
    fn test_part2() {
        let input = parse("7,1\n11,1\n11,7\n9,7\n9,5\n2,5\n2,3\n7,3");
        assert_eq!(part2(&input), 24);
    }
}
