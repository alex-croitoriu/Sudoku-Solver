use std::io::{self, BufRead};
use std::time::Instant;
use std::fs::File;

mod sudoku;
mod config;

use crate::sudoku::Sudoku;
use crate::config::
{
    FILE_NAME as file_name, 
    PRINT_STATS as print_stats,
    NUMBER_OF_GRIDS as number_of_grids, 
    PRINT_SOLVED_GRIDS as print_solved_grids,
    MRV_HEURISTIC as heuristic
};

fn main() 
{
    let mut solved = 0;
    let mut board = [0; 81];

    let start = Instant::now();

    if let Ok(file) = File::open(file_name)
    {
        let lines = io::BufReader::new(file).lines();

        for (i, line) in lines.enumerate()
        {
            if i == number_of_grids
            {
                break;
            }
            if let Ok(grid) = line
            {
                for (i, digit) in grid.trim().bytes().enumerate()
                {
                    board[i] = digit as u32 - '0' as u32;
                }

                let mut sudoku: Sudoku = Sudoku::new(board, heuristic);
                sudoku.solve(0);

                if sudoku.solved
                {
                    solved += 1;
                }

                if print_solved_grids
                {
                    println!("{}", sudoku);
                }

            }
        }

        let elapsed = start.elapsed().as_secs_f32();

        if print_stats
        {
            println!("Grids solved: {}/{}", solved, number_of_grids);
            println!("Time elapsed: {} seconds", elapsed);
            println!("Average time per grid: {} seconds", elapsed / solved as f32);
            println!("Average grids per second: {}", solved as f32 / elapsed);
            println!();
        }
    }
    else
    {
        eprintln!("File not found.");
    }

}