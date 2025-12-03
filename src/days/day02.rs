use std::time::Instant;
// use regex::Regex;

////////////////////////////////////////////////////////////////
// Advent of Code 2025 Day 2
// Link: <a href="...">https://adventofcode.com/2025/day/2</a>
// 02 December 2025
////////////////////////////////////////////////////////////////

// #[allow(dead_code)]
#[allow(unused_variables)]
pub fn run() {
    println!("AoC 2025 Day 2");

    // Read the puzzle data file contents into a string
    // let filename = "./inputs/day02-test.txt";
    let filename = "./inputs/day02.txt";

    // Read the puzzle data file contents into a string
    let input = std::fs::read_to_string(filename)
        .expect("Failed to read input file for Day 02");

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
    let mut bad_id_numbers:Vec<u64> = vec![];
    for range_text in input_vec[0].split(",") {
        let this_range = range_text.split("-").collect::<Vec<&str>>();
        let range_start:u64 = this_range[0].parse().unwrap();
        let range_end:u64 = this_range[1].parse().unwrap();
        // println!("Ranges parsed and ready: {}..={}", range_start, range_end);

        for i in range_start..=range_end {
            let i_as_str = i.to_string();
            let str_len = i_as_str.len();
            if str_len.is_multiple_of(2) {
                let (front, back) = i_as_str.split_at(str_len / 2);
                if front == back {
                    bad_id_numbers.push(i);
                }
            }
        }
    }

    // with test data result should be ...
    // assert_eq!(bad_id_numbers.iter().sum::<u64>(),1227775554);

    let answer_p1 = bad_id_numbers.iter().sum::<u64>();
    println!("Part 1 answer {}", answer_p1);
    let lap1 = stop_watch.elapsed();
    println!("Elapsed time part 1: {:.2?}", lap1);

    //////////
    // Part 2
    //////////

    let mut bad_id_numbers2:Vec<u64> = vec![];
    for range_text in input_vec[0].split(",") {
        let this_range = range_text.split("-").collect::<Vec<&str>>();
        let range_start:u64 = this_range[0].parse().unwrap();
        let range_end:u64 = this_range[1].parse().unwrap();
        // println!("Ranges parsed and ready: {}..={}", range_start, range_end);

        for i in range_start..=range_end {
            let i_as_str = i.to_string();
            let str_len = i_as_str.len();

            let mut divisors:Vec<usize> = vec![];
            for d in 1..=str_len {
                if str_len.is_multiple_of(d) {
                    divisors.push(d);
                }
            }
            //println!("Divisors {}, {:?}", str_len, divisors);

            for slice_size in divisors {
                if str_len.is_multiple_of(slice_size as usize) {
                    let mut slices:Vec<&str> = vec![];
                    for i in (0..i_as_str.len()).step_by(slice_size as usize) {
                        slices.push(&i_as_str[i..i+slice_size]);
                    }
                    // println!("slices: {:?}", slices);
                    if slices.len() > 1 {
                        let first = slices[0];
                        let test = slices.iter().filter(|&s| *s == first).collect::<Vec<&&str>>();
                        if test.len() == slices.len() {
                            bad_id_numbers2.push(i);
                        }
                    }
                }
            }
        }
    }

    bad_id_numbers2.sort_unstable();
    bad_id_numbers2.dedup();

    // with test data result should be ...
    // assert_eq!(bad_id_numbers2.iter().sum::<u64>(),4174379265);

    let answer_p2 = bad_id_numbers2.iter().sum::<u64>();
    println!("Part 2 answer {}", answer_p2);
    println!("Elapsed time part 2: {:.2?}", stop_watch.elapsed() - lap1);

    println!("\nTotal elapsed runtime: {:.2?}", stop_watch.elapsed());
}
