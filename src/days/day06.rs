use std::time::Instant;

////////////////////////////////////////////////////////////////
// Advent of Code 2025 Day 6
// Link: <a href="...">https://adventofcode.com/2025/day/6</a>
// 6 December 2025
////////////////////////////////////////////////////////////////

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn run() {
    println!("AoC 2025 Day 6");

    // Read the puzzle data file contents into a string
    // let filename = "./inputs/day06-test.txt";
    let filename = "./inputs/day06.txt";

    // Read the puzzle data file contents into a string
    let input = std::fs::read_to_string(filename).expect("Failed to read input file for Day 6");

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

    let num_operands = input_vec.len()-1;

    let mut numbers: Vec<Vec<u64>> = Vec::new();
    for (row, line) in input_vec.iter().enumerate() {
        if !(line.contains("+") && line.contains("*")) {
            let l = line.split_whitespace().collect::<Vec<&str>>();
            let m = l
                .iter()
                .map(|s| s.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            numbers.push(m);
        }
    }
    let operations = input_vec
        .iter()
        .last()
        .unwrap()
        .split_whitespace()
        .collect::<Vec<&str>>();

    let mut answers:Vec<u64> = Vec::new();

    for (idx, op) in operations.iter().enumerate() {
        let mut operands:Vec<u64> = vec![];
        for i in 0..numbers.len() {
            operands.push(numbers[i][idx]);
        }

        match *op {
            "+" => {
                answers.push(operands.iter().sum::<u64>());
            }
            "*" => {
                answers.push(operands.iter().product::<u64>());
            }
            _ => eprintln!("Unknown operation {}", op),
        }
    }

    let mut sum:u64 = 0;
    for answer in answers {
        if let Some(total) = sum.checked_add(answer) {
            sum = total;
        } else {
            println!("Overflow occurred while adding up the answers.");
        }
    }

    println!("Part 1 answer {}", sum);
    let lap1 = stop_watch.elapsed();
    println!("Elapsed time part 1: {:.2?}\n", lap1);

    //////////
    // Part 2
    //////////
    let answer_p2 = 0;
    println!("Part 2 answer {}", answer_p2);
    println!("Elapsed time part 2: {:.2?}", stop_watch.elapsed() - lap1);

    println!("\nTotal elapsed runtime: {:.2?}", stop_watch.elapsed());
}
