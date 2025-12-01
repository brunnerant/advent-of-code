const INPUT: &str = include_str!("../assets/day07.txt");

fn parse_input(input: &str) -> Vec<(usize, Vec<usize>)> {
    let mut result = Vec::new();
    for line in input.lines() {
        let [total, numbers] = line.split(':').collect::<Vec<_>>()[..] else {
            panic!("unexpected format for input");
        };
        let total = total.parse::<usize>().unwrap();
        let numbers = numbers
            .trim()
            .split_ascii_whitespace()
            .map(|n| n.parse::<usize>().unwrap())
            .collect();
        result.push((total, numbers));
    }
    result
}

fn is_possible_part1(numbers: &[usize], target: usize) -> bool {
    match numbers.len() {
        0 => false,
        1 => numbers[0] == target,
        _ => {
            let firsts = &numbers[..numbers.len() - 1];
            let last = numbers[numbers.len() - 1];
            (target % last == 0 && is_possible_part1(firsts, target / last))
                || (last <= target && is_possible_part1(firsts, target - last))
        }
    }
}

fn sum_possible(input: &str, possible: impl Fn(&[usize], usize) -> bool) -> usize {
    let lines = parse_input(input);
    let mut result = 0;
    for (target, numbers) in lines {
        if possible(&numbers, target) {
            result += target;
        }
    }
    result
}

pub fn part1() -> usize {
    sum_possible(INPUT, is_possible_part1)
}

fn unconcat(target: usize, last: usize) -> Option<usize> {
    if last > target {
        None
    } else {
        let num_digits = last.checked_ilog10().unwrap_or(0) + 1;
        let divisor = 10_usize.pow(num_digits);
        if (target - last) % divisor == 0 {
            Some((target - last) / divisor)
        } else {
            None
        }
    }
}

fn is_possible_part2(numbers: &[usize], target: usize) -> bool {
    match numbers.len() {
        0 => false,
        1 => numbers[0] == target,
        _ => {
            let firsts = &numbers[..numbers.len() - 1];
            let last = numbers[numbers.len() - 1];
            (target % last == 0 && is_possible_part2(firsts, target / last))
                || (last <= target && is_possible_part2(firsts, target - last))
                || unconcat(target, last)
                    .map(|new_target| is_possible_part2(firsts, new_target))
                    .unwrap_or(false)
        }
    }
}

pub fn part2() -> usize {
    sum_possible(INPUT, is_possible_part2)
}

#[cfg(test)]
mod tests {
    use crate::day07::{is_possible_part1, is_possible_part2, sum_possible, unconcat};

    const TEST_INPUT: &str = "\
            190: 10 19\n\
            3267: 81 40 27\n\
            83: 17 5\n\
            156: 15 6\n\
            7290: 6 8 6 15\n\
            161011: 16 10 13\n\
            192: 17 8 14\n\
            21037: 9 7 18 13\n\
            292: 11 6 16 20";

    #[test]
    fn test_unconcat() {
        assert_eq!(unconcat(101, 1), Some(10));
        assert_eq!(unconcat(101, 2), None);
        assert_eq!(unconcat(100, 0), Some(10));
        assert_eq!(unconcat(0, 0), Some(0));
        assert_eq!(unconcat(100, 100), Some(0));
        assert_eq!(unconcat(1010, 10), Some(10));
        assert_eq!(unconcat(1010, 11), None);
    }

    #[test]
    fn test_part1() {
        assert_eq!(sum_possible(TEST_INPUT, is_possible_part1), 3749);
    }

    #[test]
    fn test_part2() {
        assert_eq!(sum_possible(TEST_INPUT, is_possible_part2), 11387);
    }
}
