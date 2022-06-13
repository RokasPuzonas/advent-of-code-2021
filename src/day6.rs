use std::num::ParseIntError;

pub fn parse_input(input: &str) -> Result<Vec<i32>, ParseIntError> {
	input
		.trim_end()
		.split_terminator(',')
		.map(|s| s.parse())
		.collect()
}

fn simulate_step(fishes: &mut Vec<i32>) {
	for i in 0..fishes.len() {
		if fishes[i] == 0 {
			fishes[i] = 7;
			fishes.push(9);
		}
	}

	for i in 0..fishes.len() {
		if fishes[i] > 0 {
			fishes[i] -= 1;
		}
	}
}

pub fn part1(input: &[i32]) -> u32 {
	let mut fishes = input.to_vec();

	for _ in 0..80 {
		simulate_step(&mut fishes)
	}

	fishes.len() as u32
}

// Instead of storing each fishes cycle as individual values group them up
// by there cycles. Because it dosen't matter where the fish is in the list.
// So just make an array of size 9 for the 9 possible fish cycle timers.
// And one extra group, for accounting for the delay that when the timer is 0,
// they produce a new fish only on the next turn.
pub fn part2(input: &[i32]) -> u64 {
	let mut groups: [u64; 10] = [0; 10];

	for fish in input.iter() {
		groups[*fish as usize + 1] += 1;
	}

	for _ in 0..256 {
		for i in 1..10 {
			groups[i - 1] += groups[i];
			groups[i] = 0;
		}
		groups[7] += groups[0];
		groups[9] += groups[0];
		groups[0] = 0;
	}

	let mut count: u64 = 0;
	for amount in groups.iter() {
		count += amount;
	}
	count
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn part1_example() {
		let input = [3, 4, 3, 1, 2];
		let result = part1(&input);
		assert_eq!(result, 5934);
	}

	#[test]
	fn part2_example() {
		let input = [3, 4, 3, 1, 2];
		let result = part2(&input);
		assert_eq!(result, 26984457539 as u64);
	}
}
