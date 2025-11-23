use anyhow::{Result, anyhow};
use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::sync::{Arc, Mutex};
use std::time::Instant;
use threadpool::ThreadPool;

mod args;
mod sudoku;

use crate::args::Cli;
use crate::sudoku::{Status, Sudoku};

fn main() -> Result<()> {
    let args = Cli::parse();

    let file = match File::open(&args.input_file) {
        Ok(file) => file,
        Err(_) => Err(anyhow!("File '{}' not found", args.input_file))?,
    };

    let size = file.metadata()?.len() as usize;
    let line_count = size / 82;

    let buffer = BufReader::with_capacity(size, file);
    let mut results = Vec::with_capacity(line_count);

    let start = Instant::now();

    if let Some(threads) = args.threads {
        let pool = ThreadPool::new(threads);
        let pool_results = Arc::new(Mutex::new(Vec::<Status>::new()));

        for (i, line) in buffer.lines().enumerate() {
            let pool_results = pool_results.clone();
            if let Ok(line) = line {
                pool.execute(move || match Sudoku::new(line.trim()) {
                    Ok(mut sudoku) => {
                        let result = sudoku.solve(0);
                        if let Ok(mut pool_results) = pool_results.lock() {
                            match result {
                                Some(grid) => pool_results.push(Status::Solved(grid)),
                                None => pool_results.push(Status::Unsolved(i + 1)),
                            }
                        }
                    }
                    Err(e) => {
                        if let Ok(mut pool_results) = pool_results.lock() {
                            pool_results.push(Status::Invalid(i + 1, e.to_string()))
                        }
                    }
                });
            }
        }

        pool.join();
        if let Ok(pool_results) = pool_results.lock() {
            results = pool_results.clone();
        }
    } else {
        for (i, line) in buffer.lines().enumerate() {
            // if let Some(max_results) = args.max_results && max_results == i {
            //     break;
            // }
            if let Ok(line) = line {
                match Sudoku::new(line.trim()) {
                    Ok(mut sudoku) => match sudoku.solve(0) {
                        Some(grid) => results.push(Status::Solved(grid)),
                        None => results.push(Status::Unsolved(i + 1)),
                    },
                    Err(e) => results.push(Status::Invalid(i + 1, e.to_string())),
                }
            }
        }
    }

    let elapsed = start.elapsed();
    let solved_grids_count = results
        .iter()
        .filter(|s| matches!(s, Status::Solved(_)))
        .count();

    println!("Grids solved: {solved_grids_count}/{}", results.len());

    if args.stats {
        println!("Execution time: {} seconds", elapsed.as_secs_f32());
        println!(
            "Average grids/second: {}",
            solved_grids_count as f32 / elapsed.as_secs_f32()
        );
    }

    if let Some(output_file) = args.output_file {
        let file = match File::create(&output_file) {
            Ok(file) => file,
            Err(_) => Err(anyhow!("File '{output_file}' not found"))?,
        };

        let mut buffer = BufWriter::with_capacity(size, file);
        for result in results {
            if args.pretty_print {
                write!(buffer, "{}", result.pretty())?;
            } else {
                write!(buffer, "{result}")?;
            }
        }

        buffer.flush()?;
    }

    Ok(())
}
