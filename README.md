# Advent of Code 2025 - Rust Runner

This project is a scaffold for solving [Advent of Code](https://adventofcode.com/) challenges in Rust.

## Features
- Command-line interface to run solutions for days 1 to 25
- Organized module structure for each day's solution
- Input files stored in `inputs/`
- Unit test support in `tests/`

## Usage
```bash
cargo run -- <day>
```
Example:
```bash
cargo run -- 1
```
This will run the solution for Day 1 using the input file `inputs/day01.txt`.

## Setup
```bash
cargo build
```
Add your solutions in `src/days/dayXX.rs` and input files in `inputs/dayXX.txt`.

## License
This project is licensed under the MIT License.
