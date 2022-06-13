use std::num::ParseIntError;

#[derive(Debug)]
pub struct BingoGame {
	numbers: Vec<i32>,
	boards: Vec<[i32; 25]>,
}

#[derive(Debug)]
pub enum ParseBingoGameError {
	NoNumbersError,
	ParseNumberError(ParseIntError),
}

pub fn parse_input(input: &str) -> Result<BingoGame, ParseBingoGameError> {
	let mut sections = input.split_terminator("\n\n");

	let numbers_section = sections.next().ok_or(ParseBingoGameError::NoNumbersError)?;

	let numbers = numbers_section
		.split_terminator(",")
		.map(|s| s.parse::<i32>())
		.collect::<Result<Vec<i32>, ParseIntError>>()
		.map_err(ParseBingoGameError::ParseNumberError)?;

	let mut boards = Vec::new();
	for section in sections {
		let mut board: [i32; 25] = [0; 25];
		let mut y = 0;
		for row in section.split_terminator("\n") {
			let mut x = 0;
			for s in row.split_whitespace() {
				let n = s
					.trim()
					.parse::<i32>()
					.map_err(ParseBingoGameError::ParseNumberError)?;
				board[5 * y + x] = n;
				x += 1;
			}
			y += 1;
		}
		boards.push(board);
	}

	Ok(BingoGame { numbers, boards })
}

fn find_number(board: &[i32], value: i32) -> Option<usize> {
	for i in 0..board.len() {
		if board[i] == value {
			return Some(i);
		}
	}
	return None;
}

fn mark_number(board: &[i32], markings: &mut i32, value: i32) -> bool {
	let pos: usize;
	match find_number(board, value) {
		None => return false,
		Some(n) => pos = n,
	};
	*markings |= 1 << pos;
	return true;
}

fn contains_win(markings: i32) -> bool {
	const WINNING_COMBINATIONS: [i32; 10] = [
		0b11111 << 0,                // First row
		0b11111 << 5,                // Second row
		0b11111 << 10,               // Third row
		0b11111 << 15,               // Fourth row
		0b11111 << 20,               // Fifth row
		0b0000100001000010000100001, // First column
		0b0001000010000100001000010, // Second column
		0b0010000100001000010000100, // Third column
		0b0100001000010000100001000, // Fourth column
		0b1000010000100001000010000, // Fifth column
	];

	for comb in WINNING_COMBINATIONS.iter() {
		if markings & comb == *comb {
			return true;
		}
	}
	false
}

fn sum_unmarked_numbers(board: &[i32], markings: i32) -> i32 {
	let mut sum = 0;
	for i in 0..25 {
		if markings & (1 << i) == 0 {
			sum += board[i];
		}
	}
	return sum;
}

pub fn part1(game: &BingoGame) -> i32 {
	let mut markings = vec![0; game.boards.len()];

	for num in game.numbers.iter() {
		for i in 0..game.boards.len() {
			if mark_number(&game.boards[i], &mut markings[i], *num) && contains_win(markings[i]) {
				return num * sum_unmarked_numbers(&game.boards[i], markings[i]);
			}
		}
	}
	-1
}

pub fn part2(game: &BingoGame) -> i32 {
	let mut markings = vec![0; game.boards.len()];
	let mut winning_numbers: Vec<i32> = vec![0; game.boards.len()];
	let mut last_winning_board = usize::MAX;

	for num in game.numbers.iter() {
		for i in 0..game.boards.len() {
			if winning_numbers[i] == 0
				&& mark_number(&game.boards[i], &mut markings[i], *num)
				&& contains_win(markings[i])
			{
				winning_numbers[i] = *num;
				last_winning_board = i;
			}
		}
	}

	return winning_numbers[last_winning_board]
		* sum_unmarked_numbers(
			&game.boards[last_winning_board],
			markings[last_winning_board],
		);
}

#[cfg(test)]
mod tests {
	use super::*;

	fn new_test_board() -> [i32; 25] {
		let mut board: [i32; 25] = [0; 25];
		for i in 0..25 {
			board[i] = i as i32;
		}
		return board;
	}

	#[test]
	fn part1_example() {
		let input = BingoGame {
			numbers: vec![
				7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19, 3,
				26, 1,
			],
			boards: vec![
				[
					22, 13, 17, 11, 0, 8, 2, 23, 4, 24, 21, 9, 14, 16, 7, 6, 10, 3, 18, 5, 1, 12, 20, 15, 19,
				],
				[
					3, 15, 0, 2, 22, 9, 18, 13, 17, 5, 19, 8, 7, 25, 23, 20, 11, 10, 24, 4, 14, 21, 16, 12, 6,
				],
				[
					14, 21, 17, 24, 4, 10, 16, 15, 9, 19, 18, 8, 23, 26, 20, 22, 11, 13, 6, 5, 2, 0, 12, 3, 7,
				],
			],
		};
		let result = part1(&input);
		assert_eq!(result, 4512);
	}

	#[test]
	fn part2_example() {
		let input = BingoGame {
			numbers: vec![
				7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19, 3,
				26, 1,
			],
			boards: vec![
				[
					22, 13, 17, 11, 0, 8, 2, 23, 4, 24, 21, 9, 14, 16, 7, 6, 10, 3, 18, 5, 1, 12, 20, 15, 19,
				],
				[
					3, 15, 0, 2, 22, 9, 18, 13, 17, 5, 19, 8, 7, 25, 23, 20, 11, 10, 24, 4, 14, 21, 16, 12, 6,
				],
				[
					14, 21, 17, 24, 4, 10, 16, 15, 9, 19, 18, 8, 23, 26, 20, 22, 11, 13, 6, 5, 2, 0, 12, 3, 7,
				],
			],
		};
		let result = part2(&input);
		assert_eq!(result, 1924);
	}

	#[test]
	fn check_mark_number() {
		let board = new_test_board();
		let mut markings = 0b0;
		mark_number(&board, &mut markings, 0);
		assert_eq!(markings, 1);

		mark_number(&board, &mut markings, 1);
		assert_eq!(markings, 3);
	}

	#[test]
	fn check_contains_win() {
		let board = new_test_board();
		let mut markings = 0b0;
		mark_number(&board, &mut markings, 0);
		mark_number(&board, &mut markings, 1);
		mark_number(&board, &mut markings, 2);
		mark_number(&board, &mut markings, 3);
		mark_number(&board, &mut markings, 4);
		mark_number(&board, &mut markings, 5);
		assert!(contains_win(markings));
	}
}
