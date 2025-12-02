const INPUT: &str = include_str!("../../assets/day01.txt");

type Input = Vec<i32>;

fn parse(input: &str) -> Input {
    input
        .lines()
        .map(|l| l.replace("L", "-").replace("R", "+").parse().unwrap())
        .collect()
}

fn part1(input: &Input) -> usize {
    let mut lock = 50;
    input
        .iter()
        .filter(|&&i| {
            lock = (lock + i).rem_euclid(100);
            lock == 0
        })
        .count()
}

fn count_zeros(lock: i32, offset: i32) -> usize {
    let result = (offset / 100).unsigned_abs() as usize;
    let offset = offset % 100;
    if lock == 0 {
        result
    } else if (offset > 0 && lock + offset >= 100) || (offset < 0 && lock + offset <= 0) {
        result + 1
    } else {
        result
    }
}

fn part2(input: &Input) -> usize {
    let mut lock = 50;
    let mut result: usize = 0;
    for &i in input.iter() {
        result += count_zeros(lock, i);
        lock = (lock + i).rem_euclid(100);
    }
    result
}

pub fn main() {
    let input = parse(INPUT);
    println!("Day 01 - Part 1: {}", part1(&input));
    println!("Day 01 - Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use crate::{count_zeros, parse, part1, part2};

    #[test]
    fn test_parse() {
        let input = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";
        assert_eq!(
            parse(input),
            vec![-68, -30, 48, -5, 60, -55, -1, -99, 14, -82]
        );
    }

    #[test]
    fn test_part1() {
        let input = parse("L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82");
        assert_eq!(part1(&input), 3);
    }

    #[test]
    fn test_count_zeros() {
        assert_eq!(count_zeros(50, 50), 1);
        assert_eq!(count_zeros(50, -50), 1);
        assert_eq!(count_zeros(50, 60), 1);
        assert_eq!(count_zeros(50, -60), 1);
        assert_eq!(count_zeros(50, 160), 2);
        assert_eq!(count_zeros(50, -160), 2);
        assert_eq!(count_zeros(0, 100), 1);
        assert_eq!(count_zeros(0, -100), 1);
        assert_eq!(count_zeros(99, 101), 2);
        assert_eq!(count_zeros(0, 200), 2);
        assert_eq!(count_zeros(0, 50), 0);
        assert_eq!(count_zeros(0, -50), 0);
    }

    #[test]
    fn test_part2() {
        let input = parse("L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82");
        assert_eq!(part2(&input), 6);
    }
}
