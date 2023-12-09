use std::io::{self, Read};

use parse_input::parse_full;
use sequence::extrapolate;

mod parse_input;
mod sequence;

#[must_use]
fn extrapolate_all(sequences: &[Vec<i64>], backwards: bool) -> i64 {
	sequences.iter().map(|l| extrapolate(l, backwards)).sum()
}

#[cfg(test)]
mod test {
	use super::*;
	const SAMPLE_INPUT: &str = include_str!("../input_sample.txt");

	#[test]
	fn test_sample_forwards() {
		let sequences = parse_full(SAMPLE_INPUT);
		assert_eq!(extrapolate_all(&sequences, false), 114);
	}

	#[test]
	fn test_sample_backwards() {
		let sequences = parse_full(SAMPLE_INPUT);
		assert_eq!(extrapolate_all(&sequences, true), 2);
	}
}

fn main() {
	let mut input = String::new();
	io::stdin()
		.read_to_string(&mut input)
		.expect("Failed to read input");

	let sequences = parse_full(&input);
	println!(
		"Extrapolate forwards: {}",
		extrapolate_all(&sequences, false)
	);
	println!(
		"Extrapolate backwards: {}",
		extrapolate_all(&sequences, true)
	);
}
