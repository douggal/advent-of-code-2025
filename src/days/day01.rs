use std::time::Instant;
// use regex::Regex;

#[allow(unused_variables)]
pub fn run() {
    println!("Day 1");

    let input = std::fs::read_to_string("inputs/day01.txt")
        .expect("Failed to read input file for Day 1");

    // Check input was correctly read in.  Look for complete first and last lines!
    // dbg!(&input);
    println!("Input data: {}", input.trim());
    println!();


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

