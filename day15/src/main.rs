use std::io::{self, Read};

use buckets::BucketList;
use hash::get_hash;
use instruction::execute;
use parse_input::{parse_instruction, to_steps};

mod buckets;
mod hash;
mod instruction;
mod parse_input;

#[must_use]
fn get_hash_sum(input: &str) -> usize {
	to_steps(input).into_iter().map(|s| get_hash(&s)).sum()
}

#[must_use]
fn get_power(input: &str) -> usize {
	let intructions = to_steps(input).into_iter().map(|s| parse_instruction(&s));
	let mut bucket_list = BucketList::new();
	for instruction in intructions {
		execute(&mut bucket_list, &instruction);
	}
	bucket_list.get_power()
}

#[cfg(test)]
mod test {
	use super::*;
	const SAMPLE_INPUT: &str = include_str!("../input_sample.txt");

	#[test]
	fn test_sample_hash() {
		assert_eq!(get_hash_sum(SAMPLE_INPUT), 1320);
	}

	#[test]
	fn test_sample_run() {
		assert_eq!(get_power(SAMPLE_INPUT), 145);
	}
}

fn main() {
	let mut input = String::new();
	io::stdin()
		.read_to_string(&mut input)
		.expect("Failed to read input");

	println!("Part 1: {}", get_hash_sum(&input));
	println!("Part 2: {}", get_power(&input));
}
