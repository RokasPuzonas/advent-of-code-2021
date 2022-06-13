use std::{collections::HashMap, convert::TryInto};

pub struct Entry([String; 10], [String; 4]);

fn parse_line(line: &str) -> Entry {
	let parts: Vec<&str> = line.split(" | ").collect();
	let unique_patterns = parts[0]
		.split_whitespace()
		.map(|s| String::from(s))
		.collect::<Vec<String>>()
		.try_into()
		.unwrap();
	let output_digits = parts[1]
		.split_whitespace()
		.map(|s| String::from(s))
		.collect::<Vec<String>>()
		.try_into()
		.unwrap();
	return Entry(unique_patterns, output_digits);
}

pub fn parse_input(input: &str) -> Vec<Entry> {
	input.lines().map(parse_line).collect()
}

pub fn part1(entries: &[Entry]) -> u32 {
	let mut count = 0;
	for entry in entries {
		for digit in entry.1.iter() {
			let len = digit.len();
			if len == 2 || len == 3 || len == 4 || len == 7 {
				count += 1;
			}
		}
	}
	count
}

// Helper function which converts a string into a binary representation
// I did this is so that the order of the letters won't matter
fn signal_to_bitmask(signal: &str) -> u32 {
	let mut bitmask = 0;
	for c in signal.chars() {
		match c {
			'a' => bitmask += 0b0000001,
			'b' => bitmask += 0b0000010,
			'c' => bitmask += 0b0000100,
			'd' => bitmask += 0b0001000,
			'e' => bitmask += 0b0010000,
			'f' => bitmask += 0b0100000,
			'g' => bitmask += 0b1000000,
			_ => bitmask += 0,
		}
	}
	return bitmask;
}

fn decode_signal(signal: &str, wire_loopup: &HashMap<u32, u32>) -> Option<u32> {
	wire_loopup.get(&signal_to_bitmask(signal)).map(|n| *n)
}

fn decode_signals(signals: &[String], wire_loopup: &HashMap<u32, u32>) -> u32 {
	let mut number = 0;
	let n = signals.len();
	for i in 0..n {
		let value = decode_signal(&signals[i], wire_loopup).unwrap();
		number += value * u32::pow(10, (n - i - 1) as u32);
	}
	return number;
}

fn decode_wire_lookup(signals: &[String; 10]) -> HashMap<u32, u32> {
	let mut wire_lookup = HashMap::new();
	let mut bitmasks: [u32; 10] = [0; 10];
	for i in 0..10 {
		bitmasks[i] = signal_to_bitmask(&signals[i]);
	}

	let mut one_bitmask = 0;
	let mut four_bitmask = 0;

	// Decode all signals which have unique number of wires
	for i in 0..10 {
		let len = signals[i].len();
		if len == 2 {
			one_bitmask = bitmasks[i];
			wire_lookup.insert(one_bitmask, 1);
		} else if len == 3 {
			wire_lookup.insert(bitmasks[i], 7);
		} else if len == 4 {
			four_bitmask = bitmasks[i];
			wire_lookup.insert(four_bitmask, 4);
		} else if len == 7 {
			wire_lookup.insert(bitmasks[i], 8);
		}
	}

	let fourdiff = four_bitmask ^ one_bitmask;

	for i in 0..10 {
		let len = signals[i].len();
		if len == 5 {
			if bitmasks[i] & one_bitmask == one_bitmask {
				wire_lookup.insert(bitmasks[i], 3);
			} else if bitmasks[i] & fourdiff == fourdiff {
				wire_lookup.insert(bitmasks[i], 5);
			} else {
				wire_lookup.insert(bitmasks[i], 2);
			}
		} else if len == 6 {
			if bitmasks[i] & four_bitmask == four_bitmask {
				wire_lookup.insert(bitmasks[i], 9);
			} else if bitmasks[i] & fourdiff == fourdiff {
				wire_lookup.insert(bitmasks[i], 6);
			} else {
				wire_lookup.insert(bitmasks[i], 0);
			}
		}
	}

	return wire_lookup;
}

fn decode_entry(entry: &Entry) -> u32 {
	let wire_lookup = decode_wire_lookup(&entry.0);
	return decode_signals(&entry.1, &wire_lookup);
}

pub fn part2(entries: &[Entry]) -> u32 {
	let mut sum = 0;
	for entry in entries {
		sum += decode_entry(entry);
	}
	sum
}

