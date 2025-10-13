use std::time::Instant;
// use regex::Regex;

/// Advent of Code 2025 Day 1
/// 1 December 2025
/// Solution

pub fn run() {
    println!("Day 1");

    // Read file contents into a string
    let input = std::fs::read_to_string("inputs/day01-test.txt")
        .expect("Failed to read input file for Day XX");

    // Cast the input as a Vector<String> or as best suites each puzzle
    let input_vec = Vec::from(
        input
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .collect::<Vec<&str>>()
    );

    // Debug:  Visually validate the input: Check for missing first and/or last row, etc!
    // dbg!(&input);
    println!("{}", input_vec.join("\n"));


    // Track program runtime by "clock on the wall"
    let now = Instant::now();

    // Part 1
    let answer_p1 = 0;
    println!("Part 1.  answer...  {answer_p1}");
    println!("Elapsed time part 1: {:.2?}", now.elapsed());
    println!();


    // Part 2
    let answer_p2 = 0;
    println!("Part 2. answer ... {answer_p2}");
    println!("Elapsed time part 2: {:.2?}", now.elapsed());

}

