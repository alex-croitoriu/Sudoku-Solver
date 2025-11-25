<h1 align="center">
  Sudoku solver
</h1>

This project brings together my passion for Sudoku and my desire to learn a new programming language - Rust. Despite my initial unfamiliarity, I discovered that Rust is not only powerful and exceptionally fast but also really enjoyable to program in.

## Approach

On the algorithmics side, the solver uses optimized backtracking to progressively solve grids. It keeps track of all empty cells and makes use of bitmasks to instantly fetch all the possible candidates for a each cell. 

It also uses the MRV (Minimum Remaining Values) heuristic which prioritizes selecting the cells with fewer candidates first, greatly reducing the number of possibilities that need to be explored in the search tree. With some optimizations such as pruning early if there are 0 candidates remaining for a cell or selecting the first cell with exactly 1 candidate, MRV proves to be an improvement in all cases.

Using multithreading is a no-brainer when it comes to large inputs that can easily be parallelized. But because the performance increase doesn't come from a direct optimization in the code itself, some may consider multithreading to be cheaty. That's why I defaulted to a single thread and added multithreading as an option.

When it comes to the code itself, Rust makes it really easy to treat all errors and assure no undefined behavior, while keeping everything fast. The performance drop was insignificant after implementing strict validations, proper error handling and command line option parsing on top of the fast algorithm.

## Benchmarks
There are 4 main datasets I used for benchmarking: [easy](https://www.kaggle.com/datasets/bryanpark/sudoku), [medium](https://www.kaggle.com/datasets/rohanrao/sudoku), [hard](https://www.kaggle.com/datasets/radcliffe/3-million-sudoku-puzzles-with-ratings) and [17 clues](https://github.com/t-dillon/tdoku/blob/master/data.zip).

The following benchmarks were generated with the help of [hyperfine](https://github.com/sharkdp/hyperfine). The solver was ran 10 times on each dataset with the `--no-progress` option (because it's slightly faster).

<div align="center">
  <img width="1533" height="1080" alt="b1" src="https://github.com/user-attachments/assets/b801928a-33cc-4e5c-94cd-182d2102e902" />
  <img width="1517" height="1080" alt="b2" src="https://github.com/user-attachments/assets/de9c1dc3-0712-47e4-967a-c93e4b107967" />
  
  <br/>
  <br/>
  
  Raw data:
  | Dataset      | Single thread | Multithreading |
  |--------------|---------------|----------------|
  | Easy         | 622878        | 4776029        |
  | Medium       | 540866        | 3853361        |
  | Hard         | 81707         | 560216         |
  | 17 clues     | 1110          | 7508           |
</div>

## Format

Each input grid has to respect the following format:

- Must be a string with length 81
- Must use '0' to denote empty cells and digits from '1' to '9' for cells that are already solved
- Must be a valid sudoku puzzle with at least one solution

Example: **004300209005009001070060043006002087190007400050083000600000105003508690042910300**

## Options
The solver accepts several command-line arguments:
| Option | Short | Description |
|--------|-------|-------------|
| `--input-file <INPUT_FILE>` | `-i` | Path to input file containing grids |
| `--output-file <OUTPUT_FILE>` | `-o` | Path to output file for writing results |
| `--single <GRID>` | `-s` | Pass a single grid directly |
| `--multithreading [<THREAD_COUNT>]` | `-m` | Use multithreading (uses all CPUs if value not specified) |
| `--max-grids <MAX_GRIDS>` | `-g` | Maximum number of grids to solve (solves all if not specified) |
| `--format` | `-f` | Write results in human-readable format |
| `--no-stats` | | Hide stats (execution time and average grids/second) |
| `--no-progress` | | Hide progress bar |
| `--help` | `-h` | Print help |

Examples:

```bash 
# Reads from dataset.txt, outputs to output.txt, uses 4 threads and writes results in human-readable format
./solver -i dataset.txt -o output.txt -m 4 -f 
# Reads from dataset.txt, doesn't dump output, solves maximum 1000 grids and doesn't print the progress bar
./solver -i dataset.txt -g 1000 --no-progress 
# Solves the grid directly
./solver -s "000000000000000001000002030000003020001040000005000060030000004070080009620007000" 
```
