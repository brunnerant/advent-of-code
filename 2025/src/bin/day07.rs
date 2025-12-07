use std::collections::{BTreeSet, HashMap};

use aoc2025::{grid::Grid, topo::Topo};

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
    // Gather the topology of the particles paths so that we can compute the final answer in topological order
    let mut grid = grid.clone();
    let mut topo = Topo::new();
    let mut to_process = BTreeSet::new();
    to_process.insert((*x, *y + 1));
    while let Some((x, y)) = to_process.pop_first() {
        if y >= grid.height || x >= grid.width {
            continue;
        }
        match grid[(x, y)] {
            Cell::Empty => {
                topo.add_edge((x, y), (x, y + 1));
                to_process.insert((x, y + 1));
                grid[(x, y)] = Cell::Beam;
            }
            Cell::Splitter => {
                topo.add_edge((x, y), (x.wrapping_sub(1), y));
                topo.add_edge((x, y), (x + 1, y));
                to_process.insert((x.wrapping_sub(1), y));
                to_process.insert((x + 1, y));
            }
            _ => {}
        }
    }

    let mut cell_count = HashMap::new();
    let mut total_count = 0;
    cell_count.insert((*x, *y + 1), 1);
    for (x, y) in topo.sort_flat().unwrap() {
        let c = cell_count[&(x, y)];
        if y >= grid.height || x >= grid.width {
            // add exiting particles to the total count
            total_count += c;
            continue;
        }
        match grid[(x, y)] {
            Cell::Beam => {
                *cell_count.entry((x, y + 1)).or_default() += c;
            }
            Cell::Splitter => {
                *cell_count.entry((x.wrapping_sub(1), y)).or_default() += c;
                *cell_count.entry((x + 1, y)).or_default() += c;
            }
            _ => {}
        }
    }
    total_count
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
