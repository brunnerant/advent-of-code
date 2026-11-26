use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

use aoc2025::grid::Grid;
use itertools::Itertools;
use regex::Regex;

const INPUT: &str = include_str!("../../assets/day10.txt");

#[derive(Debug)]
struct Problem {
    pub target: usize,
    pub switches: Vec<usize>,
    pub levels: Vec<usize>,
}

type Input = Vec<Problem>;

fn encode_indices(indices: impl Iterator<Item = usize>) -> usize {
    let mut result = 0;
    for i in indices {
        result |= 1 << i
    }
    result
}

fn encode_bits(bits: impl Iterator<Item = bool>) -> usize {
    encode_indices(bits.enumerate().filter_map(|(i, b)| b.then_some(i)))
}

fn parse(input: &str) -> Input {
    let target_regex = Regex::new(r"\[(.*)\]").unwrap();
    let switch_regex = Regex::new(r"\(((?:\d|,)*)\)").unwrap();
    let level_regex = Regex::new(r"\{(\d+(,\d+)*)\}").unwrap();
    let mut result = Vec::new();
    for line in input.lines() {
        let target = encode_bits(
            target_regex.captures(line).unwrap()[1]
                .chars()
                .map(|c| c == '#'),
        );
        let switches = switch_regex
            .captures_iter(line)
            .map(|c| encode_indices(c[1].split(',').map(|n| n.parse().unwrap())))
            .collect();
        let levels = level_regex.captures(line).unwrap()[1]
            .split(",")
            .map(|s| s.parse().unwrap())
            .collect();
        result.push(Problem {
            target,
            switches,
            levels,
        });
    }
    result
}

#[derive(PartialEq, Eq, Debug)]
struct State {
    lights: usize,
    num_switches: usize,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.num_switches.cmp(&other.num_switches)
    }
}

fn num_switches(problem: &Problem) -> Option<usize> {
    let mut queue = BinaryHeap::new();
    let mut visited = HashSet::new();
    queue.push(Reverse(State {
        lights: 0,
        num_switches: 0,
    }));
    while let Some(Reverse(state)) = queue.pop() {
        if visited.contains(&state.lights) {
            continue;
        }
        if state.lights == problem.target {
            return Some(state.num_switches);
        }
        visited.insert(state.lights);
        for switch in &problem.switches {
            queue.push(Reverse(State {
                lights: state.lights ^ switch,
                num_switches: state.num_switches + 1,
            }));
        }
    }
    None
}

