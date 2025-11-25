use clap::{ArgGroup, Parser};

#[derive(Parser)]
#[command(name = "Sudoku Solver")]
#[command(group(ArgGroup::new("input").args(["input_file", "single"]).required(true).multiple(false)))]
#[command(group(ArgGroup::new("format_target").args(["output_file", "single"]).multiple(true)))]
pub struct Cli {
    /// Path to input file containing grids
    #[arg(short, long)]
    pub input_file: Option<String>,

    /// Path to output file for writing results
    #[arg(short, long)]
    pub output_file: Option<String>,

    /// Pass a single grid directly
    #[arg(short, long, value_name = "GRID")]
    pub single: Option<String>,

    /// Use multithreading (uses all cpus if value not specified)
    #[arg(short, long, num_args = 0..=1, value_name = "THREAD_COUNT", default_missing_value = "0")]
    pub multithreading: Option<usize>,

    /// Maximum number of grids to solve (solves all if not specified)
    #[arg(short = 'g', long)]
    pub max_grids: Option<usize>,

    /// Write results in human-readable format
    #[arg(short, long, default_value_t = false, requires = "format_target")]
    pub format: bool,

    /// Hide stats (execution time and average grids/second)
    #[arg(long, default_value_t = false)]
    pub no_stats: bool,

    /// Hide progress bar
    #[arg(long, default_value_t = false)]
    pub no_progress: bool,
}
