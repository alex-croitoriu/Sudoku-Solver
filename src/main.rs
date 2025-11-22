use std::fs::File;
use std::io::{self, BufRead};
use std::time::Instant;

mod config;
mod sudoku;

use crate::config::{
    BENCHMARK as benchmark, FILE_PATH as file_path, MAX_GRIDS as max_grids,
    PRINT_SOLVED_GRIDS as print_solved_grids, USE_MRV_HEURISTIC as use_mrv_heuristic,
};
use crate::sudoku::Sudoku;

fn main() {
    let mut grids_solved = 0;
    let start = Instant::now();

    if let Ok(file) = File::open(file_path) {
        let buffer = io::BufReader::with_capacity(1_000_000, file);

        for (i, line) in buffer.lines().enumerate() {
            if i as isize == max_grids {
                break;
            }
            if let Ok(line) = line {
                match Sudoku::new(line.trim(), use_mrv_heuristic) {
                    Ok(mut sudoku) => {
                        match sudoku.solve(0) {
                            Some(()) => grids_solved += 1,
                            None => println!("Grid on line {} has no solution", i + 1),
                        }
                        if print_solved_grids {
                            println!("{}", sudoku);
                        }
                    }
                    Err(e) => eprintln!("Invalid grid on line {}: {e}", i + 1),
                }
            }
        }

        let elapsed = start.elapsed().as_secs_f32();

        println!("Grids solved: {}", grids_solved);

        if benchmark {
            println!("Time elapsed: {} seconds", elapsed);
            println!(
                "Average time per grid: {} ms",
                (elapsed * 1000f32) / grids_solved as f32
            );
            println!(
                "Average grids per second: {}",
                grids_solved as f32 / elapsed
            );
            println!();
        }
    } else {
        eprintln!("File '{}' not found", file_path);
    }
}
