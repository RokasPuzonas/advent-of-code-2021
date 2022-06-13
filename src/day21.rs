use std::collections::HashMap;

pub fn parse_input(input: &str) -> (u8, u8) {
	let players: Vec<u8> = input
		.lines()
		.map(|l| l.split_once(": ").unwrap().1.parse().unwrap())
		.collect();
	return (players[0], players[1]);
}

pub fn part1(starting_positions: &(u8, u8)) -> u32 {
	let mut player1_position = starting_positions.0 as u32;
	let mut player2_position = starting_positions.1 as u32;
	let mut player1_score: u32 = 0;
	let mut player2_score: u32 = 0;
	let mut rolled_count = 0;

	let mut is_player1_turn = true;

	while player1_score < 1000 && player2_score < 1000 {
		let position;
		let score;
		if is_player1_turn {
			position = &mut player1_position;
			score = &mut player1_score;
		} else {
			position = &mut player2_position;
			score = &mut player2_score;
		}

		*position += (rolled_count + 1 - 1) % 100 + 1;
		*position += (rolled_count + 2 - 1) % 100 + 1;
		*position += (rolled_count + 3 - 1) % 100 + 1;

		*position = (*position - 1) % 10 + 1;
		*score += *position as u32;

		rolled_count += 3;
		is_player1_turn = !is_player1_turn;
	}

	player1_score.min(player2_score) * rolled_count as u32
}

fn get_wins_amount(
	starting_pos1: u32,
	starting_pos2: u32,
	starting_score1: u32,
	starting_score2: u32,
	memo: &mut HashMap<(u32, u32, u32, u32), (u64, u64)>,
) -> (u64, u64) {
	let memo_key = (
		starting_pos1,
		starting_pos2,
		starting_score1,
		starting_score2,
	);
	if memo.contains_key(&memo_key) {
		return *memo.get(&memo_key).unwrap();
	}
	let mut total_wins1 = 0;
	let mut total_wins2 = 0;

	for dice1 in 1..=3 {
		for dice2 in 1..=3 {
			for dice3 in 1..=3 {
				let pos1 = (starting_pos1 + dice1 + dice2 + dice3 - 1) % 10 + 1;
				let score1 = starting_score1 + pos1;
				if score1 >= 21 {
					total_wins1 += 1
				} else {
					let (wins2, wins1) = get_wins_amount(starting_pos2, pos1, starting_score2, score1, memo);
					total_wins1 += wins1;
					total_wins2 += wins2;
				}
			}
		}
	}

	memo.insert(memo_key, (total_wins1, total_wins2));

	(total_wins1, total_wins2)
}

pub fn part2(positions: &(u8, u8)) -> u64 {
	let mut memo = HashMap::new();
	let (wins1, wins2) = get_wins_amount(positions.0 as u32, positions.1 as u32, 0, 0, &mut memo);
	wins1.max(wins2)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn part1_example() {
		let input = (4, 8);
		let result = part1(&input);
		assert_eq!(result, 739785);
	}

	#[test]
	fn part2_example() {
		let input = (4, 8);
		let result = part2(&input);
		assert_eq!(result, 444356092776315);
	}
}
