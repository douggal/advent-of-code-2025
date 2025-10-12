use std::time::Instant;
use chrono::Utc;
use regex::Regex;


pub fn run() {
    println!("--- Day 1: ... ---\n");

    let input = std::fs::read_to_string("inputs/day01.txt")
        .expect("Failed to read input file for Day 1");

    // Check input was correctly read in.  Look for first and last values!
    dbg!(&input);
    println!("Day 1 input: {}", input.trim());

    // regex to match a single digit 0 to 9
    let re_digit = Regex::new(r"[0-9]").unwrap();

    // Track program runtime by "clock on the wall"
    let now = Instant::now();

    // part 1
    let answer_p1 = 0;
    println!("Day 01 Part 1.  answer...  {answer_p1}");
    let elapsed = now.elapsed();
    println!("Elapsed time part 1: {:.2?}", elapsed);


    // part 2
    let answer_p2 = 0;
    println!("Day 01 Part 2. answer ... {answer_p2}");


    // End
    let current_datetime = Utc::now();
    println!(
        "End.  Current date and time (UTC): {}",
        current_datetime.format("%Y-%m-%d %H:%M:%S")
    );
}

