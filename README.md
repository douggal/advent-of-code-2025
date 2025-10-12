# Advent of Code 2025 - Rust Runner

My Rust-lang solutions to the Advent of Code programming contest, December 2025.
This project is a scaffold with day's puzzle in its own module.

Advent of Code website:  [Advent of Code](https://adventofcode.com)

Solutions are in Rust lang v1.90+ (2021 Edition) unless otherwise noted.

[Rust language cheat sheet](https://cheats.rs/)

Some algorithms chosen were my attempt to exercise a new-to-me programming technique and may not be
an optimal solution.


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

## Notes on Rust
- Informal way to compute elapsed time link: [Stackoveflow](https://stackoverflow.com/questions/13322479/how-to-benchmark-programs-in-rust)
- Nifty way to replace chars in a string this using match expression. Link: [stackoverflow.com](https://stackoverflow.com/questions/34606043/how-do-i-replace-specific-characters-idiomatically-in-rust)
```rust
   let s:String = input.chars()
   .map(|x| match x {
   '\n' => ' ',
   _ => x
   }).collect();
```
- How to find overlapping matches.  Look ahead:  https://stackoverflow.com/questions/11430863/how-to-find-overlapping-matches-with-a-regexp

## Notes by Day

### --- Day 1: TBD ---

### My stats at the end of the official contest
As of end of day 25 December 2025:
- ??? stars,
- and total of ??? with at least one part complete.
