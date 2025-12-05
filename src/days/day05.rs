use std::collections::HashMap;
use std::time::Instant;
// use regex::Regex;

////////////////////////////////////////////////////////////////
// Advent of Code 2025 Day 5
// Link: <a href="...">https://adventofcode.com/2025/day/5</a>
// 5 December 2025
////////////////////////////////////////////////////////////////

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn run() {
    println!("AoC 2025 Day 5");

    // Read the puzzle data file contents into a string
    // let filename = "./inputs/day05-test.txt";
    let filename = "./inputs/day05.txt";

    // Read the puzzle data file contents into a string
    let input = std::fs::read_to_string(filename).expect("Failed to read input file for Day 5");

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

    // puzzle seems simple enough???
    // for each row of data, if it is a range (has a dash) parse and to a HashMap
    // else it is a test value.  Add test values to a vector
    let mut ingredient_id_ranges: HashMap<u64, u64> = HashMap::new();
    let mut ingredients: Vec<u64> = Vec::new();
    for (row, line) in input_vec.iter().enumerate() {
        if line.contains("-") {
            let r = line.split("-").collect::<Vec<&str>>();
            let r1 = r[0].parse::<u64>().unwrap();
            let r2 = r[1].parse::<u64>().unwrap();
            if !ingredient_id_ranges.contains_key(&r1) {
                ingredient_id_ranges.insert(r1, r2);
            }
            else {
                println!("Collision occupied! {:?}, {:?}", r1, r2);

                let value = match ingredient_id_ranges.get(&r1) {
                    Some(value) => value,
                    None => { panic!("Key not found") },
                };
                // either contained within - no action needed
                // or overlap right - adjust value
                if r2 > *value {
                    // adjust value
                    ingredient_id_ranges.insert(r1,r2);
                    println!("Updated entry {:?}, {:?}", r1, r2);
                } else {
                    println!("No action needed {:?}, {:?}", r1, r2);
                }
            }
        } else {
            ingredients.push(line.parse::<u64>().unwrap());
        }
    }

    let mut freshs: Vec<u8> = Vec::new();
    let mut keys: Vec<u64> = ingredient_id_ranges.keys().map(|k| k.clone()).collect();
    keys.sort();
    for ingredient in ingredients.iter() {
        // find all the keys (start of range) less than or equal to test value, this ingredient
        let ks = keys.iter().take_while(|x| x <= &ingredient);

        // for each candidate key, see if this ingredient falls within range
        for k in ks {
            let range_ending_value = ingredient_id_ranges.get(k).unwrap();
            if  ingredient <= range_ending_value {
                freshs.push(1 as u8);
                // once found, break out we're done, don't count more than once
                break;
            }
        }
    }

    let answer_p1 = freshs.iter().map(|x| x.clone() as u64).sum::<u64>();
    println!("Part 1 answer {}", answer_p1);
    let lap1 = stop_watch.elapsed();
    println!("Elapsed time part 1: {:.2?}\n", lap1);

    //////////
    // Part 2
    //////////

    // for each (key, value) does it overlap with an existing range?
    let mut ingredient_ranges_clean: HashMap<u64, u64> = HashMap::new();
    for k in keys.iter() {
        for start in ingredient_ranges_clean.keys() {
            if k >= start {
                let range_ending_value = ingredient_ranges_clean.get(k).unwrap();
            }
        }
    }

    let answer_p2 = 0;
    println!("Part 2 answer {answer_p2}");
    println!("Elapsed time part 2: {:.2?}", stop_watch.elapsed() - lap1);

    println!("\nTotal elapsed runtime: {:.2?}", stop_watch.elapsed());
}
