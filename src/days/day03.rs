use std::time::Instant;

////////////////////////////////////////////////////////////////
// Advent of Code 2025 Day 3
// Link: <a href="...">https://adventofcode.com/2025/day/3</a>
// 3 December 2025
////////////////////////////////////////////////////////////////

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn run() {
    println!("AoC 2025 Day 3");

    // Read the puzzle data file contents into a string
    // let filename = "./inputs/day03-test.txt";
    let filename = "./inputs/day03.txt";

    // Read the puzzle data file contents into a string
    let input = std::fs::read_to_string(filename).expect("Failed to read input file for Day 3");

    // Cast the input as a Vector<String> with leading and trailing
    // whitespace trimmed, or as best suites each puzzle
    let input_vec = Vec::from(
        input
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .collect::<Vec<&str>>(),
    );

    // Debug:  Visually validate the puzzle input: Check for missing first and/or last row, etc!
    // dbg!(&input);
    // Debug:  Visually validate the puzzle input: Check for missing first and/or last row, etc!
    #[cfg(debug_assertions)]
    {
        println!("Input file: {}", filename);
        println!("Lines of data in input: {}", input_vec.len());
        //println!("Raw input: {:?}", input);
        println!("Input as Vec<String>:\n{}", input_vec.join("\n"));
        println!("End input inspection\n");
    }

    // Track program runtime by elapsed time as shown by a "clock on the wall"
    let stop_watch = Instant::now();

    //////////
    // Part 1
    //////////
    let mut joltages: Vec<u64> = vec![];
    for (row, line) in input_vec.iter().enumerate() {
        // find two biggest digits in each row,
        // combine them and turn into a integer:  that's the highest voltage
        let digits = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u64)
            .collect::<Vec<u64>>();

        let mut big = digits[0];
        let mut next_big = digits[1];

        for i in 0..digits.len()-1 {
            if big < digits[i] {
                big = digits[i];
                next_big = digits[i+1];
            }
            for j in i +1..digits.len() {
                // println!("D: {}, E: {}, Big {}, Next Big {}", digits[i], digits[j], big, next_big);
                if next_big < digits[j]  {
                    next_big = digits[j];
                }
            }
        }
        let j = big*10 + next_big;
            joltages.push(big*10+next_big);



    }
    // println!("{:?}", joltages);

    let answer_p1 = joltages.iter().sum::<u64>();
    println!("Part 1 answer {}", answer_p1);
    let lap1 = stop_watch.elapsed();
    println!("Elapsed time part 1: {:.2?}\n", lap1);

    //////////
    // Part 2
    //////////
    let answer_p2 = 0;
    println!("Part 2. answer ... {answer_p2}");
    println!("Elapsed time part 2: {:.2?}", stop_watch.elapsed() - lap1);

    println!("\nTotal elapsed runtime: {:.2?}", stop_watch.elapsed());
}
