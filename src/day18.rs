// Solution gotten from: https://github.com/AxlLind/AdventOfCode2021/blob/main/src/bin/18.rs

fn parse_snailfish(line: &str) -> Vec<(i32, u8)> {
	let mut depth = 0;
	let mut nums = Vec::new();
	for c in line.chars() {
		match c {
			'[' => depth += 1,
			']' => depth -= 1,
			',' => {}
			_ => nums.push(((c as u8 - b'0') as i32, depth)),
		}
	}
	return nums;
}

fn find_deepest_index(num: &Vec<(i32, u8)>) -> usize {
	let mut deepest_index = 0;
	let mut deepest_depth = 0;
	for i in 0..num.len() {
		let depth = num[i].1;
		if deepest_depth < depth {
			deepest_depth = depth;
			deepest_index = i;
		}
	}
	return deepest_index;
}

pub fn parse_input(input: &str) -> Vec<Vec<(i32, u8)>> {
	let mut nums = Vec::new();
	for line in input.lines() {
		nums.push(parse_snailfish(line));
	}
	return nums;
}

fn add_snailfish(a: &Vec<(i32, u8)>, b: &Vec<(i32, u8)>) -> Vec<(i32, u8)> {
	let mut added = Vec::new();
	for (num, depth) in a {
		added.push((*num, depth + 1));
	}
	for (num, depth) in b {
		added.push((*num, depth + 1));
	}
	return added;
}

fn try_exploding(num: &mut Vec<(i32, u8)>) -> bool {
	let i = find_deepest_index(num);
	if num[i].1 < 5 {
		return false;
	}
	let (left_num, depth) = num[i];
	let right_num = num.remove(i + 1).0;
	if i > 0 {
		num[i - 1].0 += left_num;
	}
	if i + 1 < num.len() {
		num[i + 1].0 += right_num;
	}
	num[i] = (0, depth - 1);
	return true;
}

fn try_splitting(num: &mut Vec<(i32, u8)>) -> bool {
	let target = match num.iter().position(|&(n, _)| n > 9) {
		Some(i) => i,
		None => return false,
	};
	let (x, depth) = num[target];
	num[target] = (x / 2, depth + 1);
	num.insert(target + 1, ((x + 1) / 2, depth + 1));
	return true;
}

fn reduce_snailfish(num: &mut Vec<(i32, u8)>) {
	loop {
		if try_exploding(num) {
			continue;
		}
		if try_splitting(num) {
			continue;
		}
		break;
	}
}

fn get_magnitude(mut num: Vec<(i32, u8)>) -> i32 {
	while num.len() > 1 {
		let i = find_deepest_index(&num);
		let (left_num, depth) = num[i];
		let right_num = num[i + 1].0;
		num[i] = (3 * left_num + 2 * right_num, depth - 1);
		num.remove(i + 1);
	}
	num[0].0
}

fn add_and_reduce(a: &Vec<(i32, u8)>, b: &Vec<(i32, u8)>) -> Vec<(i32, u8)> {
	let mut result = add_snailfish(a, b);
	reduce_snailfish(&mut result);
	return result;
}

fn sum(nums: &Vec<Vec<(i32, u8)>>) -> Vec<(i32, u8)> {
	let mut result = nums[0].clone();
	for i in 1..nums.len() {
		result = add_snailfish(&result, &nums[i]);
		reduce_snailfish(&mut result);
	}
	return result;
}

pub fn part1(nums: &Vec<Vec<(i32, u8)>>) -> i32 {
	return get_magnitude(sum(nums));
}

pub fn part2(nums: &Vec<Vec<(i32, u8)>>) -> i32 {
	let mut max_magnitude = 0;
	let n = nums.len();
	for i in 0..n {
		for j in 0..n - 1 {
			let a = &nums[i];
			let b = &nums[j];
			max_magnitude = max_magnitude.max(get_magnitude(add_and_reduce(a, b)));
			max_magnitude = max_magnitude.max(get_magnitude(add_and_reduce(b, a)));
		}
	}
	return max_magnitude;
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn part1_example() {
		let nums = parse_input(
			"[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]",
		);
		let result = part1(&nums);
		assert_eq!(result, 4140);
	}

	#[test]
	fn part2_example() {
		let nums = parse_input(
			"[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]",
		);
		let result = part2(&nums);
		assert_eq!(result, 3993);
	}

	fn test_explosion(initial: &str, expected: &str) {
		let mut num = parse_snailfish(initial);
		try_exploding(&mut num);
		assert_eq!(num, parse_snailfish(expected));
	}

	#[test]
	fn exploding_1() {
		test_explosion("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]");
	}
	#[test]
	fn exploding_2() {
		test_explosion("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]");
	}
	#[test]
	fn exploding_3() {
		test_explosion("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]");
	}
	#[test]
	fn exploding_4() {
		test_explosion(
			"[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
			"[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
		);
	}
	#[test]
	fn exploding_5() {
		test_explosion(
			"[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
			"[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
		);
	}

	fn test_sum(nums: Vec<&str>, expected: &str) {
		let mut parsed_nums = Vec::new();
		for num in nums {
			parsed_nums.push(parse_snailfish(num));
		}
		assert_eq!(sum(&parsed_nums), parse_snailfish(expected));
	}

	#[test]
	fn sum_1() {
		let nums = vec!["[1,1]", "[2,2]", "[3,3]", "[4,4]"];
		test_sum(nums, "[[[[1,1],[2,2]],[3,3]],[4,4]]");
	}
	#[test]
	fn sum_2() {
		let nums = vec!["[1,1]", "[2,2]", "[3,3]", "[4,4]", "[5,5]"];
		test_sum(nums, "[[[[3,0],[5,3]],[4,4]],[5,5]]");
	}
	#[test]
	fn sum_3() {
		let nums = vec!["[1,1]", "[2,2]", "[3,3]", "[4,4]", "[5,5]", "[6,6]"];
		test_sum(nums, "[[[[5,0],[7,4]],[5,5]],[6,6]]");
	}
	#[test]
	fn sum_4() {
		let nums = vec![
			"[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]",
			"[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
			"[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]",
			"[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]",
			"[7,[5,[[3,8],[1,4]]]]",
			"[[2,[2,2]],[8,[8,1]]]",
			"[2,9]",
			"[1,[[[9,3],9],[[9,0],[0,7]]]]",
			"[[[5,[7,4]],7],1]",
			"[[[[4,2],2],6],[8,7]]",
		];
		test_sum(
			nums,
			"[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
		);
	}

	fn test_magnitude(initial: &str, expected: i32) {
		let num = parse_snailfish(initial);
		assert_eq!(get_magnitude(num), expected);
	}

	#[test]
	fn magnitude_1() {
		test_magnitude("[[1,2],[[3,4],5]]", 143);
	}
	#[test]
	fn magnitude_2() {
		test_magnitude("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", 1384);
	}
	#[test]
	fn magnitude_3() {
		test_magnitude("[[[[1,1],[2,2]],[3,3]],[4,4]]", 445);
	}
	#[test]
	fn magnitude_4() {
		test_magnitude("[[[[3,0],[5,3]],[4,4]],[5,5]]", 791);
	}
	#[test]
	fn magnitude_5() {
		test_magnitude("[[[[5,0],[7,4]],[5,5]],[6,6]]", 1137);
	}
	#[test]
	fn magnitude_6() {
		test_magnitude(
			"[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
			3488,
		);
	}
}
