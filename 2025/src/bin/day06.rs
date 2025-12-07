use itertools::Itertools;

const INPUT: &str = include_str!("../../assets/day06.txt");

#[derive(Clone, Copy, Debug)]
enum Op {
    Add,
    Mul,
}

fn part1(input: &str) -> usize {
    let mut lines = input.lines();
    let numbers: Vec<Vec<_>> = lines
        .take_while_ref(|l| !l.starts_with(&['+', '*']))
        .map(|l| {
            l.trim()
                .split_ascii_whitespace()
                .map(|n| n.parse::<usize>().unwrap())
                .collect()
        })
        .collect();
    let operators: Vec<_> = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(|op| match op {
            "+" => Op::Add,
            "*" => Op::Mul,
            _ => panic!("invalid operator"),
        })
        .collect();

    let num_operands = numbers.len();
    for i in 0..num_operands {
        assert_eq!(numbers[i].len(), operators.len());
    }

    let mut result = 0;
    for (i, op) in operators.into_iter().enumerate() {
        result += match op {
            Op::Add => (0..num_operands).map(|j| numbers[j][i]).sum::<usize>(),
            Op::Mul => (0..num_operands).map(|j| numbers[j][i]).product(),
        }
    }
    result
}

fn part2(input: &str) -> usize {
    let lines: Vec<_> = input.lines().collect();
    let line_length = lines[0].len();
    for i in 1..lines.len() {
        assert_eq!(lines[i].len(), line_length);
    }

    // group operators into range [a..b] of characters for operation, and operator type
    let mut operators: Vec<_> = lines[lines.len() - 1]
        .bytes()
        .enumerate()
        .filter_map(|(i, c)| match c {
            b'+' => Some((i, 0, Op::Add)),
            b'*' => Some((i, 0, Op::Mul)),
            _ => None,
        })
        .collect();

    // compute the end of the range from the beginning of the next range
    let num_operations = operators.len();
    let num_operands = lines.len() - 1;
    for i in 0..num_operations - 1 {
        operators[i].1 = operators[i + 1].0 - 1;
    }
    operators[num_operations - 1].1 = line_length;

    let mut result = 0;
    for (a, b, op) in operators {
        let operands: Vec<_> = (a..b)
            .map(|i| {
                let chars = (0..num_operands).map(|j| lines[j].as_bytes()[i]).collect();
                let number = String::from_utf8(chars).unwrap();
                number.trim().parse::<usize>().unwrap()
            })
            .collect();
        result += match op {
            Op::Add => operands.iter().sum::<usize>(),
            Op::Mul => operands.iter().product(),
        }
    }
    result
}

pub fn main() {
    println!("Day 06 - Part 1: {}", part1(INPUT));
    println!("Day 06 - Part 2: {}", part2(INPUT));
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn test_part1() {
        let input = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ";
        assert_eq!(part1(&input), 4277556);
    }

    #[test]
    fn test_part2() {
        let input = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ";
        assert_eq!(part2(&input), 3263827);
    }
}
