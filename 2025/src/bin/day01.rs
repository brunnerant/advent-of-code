const INPUT: &str = include_str!("../../assets/day01.txt");

fn parse(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|line| {
            let (dir, number) = line.split_at(1);
            let sign = match dir {
                "L" => -1,
                "R" => 1,
                _ => panic!("invalid sign"),
            };
            sign * number.parse::<i32>().expect("invalid number")
        })
        .collect()
}

fn part1_algo(input: &str) -> usize {
    let input = parse(input);
    let mut lock = 50;
    input
        .into_iter()
        .filter(|&i| {
            lock = (lock + i).rem_euclid(100);
            lock == 0
        })
        .count()
}

fn part1() -> usize {
    part1_algo(INPUT)
}

fn count_zeros(lock: i32, offset: i32) -> usize {
    // no movement -> nothing happens and we don't count going to zero again
    if offset == 0 {
        return 0;
    }

    // question to ask: how many multiples of 100 does the range [start, end] contain ?
    let mut start = lock + offset.signum();
    let mut end = lock + offset;

    // simplify the problem: always have an increasing interval start -> end
    if offset.signum() == -1 {
        start = -start;
        end = -end;
    }

    // count how many full cycles are contained within the interval by the pigeonhole principle
    let div = (end - start).div_euclid(100);
    let result = div.abs() as usize;

    // truncate the interval by that many full cycles and see if it contains one last occurence of zero
    start += div * 100;
    if start.div_euclid(100) != end.div_euclid(100) || start.rem_euclid(100) == 0 {
        result + 1
    } else {
        result
    }
}

fn part2_algo(input: &str) -> usize {
    let input = parse(input);
    let mut lock = 50;
    let mut result: usize = 0;
    for i in input {
        result += count_zeros(lock, i);
        lock = (lock + i).rem_euclid(100);
    }
    result
}

fn part2() -> usize {
    part2_algo(INPUT)
}

pub fn main() {
    println!("Day 01 - Part 1: {}", part1());
    println!("Day 01 - Part 2: {}", part2());
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parse() {
        let input = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";
        assert_eq!(
            super::parse(input),
            vec![-68, -30, 48, -5, 60, -55, -1, -99, 14, -82]
        );
    }

    #[test]
    fn test_part1() {
        let input = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";
        assert_eq!(super::part1_algo(input), 3);
    }

    #[test]
    fn test_count_zeros() {
        assert_eq!(super::count_zeros(50, 50), 1);
        assert_eq!(super::count_zeros(50, -50), 1);
        assert_eq!(super::count_zeros(50, 60), 1);
        assert_eq!(super::count_zeros(50, -60), 1);
        assert_eq!(super::count_zeros(50, 160), 2);
        assert_eq!(super::count_zeros(50, -160), 2);
        assert_eq!(super::count_zeros(0, 100), 1);
        assert_eq!(super::count_zeros(0, -100), 1);
        assert_eq!(super::count_zeros(-1, 101), 2);
        assert_eq!(super::count_zeros(0, 200), 2);
        assert_eq!(super::count_zeros(0, 50), 0);
        assert_eq!(super::count_zeros(0, -50), 0);
    }

    #[test]
    fn test_part2() {
        let input = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";
        assert_eq!(super::part2_algo(input), 6);
    }
}
