use std::io::{self, Read};

use parse_input::parse_full;

mod parse_input;
mod starfield;

#[must_use]
fn get_sum_distances(input: &str, expand_factor: usize) -> i64 {
	let starfield = parse_full(input);
	let expanded = starfield.expand(expand_factor);
	expanded.get_sum_distances()
}

#[cfg(test)]
mod test {
	use super::*;
	const SAMPLE_INPUT: &str = include_str!("../input_sample.txt");

	#[test]
	fn test_sample_expand_2() {
		assert_eq!(get_sum_distances(SAMPLE_INPUT, 2), 374);
	}

	#[test]
	fn test_sample_expand_10() {
		assert_eq!(get_sum_distances(SAMPLE_INPUT, 10), 1030);
	}

	#[test]
	fn test_sample_expand_100() {
		assert_eq!(get_sum_distances(SAMPLE_INPUT, 100), 8410);
	}
}

fn main() {
	let mut input = String::new();
	io::stdin()
		.read_to_string(&mut input)
		.expect("Failed to read input");

	println!("Expand by 2: {}", get_sum_distances(&input, 2));
	println!(
		"Expand by a million: {}",
		get_sum_distances(&input, 1_000_000)
	);
}
