mod day1;
mod day2;

use std::fs::File;
use std::io::prelude::*;

fn main() {
    let input_filename = "input1.txt";

    let mut input_file = File::open(input_filename)
        .expect("Input file not found");

    let mut contents = String::new();
    input_file.read_to_string(&mut contents)
        .expect("Could not read input file");

    let input = day2::parse_input(input_filename)
        .expect("Failed to parse input");

    println!("Part 1 result: {}", day2::part1(&input));
    println!("Part 2 result: {}", day2::part2(&input));
}

