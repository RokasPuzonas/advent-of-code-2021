use std::{num::ParseIntError, slice::Iter};

pub fn parse_input(input: &str) -> Result<Vec<i32>, ParseIntError> {
    input.split_whitespace()
        .map(|s| i32::from_str_radix(s, 2))
        .collect()
}

fn calculate_max_bits(nums: &[i32]) -> u32 {
    let mut max_bits = 0;
    for num in nums {
        max_bits = max_bits.max((*num as f32).log2().ceil() as u32);
    }
    return max_bits;
}

fn count_bits(nums: Iter<i32>, power: &i32) -> u32 {
    let mut bits = 0;
    for num in nums {
        if num & power > 0 {
            bits += 1;
        }
    }
    return bits;
}

pub fn part1(diagnostics: &[i32]) -> i32 {
    let n = diagnostics.len() as u32;
    let mut gamma = 0;
    let mut epsilon = 0;

    let max_bits = calculate_max_bits(&diagnostics);
    let mut power = 1;
    for _ in 1..=max_bits {
        let bits = count_bits(diagnostics.iter(), &power);
        if 2*bits >= n {
            gamma += power;
        } else {
            epsilon += power;
        }
        power *= 2;
    }

    return gamma * epsilon;
}

pub fn part2(diagnostics: &[i32]) -> i32 {
    let mut carbon_diagnostics = Vec::new();
    let mut oxygen_diagnostics = Vec::new();

    carbon_diagnostics.extend_from_slice(diagnostics);
    oxygen_diagnostics.extend_from_slice(diagnostics);

    let max_bits = calculate_max_bits(&diagnostics);
    let mut power = 2i32.pow(max_bits-1);
    for _ in 1..=max_bits {
        let oxygen_len = oxygen_diagnostics.len() as u32;
        if oxygen_len > 1 {
            let bit_count = count_bits(oxygen_diagnostics.iter(), &power);
            if 2*bit_count >= oxygen_len {
                oxygen_diagnostics = oxygen_diagnostics.into_iter().filter(|n| n & power > 0).collect();
            } else {
                oxygen_diagnostics = oxygen_diagnostics.into_iter().filter(|n| n & power == 0).collect();
            }
        }

        let carbon_len = carbon_diagnostics.len() as u32;
        if carbon_len > 1 {
            let bit_count = count_bits(carbon_diagnostics.iter(), &power);
            if 2*bit_count < carbon_len {
                carbon_diagnostics = carbon_diagnostics.into_iter().filter(|n| n & power > 0).collect();
            } else {
                carbon_diagnostics = carbon_diagnostics.into_iter().filter(|n| n & power == 0).collect();
            }
        }

        if oxygen_len == 1 && carbon_len == 1 { break; }

        power /= 2;
    }

    return carbon_diagnostics[0] * oxygen_diagnostics[0];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let diagnostics = [0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000, 0b11001, 0b00010, 0b01010];
        let result = part1(&diagnostics);
        assert_eq!(result, 198);
    }

    #[test]
    fn part2_example() {
        let diagnostics = [0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000, 0b11001, 0b00010, 0b01010];
        let result = part2(&diagnostics);
        assert_eq!(result, 230);
    }
}

