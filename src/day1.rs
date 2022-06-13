use std::num::ParseIntError;

pub fn parse_input(input: &str) -> Result<Vec<u32>, ParseIntError> {
	input.split_whitespace().map(|s| s.parse()).collect()
}

pub fn part1(depths: &[u32]) -> u32 {
	let mut count = 0;
	for i in 1..depths.len() {
		if depths[i] > depths[i - 1] {
			count += 1;
		}
	}
	return count;
}

pub fn part2(depths: &[u32]) -> u32 {
	let mut count = 0;
	for i in 2..depths.len() - 1 {
		if depths[i + 1] > depths[i - 2] {
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