fn part1(input: &Input) -> usize {
    input.iter().map(|p| num_switches(p).unwrap()).sum()
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    if a < b {
        (a, b) = (b, a);
    }
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

struct ReducedMatrix {
    elems: Grid<isize>,
    pivots: usize,
    max_values: Vec<usize>,
}

impl ReducedMatrix {
    pub fn new(problem: &Problem) -> Option<ReducedMatrix> {
        let m = problem.levels.len();
        let n = problem.switches.len();
        let mut elems = Grid::fill_with(n + 1, m, |(x, y)| {
            if x == n {
                problem.levels[y] as isize
            } else {
                ((problem.switches[x] >> y) & 1) as isize
            }
        });
        let mut max_values: Vec<usize> = (0..n)
            .map(|i| {
                (0..m)
                    .filter_map(|j| (elems[(i, j)] != 0).then_some(problem.levels[j]))
                    .max()
                    .unwrap()
            })
            .collect();

        // Place pivots one by one
        let mut k = 0;
        while k < m {
            // Chose the column that has the minimal number of non-zero entries (but at least one) among the remaining ones
            let Some(i) = (k..n)
                .filter_map(|i| {
                    let non_zero = (k..m).filter(|&j| elems[(i, j)] != 0).count();
                    (non_zero > 0).then_some((i, non_zero))
                })
                .min_by_key(|&(_, non_zero)| non_zero)
                .map(|(i, _)| i)
            else {
                break;
            };

            // Swap rows and column to bring the pivot along the diagonal
            let j = (k..m).find(|&j| elems[(i, j)] != 0).unwrap();
            elems.swap_columns(k, i);
            elems.swap_rows(k, j);
            max_values.swap(k, i);

            // Reduce the pivot as much as possible
            let g = (k..n + 1)
                .filter_map(|i| (elems[(i, k)] != 0).then_some(elems[(i, k)].unsigned_abs()))
                .reduce(gcd)
                .unwrap() as isize;
            let g = if elems[(k, k)] < 0 { -g } else { g };
            for i in k..n + 1 {
                elems[(i, k)] /= g;
            }

            // Eliminate the non-zero entries in other rows
            let pivot = elems[(k, k)];
            for i in 0..m {
                if i == k || elems[(k, i)] == 0 {
                    continue;
                }
                let g = gcd(pivot.unsigned_abs(), elems[(k, i)].unsigned_abs()) as isize;
                let a = pivot / g;
                let b = elems[(k, i)] / g;
                for j in k..n + 1 {
                    elems[(j, i)] = a * elems[(j, i)] - b * elems[(j, k)];
                }
            }

            k += 1;
        }

        // The system is possible if all rows below the pivots are all zero
        // otherwise, something of the form 0 0 0 0 0 | x > 0 is not possible because it represents an equation 0 = x > 0
        let possible = (k..m).all(|j| (0..n).all(|i| elems[(i, j)] == 0));
        possible.then_some(Self {
            elems,
            pivots: k,
            max_values,
        })
    }

    pub fn min_solution(&self) -> usize {
        let n = self.elems.width - 1;
        let num_free = n - self.pivots;
        let mut min = usize::MAX;
        let mut next_assignment: Vec<usize> = vec![];
        loop {
            // Check if the assignment is feasible so far
            let mut norm = next_assignment.iter().sum::<usize>();
            let feasible = (0..self.pivots).all(|r| {
                // Condition 1: remaining free variables can satisfy the row over the integers
                let target = self.elems[(n, r)]
                    - next_assignment
                        .iter()
                        .enumerate()
                        .map(|(i, &x)| self.elems[(self.pivots + i, r)] * x as isize)
                        .sum::<isize>();
                let gcd = (self.pivots + next_assignment.len()..n)
                    .fold(self.elems[(r, r)] as usize, |a, b| {
                        gcd(a, self.elems[(b, r)].unsigned_abs())
                    });
                if target.rem_euclid(gcd as isize) != 0 {
                    return false;
                }

                // Condition 2: the remaining free variables should not be negative
                match (self.pivots + next_assignment.len()..n)
                    .filter_map(|i| {
                        let x = self.elems[(i, r)].signum();
                        (x != 0).then_some(x)
                    })
                    .all_equal_value()
                {
                    // If the target is negative but all free variables have a positive coeff, impossible
                    Ok(sign) => !(sign == 1 && target.signum() == -1),
                    Err(None) => {
                        // If the dependent variable is constrained and positive, record its value in the norm
                        if target.signum() >= 0 {
                            assert_eq!(gcd, self.elems[(r, r)] as usize);
                            assert_eq!(target % gcd as isize, 0);
                            norm += target as usize / gcd;
                            return true;
                        }
                        false
                    }
                    _ => true,
                }
            });

            // Decide whether to search deeper or continue at the current level
            let dive = if feasible {
                if next_assignment.len() == num_free {
                    // Fully constrained, so we record the score and move on
                    min = min.min(norm);
                    false
                } else {
                    // Explore deeper if we are still more optimal than the best so far
                    norm < min
                }
            } else {
                // If the solution is not optimal, adding variables won't improve it
                false
            };

            if dive {
                // Recurse deeper
                next_assignment.push(0);
            } else {
                // Change the last assignment or pop it if it became too big
                while let Some(last) = next_assignment.pop() {
                    if last < self.max_values[self.pivots + next_assignment.len()] {
                        next_assignment.push(last + 1);
                        break;
                    }
                }

                // We exhausted the search space
                if next_assignment.is_empty() {
                    return min;
                }
            }
        }
    }
}

fn part2(input: &Input) -> usize {
    input
        .iter()
        .map(|p| ReducedMatrix::new(p).unwrap().min_solution())
        .sum()
}

pub fn main() {
    let input = parse(INPUT);
    println!("Day 10 - Part 1: {}", part1(&input));
    println!("Day 10 - Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use crate::{parse, part1, part2};

    #[test]
    fn test_part1() {
        let input = parse(
            "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}\n\
             [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}\n\
             [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}",
        );
        assert_eq!(part1(&input), 7);
    }

    #[test]
    fn test_part2() {
        let input = parse(
            "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}\n\
             [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}\n\
             [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}",
        );
        assert_eq!(part2(&input), 33);
        assert_eq!(part2(&parse("[] (0) (1) (0,1) (2,3) {1,1,0,0}")), 1);
        assert_eq!(part2(&parse("[] (0) (1,2) (1,3) (2,3) (3) {1,2,2,2}")), 4);
        assert_eq!(part2(&parse("[] (0,1) (1,2) (2,0) {3,3,2}")), 4);
    }
}
