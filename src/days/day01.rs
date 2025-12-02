use std::time::Instant;
// use regex::Regex;


////////////////////////////////////////////////////////////////
// Advent of Code 2025 Day 1
// Link: <a href="...">https://adventofcode.com/2025/day/1</a>
// 1 December 2025
////////////////////////////////////////////////////////////////

pub fn run() {
    println!("AoC 2025 Day 1");

    // let filename = "./inputs/day01-test.txt";
    let filename = "./inputs/day01.txt";

    // Read the puzzle data file contents into a string
    let input = std::fs::read_to_string(filename)
        .expect("Failed to read input file for Day 1");

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
    // Debug:  Visually validate the puzzle input: Check for missing first and/or last row, etc!
    println!("Raw input: {:?}", input);
    println!("Input as Vec<String>:\n{}", input_vec.join("\n"));
    println!("End input inspection\n");


    // Track program runtime by elapsed time shown by a "clock on the wall"
    let stop_watch = Instant::now();

    //////////
    // Part 1
    //////////

    // Iterate over the input vector of String
    let max:i32 = 100;
    let mut result_vec: Vec<i32> = vec![];
    result_vec.push(50);
    for (row, line) in input_vec.iter().enumerate() {
        let (direction, count) = line.split_at(1);
        let count: i32 = count.parse().unwrap();
        let count = count % max;

        let dial_points_to = result_vec[result_vec.len() - 1];
        match direction {
            "R" => {
                result_vec.push((dial_points_to + count) % max);
            }
            "L" => {
                if (dial_points_to - count) >= 0 {
                    result_vec.push(dial_points_to - count);
                } else {
                    result_vec.push(max - (dial_points_to - count).abs());
                }
            }
            _ => eprintln!("Error on line {}", row),
        }

    }

    let answer_p1 = result_vec.iter().filter(|&&v| v == 0).count();
    println!("Part 1 answer: {}", answer_p1);
    let lap1 = stop_watch.elapsed();
    println!("Elapsed time part 1: {:.2?}", lap1);
    println!();


    //////////
    // Part 2
    //////////
    let answer_p2 = 0;
    println!("Part 2. answer ... {answer_p2}");
    println!("Elapsed time part 2: {:.2?}", stop_watch.elapsed()-lap1);

    println!("\nTotal elapsed runtime: {:.2?}", stop_watch.elapsed());
}

