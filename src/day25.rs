#[derive(Debug, PartialEq, Eq)]
pub enum Tile {
	East,
	South,
	Empty
}

type SeaFloor = Vec<Vec<Tile>>;

pub fn parse_input(input: &str) -> SeaFloor {
	let mut floor = vec![];

	for line in input.lines() {
		let mut row = vec![];
		for char in line.chars() {
			match char {
				'>' => row.push(Tile::East),
				'v' => row.push(Tile::South),
				'.' => row.push(Tile::Empty),
				_ => panic!()
			};
		}
		floor.push(row);
	}
	floor
}

fn step(floor: &mut SeaFloor) -> bool {
	let mut moved = false;
	let width = floor.first().unwrap().len();
	let height = floor.len();

	{
		for row in floor.iter_mut() {
			let mut moved_tiles = vec![];
			for (x, tile) in row.iter().enumerate() {
				if *tile == Tile::East && row[(x+1) % width] == Tile::Empty {
					moved_tiles.push(x);
				}
			}

			if !moved_tiles.is_empty() {
				moved = true;
			}

			for x in moved_tiles {
				row[x] = Tile::Empty;
				row[(x+1) % width] = Tile::East;
			}
		}
	}

	{
		for x in 0..width {
			let mut moved_tiles = vec![];
			for y in 0..height {
				if floor[y][x] == Tile::South && floor[(y+1) % height][x] == Tile::Empty {
					moved_tiles.push(y);
				}
			}

			if !moved_tiles.is_empty() {
				moved = true;
			}

			for y in moved_tiles {
				floor[y][x] = Tile::Empty;
				floor[(y+1) % height][x] = Tile::South;
			}
		}
	}

	moved
}

fn show_seafloor(floor: &SeaFloor) {
	for row in floor {
		for tile in row {
			match tile {
				Tile::East => print!(">"),
				Tile::South => print!("v"),
				Tile::Empty => print!("."),
			}
		}
		println!();
	}
	println!();
}

pub fn part1(mut floor: SeaFloor) -> u32 {
	let mut count = 1;
	while step(&mut floor) {
		count += 1;
	}
	count
}

pub fn part2(floor: SeaFloor) -> u32 {
	todo!();
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn part1_example() {
 		let map = parse_input(&[
			"v...>>.vv>",
			".vv>>.vv..",
			">>.>v>...v",
			">>v>>.>.v.",
			"v>v.vv.v..",
			">.>>..v...",
			".vv..>.>v.",
			"v.v..>>v.v",
			"....v..v.>",
		].join("\n"));
		let result = part1(map);
		assert_eq!(result, 58);
	}
}
