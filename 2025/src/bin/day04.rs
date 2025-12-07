use std::collections::BTreeSet;

use aoc2025::grid::Grid;
use itertools::Itertools;

const INPUT: &str = include_str!("../../assets/day04.txt");

type Input = Grid<bool>;

fn parse(input: &str) -> Input {
    Grid::from_lines(input.lines().map(|l| {
        l.chars().map(|c| match c {
            '@' => true,
            '.' => false,
            _ => panic!("invalid grid character"),
        })
    }))
    .unwrap()
}

fn part1(input: &Input) -> usize {
    let mut result = 0;
    for (x, y) in input.positions() {
        if !input[(x, y)] {
            continue;
        }
        let num_adjacent = input
            .adjacent_cells((x, y))
            .filter(|&(i, j)| input[(i, j)])
            .count();
        if num_adjacent < 4 {
            result += 1;
        }
    }
    result
}

fn part2(input: &Input) -> usize {
    let mut input = input.clone();
    let mut result = 0;
    let mut to_check =
        BTreeSet::<(usize, usize)>::from_iter((0..input.width).cartesian_product(0..input.height));
    while let Some((x, y)) = to_check.pop_first() {
        if !input[(x, y)] {
            continue;
        }
        let num_adjacent = input
            .adjacent_cells((x, y))
            .filter(|&(i, j)| input[(i, j)])
            .count();
        if num_adjacent < 4 {
            result += 1;
            to_check.extend(input.adjacent_cells((x, y)));
            input[(x, y)] = false;
        }
    }
    result
}

pub fn main() {
    let input = parse(INPUT);
    println!("Day 04 - Part 1: {}", part1(&input));
    println!("Day 04 - Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use crate::{parse, part1, part2};

    #[test]
    fn test_part1() {
        let input = parse(
            "..@@.@@@@.\n\
             @@@.@.@.@@\n\
             @@@@@.@.@@\n\
             @.@@@@..@.\n\
             @@.@@@@.@@\n\
             .@@@@@@@.@\n\
             .@.@.@.@@@\n\
             @.@@@.@@@@\n\
             .@@@@@@@@.\n\
             @.@.@@@.@.",
        );
        assert_eq!(part1(&input), 13);
    }

    #[test]
    fn test_part2() {
        let input = parse(
            "..@@.@@@@.\n\
             @@@.@.@.@@\n\
             @@@@@.@.@@\n\
             @.@@@@..@.\n\
             @@.@@@@.@@\n\
             .@@@@@@@.@\n\
             .@.@.@.@@@\n\
             @.@@@.@@@@\n\
             .@@@@@@@@.\n\
             @.@.@@@.@.",
        );
        assert_eq!(part2(&input), 43);
    }
}
