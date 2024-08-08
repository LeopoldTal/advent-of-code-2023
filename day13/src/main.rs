use std::io::{self, Read};

use grid::{get_reflect, get_reflect_with_flip};
use parse_input::parse_full;

mod grid;
mod parse_input;

/// Part 1: sums the reflection indices of all grids, weighted by horizontal vs vertical.
#[must_use]
fn get_index_sum(input: &str) -> usize {
	let grids = parse_full(input);
	grids.into_iter().map(|grid| get_reflect(&grid)).sum()
}

/// Part 2: sums the reflection indices of all grids with 1 flip, weighted by horizontal vs vertical.
#[must_use]
fn get_flipped_sum(input: &str) -> usize {
	let grids = parse_full(input);
	grids
		.into_iter()
		.map(|grid| get_reflect_with_flip(&grid))
		.sum()
}

#[cfg(test)]
mod test {
	use super::*;
	const SAMPLE_INPUT: &str = include_str!("../input_sample.txt");

	#[test]
	fn test_sample() {
		assert_eq!(get_index_sum(SAMPLE_INPUT), 405);
		assert_eq!(get_flipped_sum(SAMPLE_INPUT), 400);
	}
}

fn main() {
	let mut input = String::new();
	io::stdin()
		.read_to_string(&mut input)
		.expect("Failed to read input");

	println!("Part 1: {}", get_index_sum(&input));
	println!("Part 2: {}", get_flipped_sum(&input));
}
