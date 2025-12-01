use itertools::Itertools;

const INPUT: &str = include_str!("../assets/day02.txt");

fn input() -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    for line in INPUT.lines() {
        let numbers: Vec<_> = line
            .split_ascii_whitespace()
            .flat_map(|s| s.parse::<i32>().ok())
            .collect();
        result.push(numbers);
    }
    result
}

fn safe_step(a: i32, b: i32, inc: bool) -> bool {
    (1..=3).contains(&(b - a).abs()) && inc == (b > a)
}

fn is_safe_it(report: impl Iterator<Item = i32>) -> bool {
    let mut increasing = None;
    report.tuple_windows().all(|(a, b)| {
        increasing.get_or_insert(b > a);
        safe_step(a, b, increasing.unwrap())
    })
}

fn is_safe(report: &[i32]) -> bool {
    is_safe_it(report.iter().copied())
}

fn is_safe_without(report: &[i32], without: usize) -> bool {
    is_safe_it(
        report
            .iter()
            .enumerate()
            .filter_map(|(i, &v)| (i != without).then_some(v)),
    )
}

fn is_safe_dampened(report: &[i32]) -> bool {
    if is_safe(report) {
        return true;
    }

    for i in 0..report.len() {
        if is_safe_without(report, i) {
            return true;
        }
    }

    false
}

pub fn part1() -> usize {
    input().iter().filter(|report| is_safe(report)).count()
}

pub fn part2() -> usize {
    input()
        .iter()
        .filter(|report| is_safe_dampened(report))
        .count()
}

#[cfg(test)]
mod tests {
    use crate::day02::is_safe_dampened;

    use super::is_safe;

    #[test]
    fn test_is_safe() {
        assert!(is_safe(&vec![7, 6, 4, 2, 1]));
        assert!(!is_safe(&vec![1, 2, 7, 8, 9]));
        assert!(!is_safe(&vec![9, 7, 6, 2, 1]));
        assert!(!is_safe(&vec![1, 3, 2, 4, 5]));
        assert!(!is_safe(&vec![8, 6, 4, 4, 1]));
        assert!(is_safe(&vec![1, 3, 6, 7, 9]));
    }

    #[test]
    fn test_is_safe_dampened() {
        assert!(is_safe_dampened(&vec![7, 6, 4, 2, 1]));
        assert!(!is_safe_dampened(&vec![1, 2, 7, 8, 9]));
        assert!(!is_safe_dampened(&vec![9, 7, 6, 2, 1]));
        assert!(is_safe_dampened(&vec![1, 3, 2, 4, 5]));
        assert!(is_safe_dampened(&vec![8, 6, 4, 4, 1]));
        assert!(is_safe_dampened(&vec![1, 3, 6, 7, 9]));
    }
}
