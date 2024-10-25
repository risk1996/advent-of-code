# Advent of Code 🎄

Welcome to my **Advent of Code** solutions repository! Here you’ll find my solutions to the [Advent of Code](https://adventofcode.com/) challenges, organized by year and day.

## Project Structure

The repository follows this folder structure:
```
advent-of-code/
├── YYYY/
│   ├── DD-TITLE/
│   │   ├── src/
│   │   │   └── main.rs         # Solution code for the day’s puzzle
│   │   ├── input.txt           # Input data for the day’s puzzle
│   │   ├── Cargo.toml          # Rust package configuration
│   │   └── README.md           # Explanation of the solution
│   └── …
├── .clippy.toml                # Configuration for Clippy, Rust’s linter
├── .rustfmt.toml               # Configuration for Rustfmt, Rust’s code formatter
└── README.md
```

Each folder represents a specific year and day of the challenge, with subdirectories named by the day and title of the puzzle. Inside each `day-DD-TITLE` folder:
- `src/main.rs`: Contains the Rust solution code.
- `input.txt`: Holds the puzzle input data.
- `Cargo.toml`: Manages dependencies and configuration.
- `README.md`: Includes a detailed explanation of the solution approach.

The project root includes:
- `.clippy.toml`: Configures [Clippy](https://github.com/rust-lang/rust-clippy), Rust’s linter, to enforce coding standards, catch common mistakes, and enhance code readability and safety.
- `.rustfmt.toml`: Configures [Rustfmt](https://github.com/rust-lang/rustfmt), Rust’s automatic code formatter, to ensure consistent code style across the project.

These configuration files help maintain clean, consistent, and idiomatic Rust code throughout the project.

This addition provides details on Clippy and Rustfmt to help users understand the purpose of each configuration file and ensure coding standards.

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
| 2015 | 🟩🟩🟩🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥🟥 |
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
