mod day4;

use std::fs::File;
use std::io::prelude::*;

fn main() {
    let input_filename = "input4.txt";

    let mut input_file = File::open(input_filename)
        .expect("Input file not found");

    let mut contents = String::new();
    input_file.read_to_string(&mut contents)
        .expect("Could not read input file");

    let input = day4::parse_input(&contents)
        .expect("Failed to parse input");

    println!("Part 1 result: {}", day4::part1(&input));
    println!("Part 2 result: {}", day4::part2(&input));
}

