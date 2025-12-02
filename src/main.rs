use chrono::{DateTime, Utc};
use chrono_tz::US::Eastern;

/// Advent of Code 2025
/// December 2025
/// https://adventofcode.com/2025

use clap::Parser;
mod days;

#[derive(Parser)]
struct Args {
    #[arg(help = "Day number (1-12)")]
    day: u8,
}

fn main() {
    println!("Advent of Code 2025");
    // Start
    let start_datetime = Utc::now();
    println!("Start date and time  (Eastern US): {}", start_datetime.with_timezone(&Eastern));

    // Read day number from command line.
    let args = Args::parse();

    match args.day {
        1 => days::day01::run(),
        2 => days::day02::run(),
        3 => days::day03::run(),
        4 => days::day04::run(),
        5 => days::day05::run(),
        6 => days::day06::run(),
        7 => days::day07::run(),
        8 => days::day08::run(),
        9 => days::day09::run(),
        10 => days::day10::run(),
        11 => days::day11::run(),
        12 => days::day12::run(),
        _ => eprintln!("Day must be between 1 and 12."),
    }

    // End
    let utc_time: DateTime<Utc> = Utc::now();
    println!("\nEnd date and time (Eastern US): {}", utc_time.with_timezone(&Eastern));
}
