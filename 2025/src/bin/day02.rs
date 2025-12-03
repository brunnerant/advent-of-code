use itertools::Itertools;

const INPUT: &str = include_str!("../../assets/day02.txt");

type Input = Vec<(usize, usize)>;

fn parse(input: &str) -> Input {
    input
        .split([',', '-'])
        .map(|x| x.parse().unwrap())
        .tuples()
        .collect()
}

fn part1(input: &Input) -> usize {
    let mut result = 0;
    for &(a, b) in input.iter() {
        // The numbers need to have an even number of digits in the interval [a_digits, b_digits]
        let a_digits = a.ilog10() + 1;
        let b_digits = b.ilog10() + 1;
        for d in a_digits.div_ceil(2)..=(b_digits / 2) {
            // Each half has d digits, so it needs to lie in the interval [10^(d - 1), 10^d - 1]
            // Furthermore, if h denotes the half, we need to have a <= h * 10^d + h = h * (10^d + 1) <= b.
            // Or equivalently, a / (10^d + 1) <= h <= b / (10^d + 1).
            // We just need to be careful about rounding in the right diretcion.
            let base = 10usize.pow(d);
            let min = a.div_ceil(base + 1).max(base / 10);
            let max = (b / (base + 1)).min(base - 1);
            for x in min..=max {
                result += (base + 1) * x;
            }
        }
    }
    result
}

fn part2(input: &Input) -> usize {
    unimplemented!()
}

pub fn main() {
    let input = parse(INPUT);
    println!("Day 02 - Part 1: {}", part1(&input));
    println!("Day 02 - Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use crate::{parse, part1, part2};

    #[test]
    fn test_part1() {
        let input = parse(
            "11-22,95-115,998-1012,1188511880-1188511890,\
            222220-222224,1698522-1698528,446443-446449,38593856-38593862,\
            565653-565659,824824821-824824827,2121212118-2121212124",
        );
        assert_eq!(part1(&input), 1227775554);
    }

    // #[test]
    // fn test_part2() {
    //     let input = parse("");
    //     assert_eq!(part2(&input), 42);
    // }
}
