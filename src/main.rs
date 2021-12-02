mod day1;
mod day2;

fn main() {
    let input_filename = "input2.txt";
    let input = day2::input_from_file(input_filename)
        .expect("Failed to read input2.txt");

    // let result1 = day1::part1(&input);
    // let result2 = day1::part2(&input);
    
    let result1 = day2::part1(&input);
    // let result2 = day2::part2(&input);

    println!("Part 1 result: {}", result1);
    // println!("Part 2 result: {}", result2);
}

