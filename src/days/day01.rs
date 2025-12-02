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
    #[cfg(debug_assertions)]
    {
        println!("Input file: {}", filename);
        println!("Lines in input: {}", input_vec.len());
        //println!("Raw input: {:?}", input);
        println!("Input as Vec<String>:\n{}", input_vec.join("\n"));
        println!("End input inspection\n");
    }

    // Track program runtime by elapsed time shown by a "clock on the wall"
    let stop_watch = Instant::now();
    let max:i32 = 100;  // count of clicks in full rotation

    //////////
    // Part 1
    //////////

    // save the starting numeral and the numeral found at the end of each move
    let mut result_vec: Vec<i32> = vec![];
    result_vec.push(50);

    // Iterate over the input vector of String
    for (row, line) in input_vec.iter().enumerate() {
        let (direction, clicks) = line.split_at(1);
        let count_clicks: i32 = clicks.parse().unwrap();
        let count_clicks = count_clicks % max;

        let dial_points_to = result_vec[result_vec.len() - 1];
        match direction {
            "R" => {
                result_vec.push((dial_points_to + count_clicks) % max);
            }
            "L" => {
                if (dial_points_to - count_clicks) >= 0 {
                    result_vec.push(dial_points_to - count_clicks);
                } else {
                    result_vec.push(max - (dial_points_to - count_clicks).abs());
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

    // save the starting numeral and the numeral found at the end of each move
    let mut result_vec: Vec<i32> = vec![];
    result_vec.push(50);

    // Iterate over the input vector of String
    for (row, line) in input_vec.iter().enumerate() {
        let (direction, clicks) = line.split_at(1);
        let count_clicks: i32 = clicks.parse().unwrap();

        // the dial points to the result of last rotation
        let dial_points_to = result_vec[result_vec.len() - 1];
        match direction {
            "R" => {
                for i in 1..=count_clicks {
                    result_vec.push((dial_points_to + i) % max);
                }
            }
            "L" => {
                for i in 1..=count_clicks {
                    let i_relative = i % max;
                    if (dial_points_to - i_relative) >= 0 {
                        result_vec.push(dial_points_to - i_relative);
                    } else {
                        result_vec.push(max - (dial_points_to - i_relative).abs());
                    }
                }
            }
            _ => eprintln!("Error on line {}", row),
        }
    }
    // dbg!(&result_vec);
    let answer_p2 = result_vec.iter().filter(|&&v| v == 0).count();
    println!("Part 2 answer {}", answer_p2);
    println!("Elapsed time part 2: {:.2?}", stop_watch.elapsed()-lap1);

    println!("\nTotal elapsed runtime by 'clock on the wall' method: {:.2?}", stop_watch.elapsed());
}

