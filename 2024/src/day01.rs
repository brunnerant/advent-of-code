use std::collections::HashMap;

const INPUT: &str = include_str!("../assets/day01.txt");

fn input() -> (Vec<i32>, Vec<i32>) {
    let mut left = Vec::new();
    let mut right = Vec::new();
    for line in INPUT.lines() {
        let numbers: Vec<_> = line
            .split_ascii_whitespace()
            .flat_map(|s| s.parse::<i32>().ok())
            .collect();
        assert!(numbers.len() == 2);
        left.push(numbers[0]);
        right.push(numbers[1]);
    }
    (left, right)
}

fn part1_algo(mut left: Vec<i32>, mut right: Vec<i32>) -> i32 {
    left.sort();
    right.sort();
    left.iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).abs())
        .sum()
}

pub fn part1() -> i32 {
    let (left, right) = input();
    part1_algo(left, right)
}

fn part2_algo(left: Vec<i32>, right: Vec<i32>) -> i32 {
    let mut counts = HashMap::new();
    for r in right {
        *counts.entry(r).or_insert(0) += 1;
    }
    left.iter()
        .map(|l| counts.get(l).copied().unwrap_or(0) * l)
        .sum()
}

pub fn part2() -> i32 {
    let (left, right) = input();
    part2_algo(left, right)
}

#[cfg(test)]
mod tests {
    use super::{part1_algo, part2_algo};

    #[test]
    fn part1() {
        assert_eq!(
            part1_algo(vec![3, 4, 2, 1, 3, 3], vec![4, 3, 5, 3, 9, 3]),
            11
        );
    }

    #[test]
    fn part2() {
        assert_eq!(
            part2_algo(vec![3, 4, 2, 1, 3, 3], vec![4, 3, 5, 3, 9, 3]),
            31
        );
    }
}
