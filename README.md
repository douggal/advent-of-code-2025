# Advent of Code 2025

This repository contains my personal solutions to the [Advent of Code 2025](https://adventofcode.com) programming 
challenges, implemented in Rust.  Solutions are in Rust lang v1.90+ (2024 Edition) unless otherwise noted.

Each day's puzzle solution is organized into its own module.

> ⚠️ Puzzle descriptions and input files are not included due to copyright restrictions. Please visit the official Advent of Code website to view the original puzzles.

![Top language](https://img.shields.io/github/languages/top/douggal/advent-of-code-2025?color=orange&logo=rust&logoColor=white)
![License](https://img.shields.io/github/license/douggal/advent-of-code-2025)
![Last commit](https://img.shields.io/github/last-commit/douggal/advent-of-code-2025)
![Stars](https://img.shields.io/github/stars/douggal/advent-of-code-2025?style=social)
<!-- Optional: CI badge (replace `ci.yml` with your workflow filename if different)
![CI](https://img.shields.io/github/actions/workflow/status/douggal/advent-of-code-2025/ci.yml?branch=main)
-->

[Rust language cheat sheet](https://cheats.rs/)

Some algorithms chosen were my attempt to exercise a new-to-me programming technique and may not be
an optimal solution.

## Features
- Command-line interface to run solutions for days 1 to 12
- Organized module structure for each day's solution
- Input files stored in `inputs/`
- Unit test support in `tests/`

## My stats at the end of the official contest
As of end of day 12 December 2025:
- ??? stars,
- and total of ??? with at least one part complete.

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
## Folder structure
The layout below shows the recommended / existing organization and how to find inputs, 
shared utilities, and each day's solution.

```
/
├─ Cargo.toml                 # workspace or crate manifest
├─ README.md                  # this file
├─ inputs/                    # puzzle inputs (one file per day)
│  ├─ day01.txt
│  ├─ day02.txt
│  └─ ...
├─ src/                       # shared library code and helpers (optional)
│  ├─ lib.rs                  # reusable functions used by multiple days
│  └─ utils/                  # small helper modules
│     └─ mod.rs
├─ src/bin/                   # per-day binaries (common layout for AoC)
│  ├─ day01.rs                # Day 1 solution (binary)
│  ├─ day02.rs                # Day 2 solution
│  └─ ...
├─ days/                      # alternative layout: per-day crates (workspace)
│  ├─ day01/
│  │  └─ src/
│  │     └─ main.rs
│  └─ ...
└─ target/                    # build output (ignored by git)
```

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

## Daily Notes & Reflections

### [Day 1...](https://adventofcode.com/2025/day/1)

## License
This project is licensed under the MIT License.

Acknowledgements
----------------
- * Advent of Code (https://adventofcode.com)
- * Rust language and community resources
- * [Amit Patel’s Thoughts on Grids](https://www.redblobgames.com/)

