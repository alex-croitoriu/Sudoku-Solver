use clap::Parser;

#[derive(Parser)]
#[command(name = "Sudoku Solver")]
pub struct Cli {
    /// Path to input file containing grids
    pub input_file: String,

    /// Path to output file for solved grids
    #[arg(group = "output")]
    pub output_file: Option<String>,

    /// Maximum number of grids to solve (will solve all grids if not specified)
    #[arg(short = 'g', long)]
    pub grid_limit: Option<usize>,

    /// Maximum number of threads to use (single-threaded if not specified)
    #[arg(short = 't', long)]
    pub threads: Option<usize>,

    /// Print grids in human-readable format
    #[arg(short, long, default_value_t = false, requires = "output")]
    pub pretty_print: bool,

    /// Print execution time and average grids/second
    #[arg(short, long, default_value_t = false)]
    pub stats: bool,
}
