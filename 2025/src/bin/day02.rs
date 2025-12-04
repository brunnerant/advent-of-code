use std::collections::HashSet;

use itertools::Itertools;

const INPUT: &str = include_str!("../../assets/day02.txt");

type Input = Vec<(usize, usize)>;

fn parse(input: &str) -> Input {
    input
        .split([',', '-'])
        .map(|x| x.parse().unwrap())
        .tuples()
        .collect()
}

fn possible_numbers(a: usize, b: usize, num_groups: u32) -> impl Iterator<Item = usize> {
    assert!(b >= a);
    let a_digits = a.ilog10() + 1;
    let b_digits = b.ilog10() + 1;
    // Iterate over the possible number of digits in the repeated part
    (a_digits.div_ceil(num_groups)..=(b_digits / num_groups)).flat_map(move |group_size| {
        // Given the part p, the multiplier m is such that the full number n
        // can be written as n = m * p
        let base = 10usize.pow(group_size);
        let multiplier = (0..num_groups).map(|i| base.pow(i)).sum();

        // The part p must be comprised in range [base / 10, base - 1] because leading zeroes are not allowed
        let min = a.div_ceil(multiplier).max(base / 10);
        let max = (b / multiplier).min(base - 1);
        (min..=max).map(move |x| x * multiplier)
    })
}

fn part1(input: &Input) -> usize {
    let mut result = 0;
    for &(a, b) in input.iter() {
        result += possible_numbers(a, b, 2).sum::<usize>();
    }
    result
}

fn part2(input: &Input) -> usize {
    let mut result = 0;
    let mut distinct = HashSet::new();
    for &(a, b) in input.iter() {
        let b_digits = b.ilog10() + 1;
        for num_groups in 2..=b_digits {
            distinct.extend(possible_numbers(a, b, num_groups));
        }
        result += distinct.drain().sum::<usize>();
    }
    result
}

pub fn main() {
    let input = parse(INPUT);
    println!("Day 02 - Part 1: {}", part1(&input));
    println!("Day 02 - Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use crate::{parse, part1, part2};

    #[test]
    fn test_part1() {
        let input = parse(
            "11-22,95-115,998-1012,1188511880-1188511890,\
            222220-222224,1698522-1698528,446443-446449,38593856-38593862,\
            565653-565659,824824821-824824827,2121212118-2121212124",
        );
        assert_eq!(part1(&input), 1227775554);
    }

    #[test]
    fn test_part2() {
        let input = parse(
            "11-22,95-115,998-1012,1188511880-1188511890,\
            222220-222224,1698522-1698528,446443-446449,38593856-38593862,\
            565653-565659,824824821-824824827,2121212118-2121212124",
        );
        assert_eq!(part2(&input), 4174379265);
    }
}
