mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;

use std::{env, process};
use std::fs::File;
use std::io::prelude::*;

fn run(day: i32, part: i32, input_filename: &str) {
    let mut input_file = File::open(input_filename)
        .expect(format!("Input file '{}' not found", input_filename).as_str());

    let mut contents = String::new();
    input_file.read_to_string(&mut contents)
        .expect("Failed to read input file");

    // Janky solution so that I could specify what I want to run
    let parse_error_msg = "Failed to parse input";
    match format!("{}.{}", day, part).as_str() {
        "1.1" => println!("{}", day1::part1(&day1::parse_input(&contents).expect(parse_error_msg))),
        "1.2" => println!("{}", day1::part2(&day1::parse_input(&contents).expect(parse_error_msg))),
        "2.1" => println!("{}", day2::part1(&day2::parse_input(&contents).expect(parse_error_msg))),
        "2.2" => println!("{}", day2::part2(&day2::parse_input(&contents).expect(parse_error_msg))),
        "3.1" => println!("{}", day3::part1(&day3::parse_input(&contents).expect(parse_error_msg))),
        "3.2" => println!("{}", day3::part2(&day3::parse_input(&contents).expect(parse_error_msg))),
        "4.1" => println!("{}", day4::part1(&day4::parse_input(&contents).expect(parse_error_msg))),
        "4.2" => println!("{}", day4::part2(&day4::parse_input(&contents).expect(parse_error_msg))),
        "5.1" => println!("{}", day5::part1(&day5::parse_input(&contents).expect(parse_error_msg))),
        "5.2" => println!("{}", day5::part2(&day5::parse_input(&contents).expect(parse_error_msg))),
        "6.1" => println!("{}", day6::part1(&day6::parse_input(&contents).expect(parse_error_msg))),
        "6.2" => println!("{}", day6::part2(&day6::parse_input(&contents).expect(parse_error_msg))),
        "7.1" => println!("{}", day7::part1(&day7::parse_input(&contents).expect(parse_error_msg))),
        "7.2" => println!("{}", day7::part2(&day7::parse_input(&contents).expect(parse_error_msg))),
        "8.1" => println!("{}", day8::part1(&day8::parse_input(&contents))),
        "8.2" => println!("{}", day8::part2(&day8::parse_input(&contents))),
        "9.1" => println!("{}", day9::part1(day9::parse_input(&contents))),
        "9.2" => println!("{}", day9::part2(day9::parse_input(&contents))),
        "10.1" => println!("{}", day10::part1(&day10::parse_input(&contents))),
        "10.2" => println!("{}", day10::part2(&day10::parse_input(&contents))),
        "11.1" => println!("{}", day11::part1(&day11::parse_input(&contents))),
        "11.2" => println!("{}", day11::part2(&day11::parse_input(&contents))),
        _ => println!("Day {} part {} not found", day, part)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: {} <day> <part> [input-file]", args[0]);
        process::exit(0);
    }

    let day = args[1].parse::<i32>()
        .expect("Failed to parse day");

    let part = args[2].parse::<i32>()
        .expect("Failed to parse part");

    let input_filename;
    if args.len() > 3 {
        input_filename = args[3].clone();
    } else {
        input_filename = format!("input/{}.txt", day);
    }
    run(day, part, &input_filename);
}

