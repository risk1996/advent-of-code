# Advent of Code 🎄

Welcome to my **Advent of Code** solutions repository! Here you’ll find my solutions to the [Advent of Code](https://adventofcode.com/) challenges, organized by year and day.

## Project Structure

The repository follows this folder structure:
```
advent-of-code/
├── aoc/                       # Common library crate with shared utilities
│   ├── src/
│   │   └── lib.rs             # Shared functions and utilities for solutions
│   └── Cargo.toml             # Library crate configuration
├── YYYY/
│   ├── DD-TITLE/
│   │   ├── src/
│   │   │   └── main.rs        # Solution code for the day’s puzzle
│   │   ├── input.txt          # Input data for the day’s puzzle
│   │   └── Cargo.toml         # Day-specific package configuration
│   └── …
├── Cargo.toml                 # Top-level workspace configuration
├── .clippy.toml               # Configuration for Clippy, Rust’s linter
├── .rustfmt.toml              # Configuration for Rustfmt, Rust’s code formatter
└── README.md
```

### Explanation of Key Directories

- `aoc/`: The **common library crate** containing shared functions and utilities used across multiple solution files. Each day's solution can access these utilities by referencing the `aoc` crate.
- `YYYY/DD-TITLE/`: Each folder represents a specific year and day of the challenge, with subdirectories organized by the day and title of the puzzle.

  Inside each `DD-TITLE` folder:
  - `src/main.rs`: Contains the Rust solution code for that day.
  - `input.txt`: Holds the puzzle input data.
  - `Cargo.toml`: Specifies dependencies and configuration for that day's solution, including a dependency on the `aoc` crate.

The project root includes:
- `.clippy.toml`: Configures [Clippy](https://github.com/rust-lang/rust-clippy), Rust’s linter, to enforce coding standards, catch common mistakes, and enhance code readability and safety.
- `.rustfmt.toml`: Configures [Rustfmt](https://github.com/rust-lang/rustfmt), Rust’s automatic code formatter, to ensure consistent code style across the project.

## How to Use

1. Install Rust and Toolchain ([rustup](https://rustup.rs/) recommended)
2. Clone the Repository:
```bash
git clone https://github.com/risk1996/advent-of-code.git
```
3. Navigate to the Solution Folder:
```bash
cd advent-of-code/YYYY/DD-TITLE
```
4. Run the Solution with Cargo:
```bash
cargo run --release
```

## Technologies Used
Language: Rust

## Progress
| Year | Completion                |
| ---- | ------------------------- |
| 2015 | 🟩🟩🟩🟩🟩🟩🟩🟩🟩🟩🟩🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥 |
| 2016 | 🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥 |
| 2017 | 🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥 |
| 2018 | 🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥 |
| 2019 | 🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥 |
| 2020 | 🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥 |
| 2021 | 🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥 |
| 2022 | 🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥 |
| 2023 | 🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥 |
| 2024 | 🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥 |

```
🟩: Completed
🟨: On Progress
🟥: Not Done
```
