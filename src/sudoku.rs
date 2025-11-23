use anyhow::{Result, anyhow};
use std::fmt;

pub struct Sudoku {
    grid: [[u8; 9]; 9],
    row_mask: [u32; 9],
    col_mask: [u32; 9],
    block_mask: [u32; 9],
    empty_cells: Vec<(usize, usize, usize)>,
}

#[derive(Clone)]
pub enum Status {
    Solved([[u8; 9]; 9]),
    Unsolved(usize),
    Invalid(usize, String),
}

impl Sudoku {
    pub fn new(line: &str) -> Result<Self> {
        let mut grid = [[0u8; 9]; 9];
        let (mut row_mask, mut col_mask, mut block_mask) = ([0; 9], [0; 9], [0; 9]);
        let mut empty_cells: Vec<(usize, usize, usize)> = Vec::new();
        empty_cells.reserve_exact(81);

        if line.len() != 81 {
            return Err(anyhow!("length is not 81"));
        }

        for (i, byte) in line.bytes().enumerate() {
            if !byte.is_ascii_digit() {
                return Err(anyhow!("non-digit character(s) found"));
            }
            grid[i / 9][i % 9] = byte - b'0';
        }

        for row in 0..9 {
            for col in 0..9 {
                let digit = grid[row][col] as usize;
                let block = row / 3 * 3 + col / 3;
                if digit == 0 {
                    empty_cells.push((row, col, block));
                } else {
                    let digit = digit - 1;
                    row_mask[row] |= 1 << digit;
                    col_mask[col] |= 1 << digit;
                    block_mask[block] |= 1 << digit;
                }
            }
        }

        Ok(Sudoku {
            grid,
            row_mask,
            col_mask,
            block_mask,
            empty_cells,
        })
    }

    pub fn solve(&mut self, current_index: usize) -> Option<[[u8; 9]; 9]> {
        if current_index == self.empty_cells.len() {
            return Some(self.grid);
        }

        let mut best_index = current_index;
        let mut fewest_candidates = 9;

        for (i, &(row, col, block)) in self.empty_cells[current_index..].iter().enumerate() {
            let mask = self.row_mask[row] | self.col_mask[col] | self.block_mask[block];
            let current_candidates = 9 - mask.count_ones();

            if current_candidates == 0 {
                return None;
            }
            if current_candidates == 1 {
                best_index = i + current_index;
                break;
            }
            if fewest_candidates > current_candidates {
                fewest_candidates = current_candidates;
                best_index = i + current_index;
            }
        }
        self.empty_cells.swap(current_index, best_index);

        let (row, col, block) = self.empty_cells[current_index];
        let mask = self.row_mask[row] | self.col_mask[col] | self.block_mask[block];
        let mut candidates_mask = !mask & 511;

        while candidates_mask != 0 {
            let candidate_bit = candidates_mask & candidates_mask.wrapping_neg();

            self.row_mask[row] |= candidate_bit;
            self.col_mask[col] |= candidate_bit;
            self.block_mask[block] |= candidate_bit;

            if self.solve(current_index + 1).is_some() {
                self.grid[row][col] = candidate_bit.trailing_zeros() as u8 + 1;
                return Some(self.grid);
            }

            self.row_mask[row] ^= candidate_bit;
            self.col_mask[col] ^= candidate_bit;
            self.block_mask[block] ^= candidate_bit;

            candidates_mask &= candidates_mask - 1;
        }

        None
    }
}

impl Status {
    pub fn pretty(&self) -> String {
        match self {
            Status::Solved(grid) => {
                let mut s = String::new();
                for (i, row) in grid.iter().enumerate() {
                    if i % 3 == 0 {
                        s.push_str("+-------+-------+-------+\n");
                    }
                    for (j, cell) in row.iter().enumerate() {
                        if j % 3 == 0 {
                            s.push_str("| ");
                        }
                        s.push_str(&format!("{cell} "));
                    }
                    s.push_str("|\n");
                }
                s.push_str("+-------+-------+-------+\n");
                s
            }
            _ => self.to_string(),
        }
    }
}
impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Status::Solved(grid) => {
                for row in grid {
                    for cell in row {
                        write!(f, "{cell}")?;
                    }
                }
                writeln!(f)?;
            }
            Status::Unsolved(line) => {
                writeln!(f, "Grid on line {line} has no solution")?;
            }
            Status::Invalid(line, message) => {
                writeln!(f, "Invalid grid on line {line}: {message}")?;
            }
        }

        Ok(())
    }
}
