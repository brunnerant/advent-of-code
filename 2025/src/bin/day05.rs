use itertools::Itertools;

const INPUT: &str = include_str!("../../assets/day05.txt");

type Input = (Vec<(usize, usize)>, Vec<usize>);

fn parse(input: &str) -> Input {
    let mut lines = input.lines();
    let mut intervals = lines
        .take_while_ref(|l| !l.is_empty())
        .map(|l| {
            let (a, b) = l.split_once('-').unwrap();
            (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap())
        })
        .collect();
    let queries = lines.skip(1).map(|l| l.parse::<usize>().unwrap()).collect();
    merge_overlaps(&mut intervals);
    (intervals, queries)
}

fn merge_overlaps(intervals: &mut Vec<(usize, usize)>) {
    intervals.sort_by_key(|&(a, _)| a);
    let mut last_idx = 0;
    for i in 1..intervals.len() {
        let (_a0, a1) = intervals[last_idx];
        let (b0, b1) = intervals[i];
        if b0 <= a1 + 1 {
            intervals[last_idx].1 = a1.max(b1);
        } else {
            last_idx += 1;
            intervals[last_idx] = intervals[i];
        }
    }
    intervals.resize(last_idx + 1, (9, 0));
}

fn is_in_intervals(intervals: &Vec<(usize, usize)>, query: usize) -> bool {
    let mut i = 0;
    let mut j = intervals.len() - 1;
    while j > i {
        let mid = (i + j) / 2;
        if query <= intervals[mid].1 {
            j = mid;
        } else {
            i = mid + 1;
        }
    }
    assert!(i == j);
    intervals[i].0 <= query && query <= intervals[i].1
}

fn part1((intervals, queries): &Input) -> usize {
    let mut result = 0;
    for &query in queries {
        if is_in_intervals(intervals, query) {
            result += 1;
        }
    }
    result
}

fn part2((intervals, _): &Input) -> usize {
    let mut result = 0;
    for &(a, b) in intervals {
        result += b - a + 1;
    }
    result
}

pub fn main() {
    let input = parse(INPUT);
    println!("Day 05 - Part 1: {}", part1(&input));
    println!("Day 05 - Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use crate::{parse, part1, part2};

    #[test]
    fn test_part1() {
        let (intervals, queries) = parse("3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32");
        assert_eq!(part1(&(intervals, queries)), 3);
    }

    #[test]
    fn test_part2() {
        let input = parse("3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32");
        assert_eq!(part2(&input), 14);
    }
}
