use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

use regex::Regex;

const INPUT: &str = include_str!("../../assets/day10.txt");

#[derive(Debug)]
struct Problem {
    pub target: usize,
    pub switches: Vec<usize>,
}

type Input = Vec<Problem>;

fn encode_indices(indices: impl Iterator<Item = usize>) -> usize {
    let mut result = 0;
    for i in indices {
        result |= 1 << i
    }
    result
}

fn encode_bits(bits: impl Iterator<Item = bool>) -> usize {
    encode_indices(bits.enumerate().filter_map(|(i, b)| b.then_some(i)))
}

fn parse(input: &str) -> Input {
    let target_regex = Regex::new(r"\[(.*)\]").unwrap();
    let switch_regex = Regex::new(r"\(((?:\d|,)*)\)").unwrap();
    let mut result = Vec::new();
    for line in input.lines() {
        let target = encode_bits(
            target_regex.captures(line).unwrap()[1]
                .chars()
                .map(|c| c == '#'),
        );
        let switches = switch_regex
            .captures_iter(line)
            .map(|c| encode_indices(c[1].split(',').map(|n| n.parse().unwrap())))
            .collect();
        result.push(Problem { target, switches });
    }
    result
}

#[derive(PartialEq, Eq, Debug)]
struct State {
    lights: usize,
    num_switches: usize,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.num_switches.cmp(&other.num_switches)
    }
}

fn num_switches(problem: &Problem) -> Option<usize> {
    let mut queue = BinaryHeap::new();
    let mut visited = HashSet::new();
    queue.push(Reverse(State {
        lights: 0,
        num_switches: 0,
    }));
    while let Some(Reverse(state)) = queue.pop() {
        if visited.contains(&state.lights) {
            continue;
        }
        if state.lights == problem.target {
            return Some(state.num_switches);
        }
        visited.insert(state.lights);
        for switch in &problem.switches {
            queue.push(Reverse(State {
                lights: state.lights ^ switch,
                num_switches: state.num_switches + 1,
            }));
        }
    }
    None
}

fn part1(input: &Input) -> usize {
    input.iter().map(|p| num_switches(p).unwrap()).sum()
}

fn part2(input: &Input) -> usize {
    unimplemented!()
}

pub fn main() {
    let input = parse(INPUT);
    println!("Day 10 - Part 1: {}", part1(&input));
    println!("Day 10 - Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use crate::{parse, part1, part2};

    #[test]
    fn test_part1() {
        let input = parse(
            "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}\n\
             [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}\n\
             [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}",
        );
        assert_eq!(part1(&input), 7);
    }

    #[test]
    fn test_part2() {
        let input = parse("");
        assert_eq!(part2(&input), 42);
    }
}
