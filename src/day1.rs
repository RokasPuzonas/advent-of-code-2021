use std::fs::File;
use std::io::prelude::*;

pub fn input_from_file(filename: &str) -> Vec<u32> {
    let mut file = File::open(filename).expect("Can't open file");

    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Oops! Can not read the file...");

    return contents.split_whitespace().map(|s| s.parse::<u32>().unwrap()).collect();
}

pub fn part1(depths: &[u32]) -> u32 {
    let mut count = 0;
    for i in 1..depths.len()  {
        if depths[i] > depths[i-1] {
            count += 1;
        }
    }
    return count;
}

pub fn part2(depths: &[u32]) -> u32 {
    let mut count = 0;
    for i in 2..depths.len()-1  {
        if depths[i+1] > depths[i-2] {
            count += 1;
        }
    }
    return count;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let result = part1(&input);
        assert_eq!(result, 7);
    }

    #[test]
    fn part2_example() {
        let input = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let result = part2(&input);
        assert_eq!(result, 5);
    }
}

