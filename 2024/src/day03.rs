use regex::Regex;

const INPUT: &str = include_str!("../assets/day03.txt");

pub fn part1_algo(input: &str) -> i32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut result = 0;
    for cap in re.captures_iter(input) {
        result += &cap[1].parse::<i32>().unwrap() * &cap[2].parse::<i32>().unwrap();
    }
    result
}

pub fn part2_algo(input: &str) -> i32 {
    let re = Regex::new(r"don't|do|mul\((\d+),(\d+)\)").unwrap();
    let mut result = 0;
    let mut enabled = true;
    for cap in re.captures_iter(input) {
        match &cap[0] {
            "do" => enabled = true,
            "don't" => enabled = false,
            _ if enabled => {
                result += &cap[1].parse::<i32>().unwrap() * &cap[2].parse::<i32>().unwrap()
            }
            _ => (),
        }
    }
    result
}

pub fn part1() -> i32 {
    part1_algo(INPUT)
}

pub fn part2() -> i32 {
    part2_algo(INPUT)
}

#[cfg(test)]
mod tests {
    use crate::day03::{part1_algo, part2_algo};

    #[test]
    fn test_part1() {
        assert_eq!(
            part1_algo("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"),
            161
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2_algo("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"),
            48
        );
    }
}
