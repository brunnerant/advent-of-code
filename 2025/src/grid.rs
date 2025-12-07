use std::ops::{Index, IndexMut};

use itertools::Itertools;

#[derive(Debug, Clone)]
pub struct Grid<T> {
    cells: Vec<T>,
    pub width: usize,
    pub height: usize,
}

impl<T> Grid<T> {
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
