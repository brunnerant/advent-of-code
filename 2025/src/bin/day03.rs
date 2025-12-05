const INPUT: &str = include_str!("../../assets/day03.txt");

type Input = Vec<Vec<u8>>;

fn parse(input: &str) -> Input {
    input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect()
}

fn best_ndigits(digits: &[u8], n: usize) -> usize {
    assert!(digits.len() >= n);
    let mut result = 0;
    let mut start = 0;
    // i represents the remaining number of digits
    for i in (0..n).rev() {
        let mut best_idx = start;
        for j in start + 1..digits.len() - i {
            if digits[j] > digits[best_idx] {
                best_idx = j;
            }
        }
        start = best_idx + 1;
        result = result * 10 + digits[best_idx] as usize;
    }
    result
}

fn part1(input: &Input) -> usize {
    let mut result = 0;
    for line in input {
        result += best_ndigits(line, 2);
    }
    result
}

fn part2(input: &Input) -> usize {
    let mut result = 0;
    for line in input {
        result += best_ndigits(line, 12);
    }
    result
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
        let input = parse("987654321111111\n811111111111119\n234234234234278\n818181911112111");
        assert_eq!(part2(&input), 3121910778619);
    }
}
