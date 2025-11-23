use anyhow::{Result, anyhow};
use clap::Parser;
use rayon::prelude::*;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::time::Instant;

mod args;
mod sudoku;

use crate::args::Cli;
use crate::sudoku::{Status, Sudoku};

fn main() -> Result<()> {
    let args = Cli::parse();

    let content = std::fs::read_to_string(&args.input_file)
        .map_err(|_| anyhow!("File '{}' not found", args.input_file))?;
    let lines = content
        .lines()
        .take(args.grid_limit.unwrap_or(usize::MAX))
        .collect::<Vec<&str>>();

    let solve_closure = |(i, line): (usize, &&str)| match Sudoku::new(line.trim()) {
        Ok(mut sudoku) => match sudoku.solve(0) {
            Some(grid) => Status::Solved(grid),
            None => Status::Unsolved(i + 1),
        },
        Err(e) => Status::Invalid(i + 1, e.to_string()),
    };

    let results: Vec<Status>;

    let start = Instant::now();

    if let Some(threads) = args.threads {
        rayon::ThreadPoolBuilder::new()
            .num_threads(threads)
            .build_global()?;

        results = lines.par_iter().enumerate().map(solve_closure).collect();
    } else {
        results = lines.iter().enumerate().map(solve_closure).collect();
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
        let mut buffer = BufWriter::new(file);
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
