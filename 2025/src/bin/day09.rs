use itertools::Itertools;

const INPUT: &str = include_str!("../../assets/day09.txt");

type Input = Vec<(isize, isize)>;

fn parse(input: &str) -> Input {
    input
        .lines()
        .flat_map(|l| l.split(','))
        .map(|n| n.parse().unwrap())
        .tuples()
        .collect()
}

fn part1(input: &Input) -> isize {
    input
        .iter()
        .tuple_combinations()
        .map(|(&(x1, y1), &(x2, y2))| ((x1 - x2).abs() + 1) * ((y1 - y2).abs() + 1))
        .max()
        .unwrap()
}

fn part2(input: &Input) -> isize {
    unimplemented!()
}

pub fn main() {
    let input = parse(INPUT);
    println!("Day 09 - Part 1: {}", part1(&input));
    println!("Day 09 - Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use crate::{parse, part1, part2};

    #[test]
    fn test_part1() {
        let input = parse("7,1\n11,1\n11,7\n9,7\n9,5\n2,5\n2,3\n7,3");
        assert_eq!(part1(&input), 50);
    }

    #[test]
    fn test_part2() {
        let input = parse("");
        assert_eq!(part2(&input), 42);
    }
}
