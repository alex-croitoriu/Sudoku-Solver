<h1 align="center">
  Sudoku solver
</h1>

This project brings together my passion for Sudoku and my desire to learn a new programming language - Rust. Despite my initial unfamiliarity, I took the challenge and discovered that Rust is not only powerful and exceptionally fast but also really enjoyable to code in.

## 💡 Approach
The solver employs a **backtracking** algorithm to progressively solve a given puzzle. It keeps track of all the empty cells and makes use of **bitmasks** to instantly fetch all the possible candidates for a specific cell.

It can also be configured to use a **heuristic** known as **MRV** (Minimum Remaining Values) which prioritizes selecting the cells with fewer candidates first. When enabled, this option enhances the solver's efficieny by increasing the likelihood of finding a solution or identifying contradictions early in the process. 

**Theoretically**, this reduces the number of possibilities that need to be explored in the search tree, ultimately making the algorithm more efficient and faster. 

**In practice**, depending on the provided dataset, this option can either speed up the algorithm by as much as **1000x** (particularly for 17 clue grids with overall large candidates sets), or it might marginally affect performance (typically applies to easy grids, as the number of candidates for each cell is already very small, making the selection of the best cell almost redundant).

## 📊 Benchmarks
There are 3 main datasets I used for benchmarking:

| [Easy](https://www.kaggle.com/datasets/bryanpark/sudoku) | [Medium](https://www.kaggle.com/datasets/rohanrao/sudoku) | [Hard](https://www.kaggle.com/datasets/radcliffe/3-million-sudoku-puzzles-with-ratings) |
|------|--------|------|
|<img src="https://github.com/JellyGamez/Sudoku-solver/assets/75379898/3b8f0c11-93f0-4e9e-9306-4580fc6151dc" width="700px">|<img src="https://github.com/JellyGamez/Sudoku-solver/assets/75379898/daf66a41-7d3a-4400-b5ce-f9f92a175254" width="700px">|<img src="https://github.com/JellyGamez/Sudoku-solver/assets/75379898/a22eb02f-a04b-49df-8600-b79e508df0f4" width="700px">|

Additionally, I used [this dataset](https://github.com/t-dillon/tdoku/blob/master/data.zip), specifically the one which consists of only 17 clues puzzles (the smallest possible number of clues for a solvable sudoku puzzle).

**The following benchmarks represent the average results obtained from 5 distinct runs for each dataset across all versions.**

<div align="center">
  
<!--
| [1. Easy](https://www.kaggle.com/datasets/bryanpark/sudoku) |
|:--------:|
|<img src="https://github.com/JellyGamez/Sudoku-solver/assets/75379898/3b8f0c11-93f0-4e9e-9306-4580fc6151dc" width="500px">|

| [2. Medium](https://www.kaggle.com/datasets/rohanrao/sudoku) |
|:--------:|
|<img src="https://github.com/JellyGamez/Sudoku-solver/assets/75379898/daf66a41-7d3a-4400-b5ce-f9f92a175254" width="500px">|

| [3. Hard](https://www.kaggle.com/datasets/radcliffe/3-million-sudoku-puzzles-with-ratings) |
|:--------:|
|<img src="https://github.com/JellyGamez/Sudoku-solver/assets/75379898/a22eb02f-a04b-49df-8600-b79e508df0f4" width="500px">|
-->

## Graphs

<img src="https://github.com/JellyGamez/Sudoku-solver/assets/75379898/c4b9ba4d-d2cc-4c0a-8e95-9d353be72d56" width="700px">

<img src="https://github.com/JellyGamez/Sudoku-solver/assets/75379898/4aedca9f-55cf-41b4-8c17-3d7abe75129e" width="700px">

## Raw data

| Grids/second | v1.0      | v2.0      | v3.0      | v3.0 with heuristic |
|--------------|-----------|-----------|-----------|---------------------|
| Easy         | 142.255,70 | 405.282,80 | 461.203,11 | 229.084,10       |
| Medium       | 28.439,10  | 108.977,01 | 120.452,13 | 242.720,17       |
| Hard         | 263,97    | 1.145,40   | 1.317,99   | 36.148,05         |
| 17 clues     | 0,04      | 0,19      | 0,24      | 559,49              |

</div>

## 📋 Update log
- **v1.0** - Plain backtracking with no optimizations
- **v1.1** - A few tweaks, including backtracking shortcuts and better compiler configuration

<!-- -->

- **v2.0** - Implemented bitmasks and proper display traits
- **v2.1** - Cleaned up code and refactored for performance
- **v2.2** - Implemented MRV heuristic

<!-- -->

- **v3.0** - Complete file restructure for improved readability and configurability

## ⚙️ Configuration

### Template
Each input grid has to respect the following template:

- It must be a string with a length of exactly 81 characters, representing the initial state of the puzzle
- It must use '0' to denote empty cells and digits from '1' to '9' for cells that are already solved
- The puzzle must be a valid sudoku puzzle with at least one solution
- Example: **004300209005009001070060043006002087190007400050083000600000105003508690042910300**


### Configuration file
Once you've prepared a file with your desired puzzles, navigate to the `src/config.rs` file and adjust your configuration settings as needed.

```rust
pub const FILE_NAME: &str = "example.txt";  // Input file path

pub const PRINT_STATS: bool = true;         // Option to print statistics after solving all grids

pub const NUMBER_OF_GRIDS: usize = 100;     // The number of grids to solve from the file

pub const PRINT_SOLVED_GRIDS: bool = true;  // Option to print solved grids to standard output

pub const MRV_HEURISTIC: bool = false;      // Option to enable fewer candidates first heuristic
```

## 🚀 Quick Start

### 1. Clone the project
```bash
git clone https://github.com/JellyGamez/Sudoku-solver.git
```

### 2. Go to the project directory
```bash
cd Sudoku-solver
```

### 3. Configure the project
[The configuration process is explained here](#configuration).
  
### 4. Launch the solver
```bash
cargo run --release
```
