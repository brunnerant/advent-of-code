use std::{
    collections::HashSet,
    hash::Hash,
    ops::{Index, IndexMut},
};

const INPUT: &str = include_str!("../assets/day06.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Obstacle,
    Patrolled,
    Empty,
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    pub fn turn_right(&mut self) {
        *self = match self {
            Dir::Up => Dir::Right,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
            Dir::Right => Dir::Down,
        }
    }
}

type Pos = (usize, usize);

#[derive(Clone)]
struct Grid {
    pub cells: Vec<Cell>,
    cols: usize,
    rows: usize,
}

impl Grid {
    pub fn from_str(str: &str) -> (Self, Pos) {
        let mut data = Vec::new();
        let mut cols = None;
        let mut guard_pos = None;
        for (y, line) in str.lines().enumerate() {
            assert_eq!(*cols.get_or_insert(line.len()), line.len());
            data.extend(line.chars().enumerate().map(|(x, c)| match c {
                '.' => Cell::Empty,
                '#' => Cell::Obstacle,
                '^' => {
                    assert!(guard_pos.is_none(), "there should be at most guard");
                    let _ = guard_pos.insert((x, y));
                    Cell::Empty
                }
                _ => panic!("unexpected character in grid"),
            }));
        }

        let cols = cols.unwrap();
        let rows = data.len() / cols;
        (
            Self {
                cells: data,
                cols,
                rows,
            },
            guard_pos.unwrap(),
        )
    }

    pub fn move_guard(&self, &(x, y): &Pos, dir: &Dir) -> Option<Pos> {
        match dir {
            Dir::Up => (y > 0).then(|| (x, y - 1)),
            Dir::Down => (y < self.rows - 1).then(|| (x, y + 1)),
            Dir::Left => (x > 0).then(|| (x - 1, y)),
            Dir::Right => (x < self.cols - 1).then(|| (x + 1, y)),
        }
    }
}

impl Index<Pos> for Grid {
    type Output = Cell;
    fn index(&self, (x, y): Pos) -> &Self::Output {
        &self.cells[y * self.rows + x]
    }
}

impl IndexMut<Pos> for Grid {
    fn index_mut(&mut self, (x, y): Pos) -> &mut Self::Output {
        &mut self.cells[y * self.rows + x]
    }
}

fn trace_path(grid: &mut Grid, mut pos: Pos, mut dir: Dir) {
    loop {
        grid[pos] = Cell::Patrolled;
        if let Some(next_pos) = grid.move_guard(&pos, &dir) {
            if grid[next_pos] == Cell::Obstacle {
                dir.turn_right();
            } else {
                pos = next_pos;
            }
        } else {
            return;
        }
    }
}

fn part1_algo(input: &str) -> usize {
    let (mut grid, guard_pos) = Grid::from_str(input);
    trace_path(&mut grid, guard_pos, Dir::Up);
    grid.cells.iter().filter(|&&c| c == Cell::Patrolled).count()
}

pub fn part1() -> usize {
    part1_algo(&INPUT)
}

fn has_cycle(grid: &Grid, mut pos: Pos, mut dir: Dir, visited: &HashSet<(Pos, Dir)>) -> bool {
    let mut visited = visited.clone();
    loop {
        if visited.contains(&(pos, dir)) {
            return true;
        }
        visited.insert((pos, dir));
        if let Some(next_pos) = grid.move_guard(&pos, &dir) {
            if grid[next_pos] == Cell::Obstacle {
                dir.turn_right();
            } else {
                pos = next_pos;
            }
        } else {
            return false;
        }
    }
}

fn part2_algo(input: &str) -> usize {
    let (mut grid, mut pos) = Grid::from_str(input);
    let mut dir = Dir::Up;
    let mut visited = HashSet::new();
    let mut tried_obstacles = HashSet::new();
    let mut result = 0;
    loop {
        visited.insert((pos, dir));
        grid[pos] = Cell::Patrolled;

        if let Some(next_pos) = grid.move_guard(&pos, &dir) {
            if grid[next_pos] == Cell::Obstacle {
                dir.turn_right();
            } else {
                if grid[next_pos] == Cell::Empty && !tried_obstacles.contains(&next_pos) {
                    grid[next_pos] = Cell::Obstacle;
                    let mut next_dir = dir;
                    next_dir.turn_right();
                    if has_cycle(&grid, pos, next_dir, &visited) {
                        result += 1;
                    }
                    grid[next_pos] = Cell::Empty;
                    tried_obstacles.insert(next_pos);
                }
                pos = next_pos;
            }
        } else {
            return result;
        }
    }
}

pub fn part2() -> usize {
    part2_algo(&INPUT)
}

#[cfg(test)]
mod tests {
    use crate::day06::{part1_algo, part2_algo};

    #[test]
    fn test_part1() {
        let grid = "\
            ....#.....\n\
            .........#\n\
            ..........\n\
            ..#.......\n\
            .......#..\n\
            ..........\n\
            .#..^.....\n\
            ........#.\n\
            #.........\n\
            ......#...";
        assert_eq!(part1_algo(grid), 41);
    }

    #[test]
    fn test_part2() {
        let grid = "\
            ....#.....\n\
            .........#\n\
            ..........\n\
            ..#.......\n\
            .......#..\n\
            ..........\n\
            .#..^.....\n\
            ........#.\n\
            #.........\n\
            ......#...";
        assert_eq!(part2_algo(grid), 6);
    }
}
