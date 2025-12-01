
const INPUT: &str = include_str!("../../assets/dayXY.txt");

fn part1_algo(input: &str) -> usize {
    todo!()
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
    println!("Day XY - Part 1: {}", part1());
    println!("Day XY - Part 2: {}", part2());
}

#[cfg(test)]
mod tests {
    use crate::dayXY::{part1_algo, part2_algo};

    #[test]
    fn test_part1() {
        let input = "";
        assert_eq!(part1_algo(input), 0);
    }

    #[test]
    fn test_part2() {
        let input = "";
        assert_eq!(part2_algo(input), 0);
    }
}
