use std::time::Instant;
// use regex::Regex;

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn run() {
    println!("Day XX");

    let input = std::fs::read_to_string("inputs/day01.txt")
        .expect("Failed to read input file for Day XX");

    // Check input was correctly read in.  Look for first and last values!
    // dbg!(&input);
    println!("Input data: {}", input.trim());

    // regex to match a single digit 0 to 9
    // let re_digit = Regex::new(r"[0-9]").unwrap();
    //
    // #[derive(Debug)]
    // struct MyStruct {
    //     name: String,
    //     value: i32,
    // }

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

