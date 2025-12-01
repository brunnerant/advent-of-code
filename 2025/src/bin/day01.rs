const INPUT: &str = include_str!("../../assets/day01.txt");

fn parse(input: &str) -> Vec<i32> {
    input.lines().map(|line| {
        let mut line = line.chars();
        let sign = line.next().expect("missing sign");
        let sign = match sign {
            'L' => -1,
            'R' => 1,
            _ => panic!("invalid sign"),
        };
        sign * line.as_str().parse::<i32>().expect("invalid number")
    }).collect()
}

fn part1_algo(input: &str) -> usize {
    let input = parse(input);
    let mut lock = 50;
    input.into_iter().filter(|&i| {
        lock = (lock + i).rem_euclid(100);
        lock == 0
    }).count()
}

fn part1() -> usize {
    part1_algo(INPUT)
}

fn part2_algo(input: &str) -> usize {
    todo!()
}

fn part2() -> usize {
    part1_algo(INPUT)
}

pub fn main() {
    println!("Day 01 - Part 1: {}", part1());
    // println!("Day 01 - Part 2: {}", part2());
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parse() {
        let input = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";
        assert_eq!(super::parse(input), vec![-68, -30, 48, -5, 60, -55, -1, -99, 14, -82]);
    }

    #[test]
    fn test_part1() {
        let input = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";
        assert_eq!(super::part1_algo(input), 3);
    }
}
