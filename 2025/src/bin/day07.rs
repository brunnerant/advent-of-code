use std::collections::BTreeSet;

use aoc2025::grid::Grid;

const INPUT: &str = include_str!("../../assets/day07.txt");

#[derive(PartialEq, Eq, Clone, Copy)]
enum Cell {
    Empty,
    Splitter,
    Start,
    Beam,
}
type Input = (Grid<Cell>, usize, usize);

fn parse(input: &str) -> Input {
    let grid = Grid::from_lines(input.lines().map(|l| {
        l.bytes().map(|b| match b {
            b'.' => Cell::Empty,
            b'^' => Cell::Splitter,
            b'S' => Cell::Start,
            _ => panic!("invalid cell"),
        })
    }))
    .unwrap();
    let (x, y) = grid
        .positions()
        .find(|&(x, y)| grid[(x, y)] == Cell::Start)
        .unwrap();
    (grid, x, y)
}

fn part1((grid, x, y): &Input) -> usize {
    let mut grid = grid.clone();
    let mut splits = 0;
    let mut to_process = BTreeSet::new();
    to_process.insert((*x, *y + 1));
    while let Some((x, y)) = to_process.pop_first() {
        if y >= grid.height || x >= grid.width {
            continue;
        }
        match grid[(x, y)] {
            Cell::Empty => {
                to_process.insert((x, y + 1));
                grid[(x, y)] = Cell::Beam;
            }
            Cell::Splitter => {
                splits += 1;
                to_process.insert((x.wrapping_sub(1), y));
                to_process.insert((x + 1, y));
            }
            _ => {}
        }
    }
    splits
}

fn part2((grid, x, y): &Input) -> usize {
    let mut grid = grid.clone();
    let mut num_trajectories = 0;
    let mut trajectories = vec![(*x, *y + 1)];
    while let Some((x, y)) = trajectories.pop() {
        if y >= grid.height || x >= grid.width {
            num_trajectories += 1;
            continue;
        }
        match grid[(x, y)] {
            Cell::Empty | Cell::Beam => {
                trajectories.push((x, y + 1));
                grid[(x, y)] = Cell::Beam;
            }
            Cell::Splitter => {
                trajectories.push((x.wrapping_sub(1), y));
                trajectories.push((x + 1, y));
            }
            _ => {}
        }
    }
    num_trajectories
}

pub fn main() {
    let input = parse(INPUT);
    println!("Day 07 - Part 1: {}", part1(&input));
    println!("Day 07 - Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use crate::{parse, part1, part2};

    #[test]
    fn test_part1() {
        let input = parse(
            ".......S.......\n\
             ...............\n\
             .......^.......\n\
             ...............\n\
             ......^.^......\n\
             ...............\n\
             .....^.^.^.....\n\
             ...............\n\
             ....^.^...^....\n\
             ...............\n\
             ...^.^...^.^...\n\
             ...............\n\
             ..^...^.....^..\n\
             ...............\n\
             .^.^.^.^.^...^.\n\
             ...............",
        );
        assert_eq!(part1(&input), 21);
    }

    #[test]
    fn test_part2() {
        let input = parse(
            ".......S.......\n\
             ...............\n\
             .......^.......\n\
             ...............\n\
             ......^.^......\n\
             ...............\n\
             .....^.^.^.....\n\
             ...............\n\
             ....^.^...^....\n\
             ...............\n\
             ...^.^...^.^...\n\
             ...............\n\
             ..^...^.....^..\n\
             ...............\n\
             .^.^.^.^.^...^.\n\
             ...............",
        );
        assert_eq!(part2(&input), 40);
    }
}
