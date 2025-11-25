use anyhow::{Result, anyhow};
use clap::Parser;
use indicatif::ProgressBar;
use rayon::prelude::*;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::time::{Duration, Instant};

mod args;
mod sudoku;

use crate::args::Cli;
use crate::sudoku::{Status, Sudoku};

fn write_results_to_file(output_file: &String, results: Vec<Status>, args: &Cli) -> Result<()> {
    let file = match File::create(output_file) {
        Ok(file) => file,
        Err(_) => Err(anyhow!("File '{output_file}' not found"))?,
    };
    let mut buffer = BufWriter::with_capacity(results.len(), file);
    let bar = if args.no_progress {
        ProgressBar::hidden()
    } else {
        ProgressBar::new(results.len() as u64)
    };

    println!("\nWriting results to output file..");
    for result in results {
        if args.format {
            write!(buffer, "{}", result.format())?;
        } else {
            write!(buffer, "{result}")?;
        }
        bar.inc(1);
    }
    buffer.flush()?;
    bar.finish_and_clear();

    Ok(())
}

fn solve_from_file(input_file: &String, args: &Cli) -> Result<(Vec<Status>, Duration)> {
    let content = std::fs::read_to_string(input_file)
        .map_err(|_| anyhow!("File '{input_file}' not found"))?;
    let lines = content
        .lines()
        .take(args.max_grids.unwrap_or(usize::MAX))
        .collect::<Vec<&str>>();

    let results: Vec<Status>;

    let bar = if args.no_progress {
        ProgressBar::hidden()
    } else {
        ProgressBar::new(lines.len() as u64)
    };

    let handler = |(i, line): (usize, &&str)| match Sudoku::new(line.trim()) {
        Ok(mut sudoku) => match sudoku.solve(0) {
            Some(grid) => {
                bar.inc(1);
                Status::Solved(grid)
            }
            None => {
                let status = Status::Unsolved(Some(i + 1));
                if bar.is_hidden() {
                    println!("{status}");
                } else {
                    bar.println(format!("{status}"));
                }
                status
            }
        },
        Err(e) => {
            let status = Status::Invalid(Some(i + 1), e.to_string());
            if bar.is_hidden() {
                println!("{status}");
            } else {
                bar.println(format!("{status}"));
            }
            status
        }
    };

    let start = Instant::now();

    println!("Solving grids from '{input_file}'..");
    if let Some(thread_count) = args.multithreading {
        rayon::ThreadPoolBuilder::new()
            .num_threads(thread_count)
            .build_global()?;
        results = lines.par_iter().enumerate().map(handler).collect();
    } else {
        results = lines.iter().enumerate().map(handler).collect();
    }

    bar.finish_and_clear();
    let execution_time = start.elapsed();

    Ok((results, execution_time))
}
fn main() -> Result<()> {
    let args = Cli::parse();

    if let Some(input_file) = &args.input_file {
        let (results, execution_time) = solve_from_file(input_file, &args)?;
        let solved_grids_count = results
            .iter()
            .filter(|s| matches!(s, Status::Solved(_)))
            .count();

        println!("\nGrids solved: {solved_grids_count}/{}", results.len());
        if !args.no_stats {
            println!("Execution time: {} seconds", execution_time.as_secs_f32());
            println!(
                "Average grids/second: {}",
                solved_grids_count as f32 / execution_time.as_secs_f32()
            );
        }

        if let Some(output_file) = &args.output_file {
            write_results_to_file(output_file, results, &args)?;
        }
    }

    if let Some(single) = &args.single {
        let status = match Sudoku::new(single) {
            Ok(mut sudoku) => match sudoku.solve(0) {
                Some(grid) => Status::Solved(grid),
                None => Status::Unsolved(None),
            },
            Err(e) => Status::Invalid(None, e.to_string()),
        };

        if args.format {
            println!("{}", status.format());
        } else {
            println!("{status}");
        }
    }

    Ok(())
}
