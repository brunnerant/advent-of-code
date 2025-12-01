const INPUT: &str = include_str!("../../assets/dayXY.txt");

type Input = ();

fn parse(input: &str) -> Input {
    unimplemented!()
}

fn part1(input: &Input) -> usize {
    unimplemented!()
}

fn part2(input: &Input) -> usize {
    unimplemented!()
}

pub fn main() {
    let input = parse(INPUT);
    println!("Day XY - Part 1: {}", part1(&input));
    println!("Day XY - Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use crate::{parse, part1, part2};

    #[test]
    fn test_parse() {
        let input = "";
        assert_eq!(parse(input), ());
    }

    #[test]
    fn test_part1() {
        let input = parse("");
        assert_eq!(part1(&input), 42);
    }

    #[test]
    fn test_part2() {
        let input = parse("");
        assert_eq!(part2(&input), 42);
    }
}
