pub enum PacketBody {
	Literal(u64),
	Operator(Vec<Packet>),
}

pub struct Packet {
	version: u8,
	r#type: u8,
	body: PacketBody,
}

fn to_bits(hex: &str) -> String {
	let mut bits = String::new();
	for c in hex.bytes() {
		match c {
			b'0'..=b'9' => bits.push_str(format!("{:0>4b}", c - b'0').as_str()),
			b'A'..=b'F' => bits.push_str(format!("{:0>4b}", c - b'A' + 10).as_str()),
			_ => (),
		}
	}
	return bits;
}

fn parse_literal_body(bits_str: &str) -> (PacketBody, u32) {
	let mut value_bits = String::new();
	let mut cursor = 0;
	let bytes = bits_str.as_bytes();
	loop {
		value_bits.push_str(&bits_str[cursor + 1..cursor + 5]);
		if bytes[cursor] == b'0' {
			break;
		}
		cursor += 5;
	}
	let value = u64::from_str_radix(&value_bits, 2).unwrap();

	return (PacketBody::Literal(value), (cursor + 5) as u32);
}

fn parse_operator_body(bits_str: &str) -> (PacketBody, u32) {
	let mut size: usize = 0;
	let mut packets = Vec::new();
	let bytes = bits_str.as_bytes();
	size += 1;
	if bytes[0] == b'1' {
		size += 11;
		let count = u32::from_str_radix(&bits_str[1..12], 2).unwrap();
		for _ in 0..count {
			let (packet, s) = parse_packet(&bits_str[size..]);
			packets.push(packet);
			size += s as usize;
		}
	} else {
		let total_size = u32::from_str_radix(&bits_str[1..16], 2).unwrap();
		size += 15;
		while ((size - 16) as u32) < total_size {
			let (packet, s) = parse_packet(&bits_str[size..]);
			packets.push(packet);
			size += s as usize;
		}
	}
	return (PacketBody::Operator(packets), size as u32);
}

fn parse_packet(bits: &str) -> (Packet, u32) {
	let r#type = u8::from_str_radix(&bits[3..6], 2).unwrap();
	let (body, body_size) = match r#type {
		4 => parse_literal_body(&bits[6..]),
		_ => parse_operator_body(&bits[6..]),
	};

	return (
		Packet {
			version: u8::from_str_radix(&bits[0..3], 2).unwrap(),
			r#type,
			body,
		},
		body_size + 6,
	);
}

pub fn parse_input(input: &str) -> Packet {
	let (packet, _) = parse_packet(&to_bits(input));
	return packet;
}

fn sum_packet_versions(packet: &Packet) -> u32 {
	let mut sum: u32 = packet.version.into();
	match &packet.body {
		PacketBody::Operator(packets) => {
			for sub_packet in packets {
				sum += sum_packet_versions(sub_packet);
			}
		}
		_ => (),
	};
	return sum;
}

pub fn part1(packet: &Packet) -> u32 {
	sum_packet_versions(packet)
}

fn eval_sum_packets(packets: &[Packet]) -> u64 {
	let mut sum = 0;
	for packet in packets {
		sum += eval_packet(packet);
	}
	return sum;
}
fn eval_product_packets(packets: &[Packet]) -> u64 {
	let mut product = 1;
	for packet in packets {
		product *= eval_packet(packet);
	}
	return product;
}
fn eval_minimum_packets(packets: &[Packet]) -> u64 {
	let mut min = u64::MAX;
	for packet in packets {
		min = min.min(eval_packet(packet));
	}
	return min;
}
fn eval_maximum_packets(packets: &[Packet]) -> u64 {
	let mut max = 0;
	for packet in packets {
		max = max.max(eval_packet(packet));
	}
	return max;
}
fn eval_greater_packets(packets: &[Packet]) -> u64 {
	let first_packet = packets.get(0).unwrap();
	let second_packet = packets.get(1).unwrap();
	if eval_packet(first_packet) > eval_packet(second_packet) {
		1
	} else {
		0
	}
}
fn eval_less_packets(packets: &[Packet]) -> u64 {
	let first_packet = packets.get(0).unwrap();
	let second_packet = packets.get(1).unwrap();
	if eval_packet(first_packet) < eval_packet(second_packet) {
		1
	} else {
		0
	}
}
fn eval_equal_packets(packets: &[Packet]) -> u64 {
	let first_packet = packets.get(0).unwrap();
	let second_packet = packets.get(1).unwrap();
	if eval_packet(first_packet) == eval_packet(second_packet) {
		1
	} else {
		0
	}
}

fn eval_packet(packet: &Packet) -> u64 {
	match &packet.body {
		PacketBody::Literal(value) => *value,
		PacketBody::Operator(packets) => match packet.r#type {
			0 => eval_sum_packets(packets),
			1 => eval_product_packets(packets),
			2 => eval_minimum_packets(packets),
			3 => eval_maximum_packets(packets),
			5 => eval_greater_packets(packets),
			6 => eval_less_packets(packets),
			7 => eval_equal_packets(packets),
			_ => unreachable!(),
		},
	}
}

pub fn part2(packet: &Packet) -> u64 {
	eval_packet(packet)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn part1_example_1() {
		let packet = parse_input("8A004A801A8002F478");
		let result = part1(&packet);
		assert_eq!(result, 16);
	}

	#[test]
	fn part1_example_2() {
		let packet = parse_input("620080001611562C8802118E34");
		let result = part1(&packet);
		assert_eq!(result, 12);
	}

	#[test]
	fn part1_example_3() {
		let packet = parse_input("C0015000016115A2E0802F182340");
		let result = part1(&packet);
		assert_eq!(result, 23);
	}

	#[test]
	fn part1_example_4() {
		let packet = parse_input("A0016C880162017C3686B18A3D4780");
		let result = part1(&packet);
		assert_eq!(result, 31);
	}

	#[test]
	fn part2_example_1() {
		let packet = parse_input("C200B40A82");
		let result = part2(&packet);
		assert_eq!(result, 3);
	}

	#[test]
	fn part2_example_2() {
		let packet = parse_input("04005AC33890");
		let result = part2(&packet);
		assert_eq!(result, 54);
	}

	#[test]
	fn part2_example_3() {
		let packet = parse_input("880086C3E88112");
		let result = part2(&packet);
		assert_eq!(result, 7);
	}

	#[test]
	fn part2_example_4() {
		let packet = parse_input("CE00C43D881120");
		let result = part2(&packet);
		assert_eq!(result, 9);
	}

	#[test]
	fn part2_example_5() {
		let packet = parse_input("D8005AC2A8F0");
		let result = part2(&packet);
		assert_eq!(result, 1);
	}

	#[test]
	fn part2_example_6() {
		let packet = parse_input("F600BC2D8F");
		let result = part2(&packet);
		assert_eq!(result, 0);
	}

	#[test]
	fn part2_example_7() {
		let packet = parse_input("9C005AC2F8F0");
		let result = part2(&packet);
		assert_eq!(result, 0);
	}

	#[test]
	fn part2_example_8() {
		let packet = parse_input("9C0141080250320F1802104A08");
		let result = part2(&packet);
		assert_eq!(result, 1);
	}
}
