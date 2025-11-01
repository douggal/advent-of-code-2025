use std::time::Instant;
// use regex::Regex;

////////////////////////////////////////////////////////////////
// Advent of Code 2025 Day XX
// Link: <a href="...">https://adventofcode.com/2022/day/XX</a>
// XX December 2025
////////////////////////////////////////////////////////////////


#[allow(dead_code)]
#[allow(unused_variables)]
pub fn run() {
    println!("Day XX");

    // Read the puzzle data file contents into a string
    let input = std::fs::read_to_string("inputs/day01.txt")
        .expect("Failed to read input file for Day XX");

    // Cast the input as a Vector<String> with leading and trailing
    // whitespace trimmed, or as best suites each puzzle
    let input_vec = Vec::from(
        input
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .collect::<Vec<&str>>()
    );

    // Debug:  Visually validate the puzzle input: Check for missing first and/or last row, etc!
    // dbg!(&input);
    println!("Input data as Vector:\n{}", input_vec.join("\n"));

    // Track program runtime by elapsed time as shown by a "clock on the wall"
    let stop_watch = Instant::now();

    // Part 1
    let answer_p1 = 0;
    println!("Part 1.  answer...  {answer_p1}");
    let lap1 = stop_watch.elapsed();
    println!("Elapsed time part 1: {:.2?}", lap1);
    println!();


    // Part 2
    let answer_p2 = 0;
    println!("Part 2. answer ... {answer_p2}");
    println!("Elapsed time part 2: {:.2?}", stop_watch.elapsed()-lap1);

    println!("\nTotal elapsed runtime: {:.2?}", stop_watch.elapsed());

}

