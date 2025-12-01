use itertools::chain;

struct LetterGrid {
    buffer: String,
    line_size: usize,
}

impl LetterGrid {
    pub fn new(lines: impl Iterator<Item = impl AsRef<str>>) -> Self {
        let mut buffer = String::new();
        let mut line_size = None;
        for line in lines {
            let line = line.as_ref();
            assert_eq!(line.chars().count(), line.len());
            assert_eq!(*line_size.get_or_insert(line.len()), line.len());
            buffer.push_str(line.as_ref());
        }
        Self {
            buffer,
            line_size: line_size.unwrap_or(0),
        }
    }

    pub fn char_at_idx(&self, idx: usize) -> char {
        unsafe { char::from_u32_unchecked(self.buffer.as_bytes()[idx] as u32) }
    }

    pub fn char_at_pos(&self, r: usize, c: usize) -> char {
        self.char_at_idx(r * self.line_size + c)
    }

    pub fn num_cols(&self) -> usize {
        self.line_size
    }

    pub fn num_rows(&self) -> usize {
        self.buffer.len() / self.line_size
    }

    pub fn num_diags(&self) -> usize {
        // a diagonal can start at the start of a row or at the start of a column
        if self.buffer.len() == 0 {
            0
        } else {
            self.num_cols() + self.num_rows() - 1
        }
    }

    pub fn row(&self, i: usize) -> String {
        assert!(i < self.num_rows());
        self.buffer[i * self.line_size..(i + 1) * self.line_size].to_string()
    }

    pub fn col(&self, i: usize) -> String {
        assert!(i < self.num_cols());
        (0..self.num_rows())
            .map(|r| self.char_at_pos(r, i))
            .collect()
    }

    fn diag_len(&self, i: usize) -> usize {
        (i + 1)
            .min(self.num_diags() - i)
            .min(self.num_cols().min(self.num_rows()))
    }

    // Diagonals go towards the lower right. Diagonal 0 is the lower left one, the last one being the upper right one.
    pub fn diag(&self, i: usize) -> String {
        assert!(i < self.num_diags());
        let len = self.diag_len(i);
        let start_idx = if i < self.num_rows() {
            (self.num_rows() - i - 1) * self.line_size
        } else {
            i - self.num_rows() + 1
        };
        let stride = self.line_size + 1;
        (0..len)
            .map(|j| self.char_at_idx(start_idx + j * stride))
            .collect()
    }

    // Inverse diagonals go towards the upper right. Diagonal 0 is the upper left one, the last one being the lower right one.
    pub fn inv_diag(&self, i: usize) -> String {
        assert!(i < self.num_diags());
        let len = self.diag_len(i);
        let start_idx = if i < self.num_rows() {
            i * self.line_size
        } else {
            (self.num_rows() - 1) * self.line_size + i - self.num_rows() + 1
        };
        let stride = self.line_size - 1;
        (0..len)
            .map(|j| self.char_at_idx(start_idx - j * stride))
            .collect()
    }
}

const INPUT: &str = include_str!("../assets/day04.txt");

fn part1_algo(input: &str) -> usize {
    let grid = LetterGrid::new(input.lines());
    let lines = chain![
        (0..grid.num_cols()).map(|c| grid.col(c)),
        (0..grid.num_rows()).map(|r| grid.row(r)),
        (0..grid.num_diags()).map(|d| grid.diag(d)),
        (0..grid.num_diags()).map(|d| grid.inv_diag(d)),
    ];

    let mut result = 0;
    for line in lines {
        result += line.match_indices("XMAS").count();
        result += line.match_indices("SAMX").count();
    }
    result
}

fn is_mas(chars: &[char]) -> bool {
    let chars: String = chars.iter().collect();
    chars == "MAS" || chars == "SAM"
}

fn part2_algo(input: &str) -> usize {
    let grid = LetterGrid::new(input.lines());
    let mut result = 0;
    for r in 1..grid.num_rows() - 1 {
        for c in 1..grid.num_cols() - 1 {
            let diag1 = [
                grid.char_at_pos(r - 1, c - 1),
                grid.char_at_pos(r, c),
                grid.char_at_pos(r + 1, c + 1),
            ];
            let diag2 = [
                grid.char_at_pos(r - 1, c + 1),
                grid.char_at_pos(r, c),
                grid.char_at_pos(r + 1, c - 1),
            ];
            if is_mas(&diag1) && is_mas(&diag2) {
                result += 1;
            }
        }
    }
    result
}

pub fn part1() -> usize {
    part1_algo(INPUT)
}

pub fn part2() -> usize {
    part2_algo(INPUT)
}

#[cfg(test)]
mod tests {
    use crate::day04::{LetterGrid, part1_algo, part2_algo};

    #[test]
    fn test_grid() {
        let grid = LetterGrid::new(
            "\
            ABCD\n\
            EFGH\n\
            IJKL"
                .lines(),
        );

        assert_eq!(grid.num_cols(), 4);
        assert_eq!(grid.num_rows(), 3);
        assert_eq!(grid.num_diags(), 6);

        assert_eq!(grid.row(1), "EFGH");
        assert_eq!(grid.col(1), "BFJ");

        assert_eq!(grid.diag(0), "I");
        assert_eq!(grid.diag(1), "EJ");
        assert_eq!(grid.diag(2), "AFK");
        assert_eq!(grid.diag(3), "BGL");
        assert_eq!(grid.diag(4), "CH");
        assert_eq!(grid.diag(5), "D");

        assert_eq!(grid.inv_diag(0), "A");
        assert_eq!(grid.inv_diag(1), "EB");
        assert_eq!(grid.inv_diag(2), "IFC");
        assert_eq!(grid.inv_diag(3), "JGD");
        assert_eq!(grid.inv_diag(4), "KH");
        assert_eq!(grid.inv_diag(5), "L");
    }

    #[test]
    fn test_part1() {
        let grid = "\
            MMMSXXMASM\n\
            MSAMXMSMSA\n\
            AMXSXMAAMM\n\
            MSAMASMSMX\n\
            XMASAMXAMM\n\
            XXAMMXXAMA\n\
            SMSMSASXSS\n\
            SAXAMASAAA\n\
            MAMMMXMMMM\n\
            MXMXAXMASX";
        assert_eq!(part1_algo(grid), 18);
    }

    #[test]
    fn test_part2() {
        let grid = "\
            MMMSXXMASM\n\
            MSAMXMSMSA\n\
            AMXSXMAAMM\n\
            MSAMASMSMX\n\
            XMASAMXAMM\n\
            XXAMMXXAMA\n\
            SMSMSASXSS\n\
            SAXAMASAAA\n\
            MAMMMXMMMM\n\
            MXMXAXMASX";
        assert_eq!(part2_algo(grid), 9);
    }
}
