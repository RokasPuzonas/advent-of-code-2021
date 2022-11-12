// Beautiful explanation: https://github.com/dphilipson/advent-of-code-2021/blob/master/src/days/day24.rs

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Register {
	X, Y, Z, W
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Operand {
	Register(Register),
	Number(i32)
}

#[derive(Debug, PartialEq, Eq)]
pub enum Instruction {
	Input(Register),
	Add(Register, Operand),
	Multiply(Register, Operand),
	Divide(Register, Operand),
	Modulo(Register, Operand),
	Equal(Register, Operand)
}

#[allow(clippy::upper_case_acronyms)]
pub struct CPU {
	rx: i32,
	ry: i32,
	rz: i32,
	rw: i32,
	input: Vec<i32>
}

impl CPU {
	fn new() -> CPU {
		CPU { rx: 0, ry: 0, rz: 0, rw: 0, input: vec![] }
	}

	fn reg(&mut self, reg: Register) -> &mut i32 {
		match reg {
			Register::X => &mut self.rx,
			Register::Y => &mut self.ry,
			Register::Z => &mut self.rz,
			Register::W => &mut self.rw,
		}
	}

	fn op(&mut self, op: Operand) -> i32 {
		match op {
			Operand::Register(reg) => *self.reg(reg),
			Operand::Number(num) => num,
		}
	}

	fn input(&mut self, reg: Register, num: i32) {
		*self.reg(reg) = num;
	}

	fn add(&mut self, reg: Register, op: Operand) {
		*self.reg(reg) += self.op(op);
	}

	fn multiply(&mut self, reg: Register, op: Operand) {
		*self.reg(reg) *= self.op(op);
	}

	fn divide(&mut self, reg: Register, op: Operand) {
		*self.reg(reg) /= self.op(op);
	}

	fn modulo(&mut self, reg: Register, op: Operand) {
		*self.reg(reg) %= self.op(op);
	}

	fn equal(&mut self, reg: Register, op: Operand) {
		let b = self.op(op);
		let a = self.reg(reg);
		*a = if *a == b { 1 } else { 0 };
	}

	fn run(&mut self, inst: &Instruction) {
		use Instruction::*;
		match *inst {
			Input(a) => {
				let b = self.input.pop().expect("Missing input");
				self.input(a, b)
			},
			Add(a, b)      => self.add(a, b),
			Multiply(a, b) => self.multiply(a, b),
			Divide(a, b)   => self.divide(a, b),
			Modulo(a, b)   => self.modulo(a, b),
			Equal(a, b)    => self.equal(a, b),
		}
	}
}

fn parse_register(input: &str) -> Register {
	match input {
		"x" => Register::X,
		"y" => Register::Y,
		"z" => Register::Z,
		"w" => Register::W,
		_ => panic!("Unknown variable '{}'", input)
	}
}

fn parse_operand(input: &str) -> Operand {
	if let Ok(number) = input.parse::<i32>() {
		Operand::Number(number)
	} else {
		Operand::Register(parse_register(input))
	}
}

pub fn parse_input(input: &str) -> Vec<Instruction> {
	let mut instructions = vec![];
	for line in input.lines() {
		if line.is_empty() { continue; }
		let parts = line.split(' ').collect::<Vec<_>>();
		let opcode = *parts.get(0).expect("Missing opcode");
		let instruction = match opcode {
			"inp" => {
				let op1 = parts.get(1).expect("Missing variable");
				Instruction::Input(parse_register(op1))
			},
			"add" => {
				let op1 = parts.get(1).expect("Missing variable");
				let op2 = parts.get(2).expect("Missing operand");
				Instruction::Add(parse_register(op1), parse_operand(op2))
			},
			"mul" => {
				let op1 = parts.get(1).expect("Missing variable");
				let op2 = parts.get(2).expect("Missing operand");
				Instruction::Multiply(parse_register(op1), parse_operand(op2))
			}
			"div" => {
				let op1 = parts.get(1).expect("Missing variable");
				let op2 = parts.get(2).expect("Missing operand");
				Instruction::Divide(parse_register(op1), parse_operand(op2))
			},
			"mod" => {
				let op1 = parts.get(1).expect("Missing variable");
				let op2 = parts.get(2).expect("Missing operand");
				Instruction::Modulo(parse_register(op1), parse_operand(op2))
			}
			"eql" => {
				let op1 = parts.get(1).expect("Missing variable");
				let op2 = parts.get(2).expect("Missing operand");
				Instruction::Equal(parse_register(op1), parse_operand(op2))
			},
			_ => panic!("Unexpected opcode '{}'", opcode)
		};
		instructions.push(instruction);
	}
	instructions
}

fn check(instructions: &[Instruction], monad: &[u8]) -> bool {
	let mut cpu = CPU::new();
	cpu.input = vec![];
	for num in monad {
		cpu.input.push(*num as i32);
	}
	cpu.input.reverse();
	for inst in instructions {
		cpu.run(inst);
	}
	cpu.rz == 0
}

fn analyze(instructions: &[Instruction]) -> Vec<(i32, i32, i32)> {
	let mut result = vec![];
	for (i, _) in instructions.iter().enumerate()
		.filter(|(_, inst)| **inst == Instruction::Input(Register::W)) {
		let a = match instructions.get(i+4) {
			Some(Instruction::Divide(_, Operand::Number(num))) => *num,
			_ => panic!()
		};

		let b = match instructions.get(i+5) {
			Some(Instruction::Add(_, Operand::Number(num))) => *num,
			_ => panic!()
		};

		let c = match instructions.get(i+15) {
			Some(Instruction::Add(_, Operand::Number(num))) => *num,
			_ => panic!()
		};

		result.push((a, b, c));
	}
	result
}

fn analyze_requirements(instructions: &[Instruction]) -> Vec<(u8, u8, i8)> {
	let mut requirements = vec![];
	let result = analyze(instructions);
	let mut stack = vec![];
	for (i, (_, check, offset)) in result.iter().enumerate() {
		if *check > 0 {
			stack.push((i, offset))
		} else {
			let top = stack.pop().unwrap();
			requirements.push((i as u8, top.0 as u8, (*check + top.1) as i8));
		}
	}
	requirements
}

fn concat_nums(nums: &[u8]) -> u64 {
	let mut answer = 0u64;
	for num in nums {
		answer *= 10;
		answer += *num as u64;
	}
	answer
}

pub fn part1(instructions: Vec<Instruction>) -> u64 {
	let mut monad = [9u8; 14];
	for (i, j, offset) in analyze_requirements(&instructions) {
		if offset > 0 {
			monad[j as usize] = (9 - offset) as u8;
		} else {
			monad[i as usize] = (9 + offset) as u8;
		}
	}

	assert!(check(&instructions, &monad));

	concat_nums(&monad)
}

pub fn part2(instructions: Vec<Instruction>) -> u64 {
	let mut monad = [1u8; 14];
	for (i, j, offset) in analyze_requirements(&instructions) {
		if offset > 0 {
			monad[i as usize] = (1 + offset) as u8;
		} else {
			monad[j as usize] = (1 - offset) as u8;
		}
	}

	assert!(check(&instructions, &monad));

	concat_nums(&monad)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn input_parsing() {
		use Instruction::*;
		use Register::*;
		let instructions = parse_input(&[
				"inp z",
				"inp x",
				"mul z 3",
				"eql z x",
			].join("\n"));
		assert_eq!(instructions, vec![
			Input(Z),
			Input(X),
			Multiply(Z, Operand::Number(3)),
			Equal(Z, Operand::Register(X))
		])
	}
}
