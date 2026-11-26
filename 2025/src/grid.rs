use std::ops::{Index, IndexMut};

use itertools::Itertools;

#[derive(Debug, Clone)]
pub struct Grid<T> {
    cells: Vec<T>,
    pub width: usize,
    pub height: usize,
}

impl<T> Grid<T> {
    pub fn fill_with_elem(width: usize, height: usize, elem: T) -> Self
    where
        T: Clone,
    {
        Self::fill_with(width, height, |_| elem.clone())
    }

    pub fn fill_with(width: usize, height: usize, func: impl Fn((usize, usize)) -> T) -> Self {
        let mut grid = Self {
            cells: Vec::with_capacity(width * height),
            width,
            height,
        };
        for pos in grid.positions() {
            grid.cells.push(func(pos));
        }
        grid
    }

    pub fn from_lines(lines: impl Iterator<Item = impl Iterator<Item = T>>) -> Option<Self> {
        let mut cells = Vec::new();
        let mut width = None;
        let mut height = 0;
        for line in lines {
            let size_before = cells.len();
            cells.extend(line);
            let line_size = cells.len() - size_before;
            if let Some(w) = width {
                if w != line_size {
                    return None;
                }
            } else {
                width = Some(line_size);
            }
            height += 1;
        }
        Some(Grid {
            cells,
            width: width.unwrap_or(0),
            height,
        })
    }

    pub fn positions(&self) -> impl Iterator<Item = (usize, usize)> + use<T> {
        (0..self.height)
            .cartesian_product(0..self.width)
            .map(|(y, x)| (x, y))
    }

    pub fn adjacent_cells(&self, (x, y): (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
        let x0: isize = if x > 0 { -1 } else { 0 };
        let x1: isize = if x < self.width - 1 { 1 } else { 0 };
        let y0: isize = if y > 0 { -1 } else { 0 };
        let y1: isize = if y < self.height - 1 { 1 } else { 0 };
        (x0..=x1)
            .cartesian_product(y0..=y1)
            .filter(|&(i, j)| (i, j) != (0, 0))
            .map(move |(i, j)| (x.wrapping_add_signed(i), y.wrapping_add_signed(j)))
    }

    pub fn swap_cells(&mut self, (x1, y1): (usize, usize), (x2, y2): (usize, usize)) {
        self.cells.swap(y1 * self.width + x1, y2 * self.width + x2);
    }

    pub fn swap_rows(&mut self, y1: usize, y2: usize) {
        for x in 0..self.width {
            self.swap_cells((x, y1), (x, y2));
        }
    }

    pub fn swap_columns(&mut self, x1: usize, x2: usize) {
        for y in 0..self.height {
            self.swap_cells((x1, y), (x2, y));
        }
    }
}

impl<T> std::fmt::Display for Grid<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (x, y) in self.positions() {
            write!(f, "{}", self[(x, y)])?;
            if x == self.width - 1 {
                writeln!(f)?;
            } else {
                write!(f, " ")?;
            }
        }
        Ok(())
    }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.cells[y * self.width + x]
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.cells[y * self.width + x]
    }
}
