use std::fs::File;
use std::io::{self, BufRead};
use std::time::Instant;

mod config;
mod sudoku;

use crate::config::{
    BENCHMARK as benchmark, FILE_PATH as file_path, MAX_GRIDS as max_grids,
    PRINT_SOLVED_GRIDS as print_solved_grids,
};
use crate::sudoku::Sudoku;

fn main() {
    let mut grids_solved = 0;
    let mut elapsed = 0f32;

    if let Ok(file) = File::open(file_path) {
        let buffer = io::BufReader::with_capacity(1_000_000, file);

        for (i, line) in buffer.lines().enumerate() {
            let start = Instant::now();
            if i as isize == max_grids {
                break;
            }
            if let Ok(line) = line {
                match Sudoku::new(line.trim()) {
                    Ok(mut sudoku) => match sudoku.solve(0) {
                        Some(()) => {
                            elapsed += start.elapsed().as_secs_f32();
                            grids_solved += 1;
                            if print_solved_grids {
                                println!("{sudoku}");
                            }
                        }
                        None => println!("Grid on line {} has no solution\n", i + 1),
                    },
                    Err(e) => eprintln!("Invalid grid on line {}: {e}\n", i + 1),
                }
            }
        }

        println!("Grids solved: {grids_solved}");

        if benchmark {
            println!("Time elapsed: {elapsed} seconds");
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
        eprintln!("File '{file_path}' not found");
    }
}
