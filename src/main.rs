mod day1;

fn main() {
    let input_filename = "input.txt";
    let input = day1::input_from_file(input_filename);
    let result1 = day1::part1(&input);
    let result2 = day1::part2(&input);
    println!("Part 1 result: {}", result1);
    println!("Part 2 result: {}", result2);
}

