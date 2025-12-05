use itertools::Itertools;

const INPUT: &str = include_str!("../../assets/day03.txt");

type Input = Vec<Vec<u8>>;

fn parse(input: &str) -> Input {
    input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect()
}

fn part1(input: &Input) -> usize {
    let mut result = 0;
    for line in input {
        assert!(line.len() >= 2);
        let i = line.len() - 2 - line[..line.len() - 1].iter().rev().position_max().unwrap();
        let j = line[i + 1..].iter().max().unwrap();
        let i = line[i];
        result += (10 * i + j) as usize;
    }
    result
}

fn part2(input: &Input) -> usize {
    unimplemented!()
}

pub fn main() {
    let input = parse(INPUT);
    println!("Day 03 - Part 1: {}", part1(&input));
    println!("Day 03 - Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use crate::{parse, part1, part2};

    #[test]
    fn test_part1() {
        let input = parse("987654321111111\n811111111111119\n234234234234278\n818181911112111");
        assert_eq!(part1(&input), 357);
    }

    #[test]
    fn test_part2() {
        let input = parse("");
        assert_eq!(part2(&input), 42);
    }
}
