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
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

use std::fs::File;
use std::io::prelude::*;
use std::{env, process};

fn run(day: i32, part: i32, input_filename: &str) {
	let mut input_file = File::open(input_filename)
		.expect(format!("Input file '{}' not found", input_filename).as_str());

	let mut contents = String::new();
	input_file
		.read_to_string(&mut contents)
		.expect("Failed to read input file");

	// Janky solution so that I could specify what I want to run
	let parse_error_msg = "Failed to parse input";
	match format!("{}.{}", day, part).as_str() {
		"1.1" => println!(
			"{}",
			day1::part1(&day1::parse_input(&contents).expect(parse_error_msg))
		),
		"1.2" => println!(
			"{}",
			day1::part2(&day1::parse_input(&contents).expect(parse_error_msg))
		),
		"2.1" => println!(
			"{}",
			day2::part1(&day2::parse_input(&contents).expect(parse_error_msg))
		),
		"2.2" => println!(
			"{}",
			day2::part2(&day2::parse_input(&contents).expect(parse_error_msg))
		),
		"3.1" => println!(
			"{}",
			day3::part1(&day3::parse_input(&contents).expect(parse_error_msg))
		),
		"3.2" => println!(
			"{}",
			day3::part2(&day3::parse_input(&contents).expect(parse_error_msg))
		),
		"4.1" => println!(
			"{}",
			day4::part1(&day4::parse_input(&contents).expect(parse_error_msg))
		),
		"4.2" => println!(
			"{}",
			day4::part2(&day4::parse_input(&contents).expect(parse_error_msg))
		),
		"5.1" => println!(
			"{}",
			day5::part1(&day5::parse_input(&contents).expect(parse_error_msg))
		),
		"5.2" => println!(
			"{}",
			day5::part2(&day5::parse_input(&contents).expect(parse_error_msg))
		),
		"6.1" => println!(
			"{}",
			day6::part1(&day6::parse_input(&contents).expect(parse_error_msg))
		),
		"6.2" => println!(
			"{}",
			day6::part2(&day6::parse_input(&contents).expect(parse_error_msg))
		),
		"7.1" => println!(
			"{}",
			day7::part1(&day7::parse_input(&contents).expect(parse_error_msg))
		),
		"7.2" => println!(
			"{}",
			day7::part2(&day7::parse_input(&contents).expect(parse_error_msg))
		),
		"8.1" => println!("{}", day8::part1(&day8::parse_input(&contents))),
		"8.2" => println!("{}", day8::part2(&day8::parse_input(&contents))),
		"9.1" => println!("{}", day9::part1(day9::parse_input(&contents))),
		"9.2" => println!("{}", day9::part2(day9::parse_input(&contents))),
		"10.1" => println!("{}", day10::part1(&day10::parse_input(&contents))),
		"10.2" => println!("{}", day10::part2(&day10::parse_input(&contents))),
		"11.1" => println!("{}", day11::part1(&day11::parse_input(&contents))),
		"11.2" => println!("{}", day11::part2(&day11::parse_input(&contents))),
		"12.1" => println!("{}", day12::part1(&day12::parse_input(&contents))),
		"12.2" => println!("{}", day12::part2(&day12::parse_input(&contents))),
		"13.1" => println!("{}", day13::part1(&day13::parse_input(&contents))),
		"13.2" => day13::part2(&day13::parse_input(&contents)),
		"14.1" => println!("{}", day14::part1(&day14::parse_input(&contents))),
		"14.2" => println!("{}", day14::part2(&day14::parse_input(&contents))),
		"15.1" => println!("{}", day15::part1(&day15::parse_input(&contents))),
		"15.2" => println!("{}", day15::part2(&mut day15::parse_input(&contents))),
		"16.1" => println!("{}", day16::part1(&day16::parse_input(&contents))),
		"16.2" => println!("{}", day16::part2(&day16::parse_input(&contents))),
		"17.1" => println!("{}", day17::part1(&day17::parse_input(&contents))),
		"17.2" => println!("{}", day17::part2(&day17::parse_input(&contents))),
		"18.1" => println!("{}", day18::part1(&day18::parse_input(&contents))),
		"18.2" => println!("{}", day18::part2(&day18::parse_input(&contents))),
		"19.1" => println!("{}", day19::part1(&day19::parse_input(&contents))),
		"19.2" => println!("{}", day19::part2(&day19::parse_input(&contents))),
		"20.1" => println!("{}", day20::part1(&day20::parse_input(&contents))),
		"20.2" => println!("{}", day20::part2(&day20::parse_input(&contents))),
		"21.1" => println!("{}", day21::part1(&day21::parse_input(&contents))),
		"21.2" => println!("{}", day21::part2(&day21::parse_input(&contents))),
		"22.1" => println!("{}", day22::part1(&day22::parse_input(&contents))),
		"22.2" => println!("{}", day22::part2(day22::parse_input(&contents))),
		"23.1" => println!("{}", day23::part1(day23::parse_input(&contents))),
		"23.2" => println!("{}", day23::part2(day23::parse_input(&contents))),
		"24.1" => println!("{}", day24::part1(day24::parse_input(&contents))),
		"24.2" => println!("{}", day24::part2(day24::parse_input(&contents))),
		"25.1" => println!("{}", day25::part1(day25::parse_input(&contents))),
		"25.2" => println!("{}", day25::part2(day25::parse_input(&contents))),
		_ => println!("Day {} part {} not found", day, part),
	}
}

fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() < 3 {
		println!("Usage: {} <day> <part> [input-file]", args[0]);
		process::exit(0);
	}

	let day = args[1].parse::<i32>().expect("Failed to parse day");

	let part = args[2].parse::<i32>().expect("Failed to parse part");

	let input_filename;
	if args.len() > 3 {
		input_filename = args[3].clone();
	} else {
		input_filename = format!("input/{}.txt", day);
	}
	run(day, part, &input_filename);
}
