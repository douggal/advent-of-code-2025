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
    let filename = "./inputs/day06-test.txt";
    // let filename = "./inputs/day06.txt";

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
            println!("{:?}", digits);
            x.push(digits);
        }
    }

    let mut numbers_pt2: Vec<Vec<u64>> = Vec::new();
    let mut new_nbrs: Vec<u64> = Vec::new();
    for i in 0..x[1].len() {


        // canonicalize each line, that is, push digits to the left and blanks to the right
        // let mut canonical = x[i].into_iter().map(|x|  x.to_string()).collect::<Vec<String>>();
        // let num_spaces = canonical.into_iter().rev().take_while(|x| x !=" ").count();  //collect::<Vec<char>>().len();
        // remove spaces and canonicalize by pushing digits to the far side (right and higher index value)
        let mut ds:Vec<String> = vec![];

        ds.push(" ".to_string()); // x[0].pop().unwrap().to_string();
        ds.push(x[0].pop().unwrap().to_string());
        ds.push(x[1].pop().unwrap().to_string());
        ds.push(x[2].pop().unwrap().to_string());

        let mut canonical: Vec<String> = Vec::new();

        for d in 0..ds.iter().len() {
            if ds[d] != " " {
                canonical.push(ds[d].clone());
            }
        }

        if canonical.iter().len() < num_operands {
            let padding = num_operands - canonical.len();
            for i in 0..=padding {
                canonical.push(" ".to_string());
            }
        }

        dbg!(&canonical);

        // let thousands = x[0].pop().unwrap().to_string();
        // let hundreds = x[1].pop().unwrap().to_string();
        // let tens = x[2].pop().unwrap().to_string();
        // let ones = x[3].pop().unwrap().to_string();

        let thousands = &"0".to_string(); // &canonical[3];
        let hundreds = &canonical[0];
        let tens = &canonical[1];
        let ones = &canonical[2];

        let mut t: u64 = 0;
        let mut h: u64 = 0;
        let mut d: u64 = 0;
        let mut o: u64 = 0;
        if *thousands != " " {
            t = thousands.parse::<u64>().unwrap();
        }
        if *hundreds != " " {
            h = hundreds.parse::<u64>().unwrap();
        }
        if *tens != " " {
            d = tens.parse::<u64>().unwrap();
        }
        if *ones != " " {
            o = ones.parse::<u64>().unwrap();
        }

        if !( /* *thousands == " ".to_string() && */ *hundreds == " ".to_string() && *tens == " ".to_string() && *ones == " ".to_string()) {
            new_nbrs.push(1000 * t + 100 * h + 10 * d + o);
        } else {
            numbers_pt2.push(new_nbrs.clone());
            new_nbrs.clear();
        }
    }

    dbg!(&numbers_pt2);

    let mut answers_pt2: Vec<u64> = Vec::new();

    for (idx, op) in operations.iter().enumerate() {
        let mut operands: Vec<u64> = vec![];
        for i in 0..num_operands {
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
