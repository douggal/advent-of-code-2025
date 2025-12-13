use std::collections::{HashMap};
use std::time::Instant;

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
    // #[cfg(debug_assertions)]
    // {
    //     println!("Input file: {}", filename);
    //     println!("Lines of data in input: {}", input_vec.len());
    //     //println!("Raw input: {:?}", input);
    //     println!("Input as Vec<String>:\n{}", input_vec.join("\n"));
    //     println!("End input inspection\n");
    // }

    // Track program runtime by elapsed time as shown by a "clock on the wall"
    let stop_watch = Instant::now();

    //////////
    // Part 1
    //////////

    // puzzle seems simple enough???
    // for each row of data, if it is a range (has a dash) parse and to a HashMap
    // else it is a test value.  Add ID  to a test values vector
    let mut id_ranges: HashMap<u64, u64> = HashMap::new();  // ingredient ID integer ranges
    let mut ingredients: Vec<u64> = Vec::new();   // ID numbers of the ingredients to check
    for (row, line) in input_vec.iter().enumerate() {
        if line.contains("-") {
            let r = line.split("-").collect::<Vec<&str>>();
            let r1 = r[0].parse::<u64>().unwrap();
            let r2 = r[1].parse::<u64>().unwrap();
            if !id_ranges.contains_key(&r1) {
                id_ranges.insert(r1, r2);
            }
            else {
                // println!("Collision detected (start value appears more than once)! {:?}, {:?}", r1, r2);
                let value = match id_ranges.get(&r1) {
                    Some(value) => value,
                    None => { panic!("Key not found") },
                };
                // either contained within - no action needed
                // or overlap right - adjust value
                if r2 > *value {
                    // adjust value
                    id_ranges.insert(r1, r2);
                    // println!("Updated entry {:?}, {:?}", r1, r2);
                } else {
                    // println!("No action needed {:?}, {:?}", r1, r2);
                }
            }
        } else {
            ingredients.push(line.parse::<u64>().unwrap());
        }
    }

    let mut freshs: Vec<u8> = Vec::new();  // place in which to accumulate IDs of the fresh ingredients
    let mut keys: Vec<u64> = id_ranges.keys().map(|k| k.clone()).collect();
    keys.sort();
    for ingredient in ingredients.iter() {
        // find all the keys (start of range) less than or equal to test value, this ingredient
        let ks = keys.iter().take_while(|x| x <= &ingredient);

        // for each candidate key, see if this ingredient falls within range
        for k in ks {
            let range_ending_value = id_ranges.get(k).unwrap();
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

    // Stack of tuples (u64, u64)
    let mut input_stack: Vec<(u64, u64)> = Vec::new();
    let mut ordered_stack: Vec<(u64, u64)> = Vec::new();

    let mut keys: Vec<u64> = id_ranges.keys().map(|k| k.clone()).collect();
    keys.sort();

    for key in &keys {
        println!("{} {}", key, id_ranges.get(&key).unwrap());
        input_stack.push((*key, *id_ranges.get(key).unwrap()));
    }

    dbg!(&input_stack);
    if input_stack.is_empty() { panic!("Input stack empty"); }

    loop {
        let test = input_stack.pop().clone().unwrap();

        if ordered_stack.is_empty() {
            ordered_stack.push(test);
        } else {
            let stack_top = ordered_stack.pop().clone().unwrap();
            if test.0 < stack_top.0 && test.1 > stack_top.1 {
                // span new tuple encompasses the old one
                // check if span more than one range, then pop until
                if !ordered_stack.is_empty() {
                    loop {
                        ordered_stack.pop().clone().unwrap();
                        if ordered_stack.is_empty() || test.1 < ordered_stack.last().unwrap().1 {
                            break;
                        }
                    }
                }
                ordered_stack.push(test);
            } else if test.0 < stack_top.0 && test.1 < stack_top.0 {
                // outside and to the left, push new item
                ordered_stack.push(stack_top);
                ordered_stack.push(test);
            } else if test.0 < stack_top.0 && test.0 <= stack_top.1 && test.1 <= stack_top.1 {
                // overlap left
                ordered_stack.push((test.0, stack_top.1));
            } else if test.0 > stack_top.0 && test.0 <= stack_top.1 && test.1 >= stack_top.1 {
                // overlap right
                ordered_stack.push((stack_top.0, test.1));
            } else if test.0 >= stack_top.0 && test.1 <= stack_top.1 {
                //between
                ordered_stack.push(stack_top);  // push same tuple back on stack
            } else /* if test.0 > stack_top.1 && test.1 >= stack_top.1 */ {
                // outside and to the right (bigger than any range)
                ordered_stack.push(test);
                ordered_stack.push(stack_top);
            }
        }

        if input_stack.len() == 0 {
            break;
        }
    }

    dbg!(&ordered_stack);

    let mut sum:u64 = 0;
    for (s, e) in ordered_stack.iter() {
        if let Some(diff) = e.checked_sub(*s) {
            if let Some(new_sum) = sum.checked_add(diff +1) {
                sum = new_sum;
            } else {
                println!("Overflow occurred while adding to sum.");
            }
        } else {
            println!("Underflow occurred during subtraction.");
        }
    }
    println!("Part 2 answer {}", sum);
    println!("Elapsed time part 2: {:.2?}", stop_watch.elapsed() - lap1);

    println!("\nTotal elapsed runtime: {:.2?}", stop_watch.elapsed());
}