#[cfg(test)]
mod tests {
	use super::*;

	// I know it's ugly
	#[test]
	fn part1_example() {
		let input = vec![
			Entry(
				[
					"be".into(),
					"cfbegad".into(),
					"cbdgef".into(),
					"fgaecd".into(),
					"cgeb".into(),
					"fdcge".into(),
					"agebfd".into(),
					"fecdb".into(),
					"fabcd".into(),
					"edb".into(),
				],
				[
					"fdgacbe".into(),
					"cefdb".into(),
					"cefbgd".into(),
					"gcbe".into(),
				],
			),
			Entry(
				[
					"edbfga".into(),
					"begcd".into(),
					"cbg".into(),
					"gc".into(),
					"gcadebf".into(),
					"fbgde".into(),
					"acbgfd".into(),
					"abcde".into(),
					"gfcbed".into(),
					"gfec".into(),
				],
				["fcgedb".into(), "cgb".into(), "dgebacf".into(), "gc".into()],
			),
			Entry(
				[
					"fgaebd".into(),
					"cg".into(),
					"bdaec".into(),
					"gdafb".into(),
					"agbcfd".into(),
					"gdcbef".into(),
					"bgcad".into(),
					"gfac".into(),
					"gcb".into(),
					"cdgabef".into(),
				],
				["cg".into(), "cg".into(), "fdcagb".into(), "cbg".into()],
			),
			Entry(
				[
					"fbegcd".into(),
					"cbd".into(),
					"adcefb".into(),
					"dageb".into(),
					"afcb".into(),
					"bc".into(),
					"aefdc".into(),
					"ecdab".into(),
					"fgdeca".into(),
					"fcdbega".into(),
				],
				[
					"efabcd".into(),
					"cedba".into(),
					"gadfec".into(),
					"cb".into(),
				],
			),
			Entry(
				[
					"aecbfdg".into(),
					"fbg".into(),
					"gf".into(),
					"bafeg".into(),
					"dbefa".into(),
					"fcge".into(),
					"gcbea".into(),
					"fcaegb".into(),
					"dgceab".into(),
					"fcbdga".into(),
				],
				[
					"gecf".into(),
					"egdcabf".into(),
					"bgf".into(),
					"bfgea".into(),
				],
			),
			Entry(
				[
					"fgeab".into(),
					"ca".into(),
					"afcebg".into(),
					"bdacfeg".into(),
					"cfaedg".into(),
					"gcfdb".into(),
					"baec".into(),
					"bfadeg".into(),
					"bafgc".into(),
					"acf".into(),
				],
				[
					"gebdcfa".into(),
					"ecba".into(),
					"ca".into(),
					"fadegcb".into(),
				],
			),
			Entry(
				[
					"dbcfg".into(),
					"fgd".into(),
					"bdegcaf".into(),
					"fgec".into(),
					"aegbdf".into(),
					"ecdfab".into(),
					"fbedc".into(),
					"dacgb".into(),
					"gdcebf".into(),
					"gf".into(),
				],
				[
					"cefg".into(),
					"dcbef".into(),
					"fcge".into(),
					"gbcadfe".into(),
				],
			),
			Entry(
				[
					"bdfegc".into(),
					"cbegaf".into(),
					"gecbf".into(),
					"dfcage".into(),
					"bdacg".into(),
					"ed".into(),
					"bedf".into(),
					"ced".into(),
					"adcbefg".into(),
					"gebcd".into(),
				],
				["ed".into(), "bcgafe".into(), "cdgba".into(), "cbgef".into()],
			),
			Entry(
				[
					"egadfb".into(),
					"cdbfeg".into(),
					"cegd".into(),
					"fecab".into(),
					"cgb".into(),
					"gbdefca".into(),
					"cg".into(),
					"fgcdab".into(),
					"egfdb".into(),
					"bfceg".into(),
				],
				["gbdfcae".into(), "bgc".into(), "cg".into(), "cgb".into()],
			),
			Entry(
				[
					"gcafb".into(),
					"gcf".into(),
					"dcaebfg".into(),
					"ecagb".into(),
					"gf".into(),
					"abcdeg".into(),
					"gaef".into(),
					"cafbge".into(),
					"fdbac".into(),
					"fegbdc".into(),
				],
				["fgae".into(), "cfgab".into(), "fg".into(), "bagce".into()],
			),
		];
		let result = part1(&input);
		assert_eq!(result, 26);
	}

