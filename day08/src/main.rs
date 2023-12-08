use std::io::{self, Read};

use crate::parse_input::parse_full;

mod graph;
mod parse_input;

#[must_use]
fn traverse(s: &str) -> usize {
	let game = parse_full(s);
	game.run("AAA", "ZZZ")
}

#[cfg(test)]
mod test {
	use super::*;
	const SAMPLE_INPUT_SINGLE_PASS: &str = include_str!("../input_sample_a.txt");
	const SAMPLE_INPUT_REPEATED: &str = include_str!("../input_sample_b.txt");

	#[test]
	fn test_sample() {
		assert_eq!(traverse(SAMPLE_INPUT_SINGLE_PASS), 2);
		assert_eq!(traverse(SAMPLE_INPUT_REPEATED), 6);
	}
}

fn main() {
	let mut input = String::new();
	io::stdin()
		.read_to_string(&mut input)
		.expect("Failed to read input");

	println!("Steps: {}", traverse(&input));
}
