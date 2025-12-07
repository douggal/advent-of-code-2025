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
            .map(|line| line.trim_end_matches('\n').trim_end_matches('\r'))
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

    let num_operands = input_vec.len() - 1;

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

    let mut answers: Vec<u64> = Vec::new();

    for (idx, op) in operations.iter().enumerate() {
        let mut operands: Vec<u64> = vec![];
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

    let mut sum: u64 = 0;
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
    let mut x: Vec<Vec<char>> = Vec::new();
    for (row, line) in input_vec.iter().enumerate() {
        if !(line.contains("+") && line.contains("*")) {
            let digits: Vec<char> = line.chars().collect();
            // println!("{:?}", digits);
            x.push(digits);
        }
    }

    let mut numbers_pt2: Vec<Vec<u64>> = Vec::new();
    let mut new_nbrs: Vec<u64> = Vec::new();
    for i in 0..x[1].len() {

        let mut ds:Vec<String> = vec![];

        ds.push(x[0].pop().unwrap().to_string());
        ds.push(x[1].pop().unwrap().to_string());
        ds.push(x[2].pop().unwrap().to_string());
        ds.push(x[3].pop().unwrap().to_string());

        let n = ds.join("");
        let trimmed = n.trim();

        // if at least 1 digit is not empty
        if !trimmed.to_string().is_empty()  {
            let parsed: Result<u64, _> = trimmed.parse();
            let num =  match parsed {
                Ok(num) => num,
                Err(_) => {eprintln!("Invalid input: '{}'", input); 0 as u64},
            };
            new_nbrs.push(num);
        } else {
            numbers_pt2.push(new_nbrs.clone());
            new_nbrs.clear();
        }

        // if we are at very last column of data...push last numbers onto vec
        if x[0].is_empty() {
            numbers_pt2.push(new_nbrs.clone());
        }
    }

    let mut answers_pt2: Vec<u64> = Vec::new();

    for (idx, op) in operations.iter().rev().enumerate() {
        let mut operands: Vec<u64> = vec![];
        for i in 0..numbers_pt2[idx].len() {
            // println!("{}, {}", idx, i);
            operands.push(numbers_pt2[idx][i]);
        }

        match *op {
            "+" => {
                answers_pt2.push(operands.iter().sum::<u64>());
            }
            "*" => {
                answers_pt2.push(operands.iter().product::<u64>());
            }
            _ => eprintln!("Unknown operation {}", op),
        }
    }

    // Add number up and watch out for possible overflow condition while doing so
    let mut sum_pt2: u64 = 0;
    for answer in answers_pt2 {
        if let Some(total) = sum_pt2.checked_add(answer) {
            sum_pt2 = total;
        } else {
            println!("Overflow occurred while adding up the answers for part 2.");
        }
    }

    println!("Part 2 answer {}", sum_pt2);
    println!("Elapsed time part 2: {:.2?}", stop_watch.elapsed() - lap1);

    println!("\nTotal elapsed runtime: {:.2?}", stop_watch.elapsed());
}