	#[test]
	fn part2_example() {
		let input = vec![
			Entry(
				[
					"be".into(),
					"cfbegad".into(),
					"cbdgef".into(),
					"fgaecd".into(),
					"cgeb".into(),
					"fdcge".into(),
					"agebfd".into(),
					"fecdb".into(),
					"fabcd".into(),
					"edb".into(),
				],
				[
					"fdgacbe".into(),
					"cefdb".into(),
					"cefbgd".into(),
					"gcbe".into(),
				],
			),
			Entry(
				[
					"edbfga".into(),
					"begcd".into(),
					"cbg".into(),
					"gc".into(),
					"gcadebf".into(),
					"fbgde".into(),
					"acbgfd".into(),
					"abcde".into(),
					"gfcbed".into(),
					"gfec".into(),
				],
				["fcgedb".into(), "cgb".into(), "dgebacf".into(), "gc".into()],
			),
			Entry(
				[
					"fgaebd".into(),
					"cg".into(),
					"bdaec".into(),
					"gdafb".into(),
					"agbcfd".into(),
					"gdcbef".into(),
					"bgcad".into(),
					"gfac".into(),
					"gcb".into(),
					"cdgabef".into(),
				],
				["cg".into(), "cg".into(), "fdcagb".into(), "cbg".into()],
			),
			Entry(
				[
					"fbegcd".into(),
					"cbd".into(),
					"adcefb".into(),
					"dageb".into(),
					"afcb".into(),
					"bc".into(),
					"aefdc".into(),
					"ecdab".into(),
					"fgdeca".into(),
					"fcdbega".into(),
				],
				[
					"efabcd".into(),
					"cedba".into(),
					"gadfec".into(),
					"cb".into(),
				],
			),
			Entry(
				[
					"aecbfdg".into(),
					"fbg".into(),
					"gf".into(),
					"bafeg".into(),
					"dbefa".into(),
					"fcge".into(),
					"gcbea".into(),
					"fcaegb".into(),
					"dgceab".into(),
					"fcbdga".into(),
				],
				[
					"gecf".into(),
					"egdcabf".into(),
					"bgf".into(),
					"bfgea".into(),
				],
			),
			Entry(
				[
					"fgeab".into(),
					"ca".into(),
					"afcebg".into(),
					"bdacfeg".into(),
					"cfaedg".into(),
					"gcfdb".into(),
					"baec".into(),
					"bfadeg".into(),
					"bafgc".into(),
					"acf".into(),
				],
				[
					"gebdcfa".into(),
					"ecba".into(),
					"ca".into(),
					"fadegcb".into(),
				],
			),
			Entry(
				[
					"dbcfg".into(),
					"fgd".into(),
					"bdegcaf".into(),
					"fgec".into(),
					"aegbdf".into(),
					"ecdfab".into(),
					"fbedc".into(),
					"dacgb".into(),
					"gdcebf".into(),
					"gf".into(),
				],
				[
					"cefg".into(),
					"dcbef".into(),
					"fcge".into(),
					"gbcadfe".into(),
				],
			),
			Entry(
				[
					"bdfegc".into(),
					"cbegaf".into(),
					"gecbf".into(),
					"dfcage".into(),
					"bdacg".into(),
					"ed".into(),
					"bedf".into(),
					"ced".into(),
					"adcbefg".into(),
					"gebcd".into(),
				],
				["ed".into(), "bcgafe".into(), "cdgba".into(), "cbgef".into()],
			),
			Entry(
				[
					"egadfb".into(),
					"cdbfeg".into(),
					"cegd".into(),
					"fecab".into(),
					"cgb".into(),
					"gbdefca".into(),
					"cg".into(),
					"fgcdab".into(),
					"egfdb".into(),
					"bfceg".into(),
				],
				["gbdfcae".into(), "bgc".into(), "cg".into(), "cgb".into()],
			),
			Entry(
				[
					"gcafb".into(),
					"gcf".into(),
					"dcaebfg".into(),
					"ecagb".into(),
					"gf".into(),
					"abcdeg".into(),
					"gaef".into(),
					"cafbge".into(),
					"fdbac".into(),
					"fegbdc".into(),
				],
				["fgae".into(), "cfgab".into(), "fg".into(), "bagce".into()],
			),
		];
		let result = part2(&input);
		assert_eq!(result, 61229);
	}
}
