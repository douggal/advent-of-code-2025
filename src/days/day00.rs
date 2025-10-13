use std::time::Instant;
// use regex::Regex;
use aoc25;
use aoc25::add_number;
use log::debug;

/// Sample code to train up on Rust lang
/// and prepare for Advent of Code contest.

#[allow(unused_variables)]
#[allow(dead_code)]
#[allow(unused_assignments)]
pub fn run() {
    println!("Day 0: Rust lang train up");

    // Read file contents into a string
    let input = std::fs::read_to_string("inputs/day00-test.txt")
        .expect("Failed to read input file for Day XX");

    // Cast the input as a Vector<String> with whitespace trimmed, or as best suites each puzzle
    let input_vec = Vec::from(
        input
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .collect::<Vec<&str>>()
    );

    // Visually validate the input: Check for missing first and/or last row!
    // dbg!(&input);
    println!("{}", input_vec.join("\n"));


    // Track program runtime by "clock on the wall"
    let now = Instant::now();


    let my_tuple = ('A',1,2);
    let my_array = [1,2,3];
    let my_array_typed: [i32; 3] = [142,32,42];

    #[derive(Debug)]
    enum TempCategory {
        HOT(Option<f32>),
        ICED(Option<f32>)
    }

    #[derive(Debug)]
    struct Coffee {
        id: i64,
        temp: TempCategory,
        name: String
    }

    let coffee = Coffee {id: 1234, temp: TempCategory::HOT(Some(102.5)), name: String::from("Regular")};

    dbg!(coffee);
    // println!("Coffee id: {}", coffee.id);


    let parsed_num : i32 = "133".parse().unwrap();
    println!("Parsed num: {}", parsed_num);

    let my_float : f32 = parsed_num as f32;
    let my_int = my_float as i32;
    println!("my_float : {}", my_float);
    println!("my_int : {}", my_int);

    let (character,num1,num2) = my_tuple;
    println!("my tuple: {}/{}/{}",character,num1,num2);


    {
        let a = 1;
        let b = a;
        println!("a: {}, b: {}", a, b);
    }

    {
        let str_a = String::from("Hello");
        let str_b = str_a.clone();
        println!("str_a : {}", str_b);
    }

    let mut aa = 1;

    fn increase(input: &mut i32){
        *input += 10;
        println!("in function, input after increase: {}", input);
    }

    increase(&mut aa);
    println!("aa after increase returns: {}", aa);


    // pattern matching
    enum MessageState {
        Pending,
        Sending,
        Received
    }

    let msg_state = MessageState::Received;
    let status_code = match msg_state {
        MessageState::Pending => 1,
        MessageState::Sending => 2,
        MessageState::Received => 3
    };
    println!("Message state: {:?}", status_code);

    if let MessageState::Pending = msg_state {
        println!("Message state is pending");
    }

    enum MessageState2 {
        Pending(i64),
        Sending(i64),
        Received(i64)
    }
    let msg_state2 = MessageState2::Sending(3);
    if let MessageState2::Pending(status_code) = msg_state2 {
        println!("Message status {}", status_code)
    }


    // functions
    fn add(a: f32, b: f32) -> f32 {
        a + b
    }
    println!("a + b = {}", add(1.0, 2.0));

    let add_ten = | x: i32 | {
        x + 10
    };

    let even_numbers: Vec<i32> = (1..10).map(add_ten)
        .filter(|x| x %2 == 0)
        .collect();
    println!("even numbers: {:?}", even_numbers);

    let z = 0;

    let answer_p1 = add_number(1);
    println!("Part 1.  answer...  {answer_p1}");
    println!("Elapsed time part 1: {:.2?}", now.elapsed());
    println!();

    let my_bool = true;
    assert_eq!(my_bool, true);


}

