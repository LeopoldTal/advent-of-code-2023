use std::io::{self, Read};

use graph::{ends_with, exact};

use crate::parse_input::parse_full;

mod arithmetic;
mod cycle_combiner;
mod cycle_detector;
mod graph;
mod parse_input;

#[must_use]
fn traverse_single(s: &str) -> usize {
	let game = parse_full(s);
	let nb_steps = game.get_steps_to_all_goals(&exact("AAA"), &exact("ZZZ"));
	nb_steps
}

#[must_use]
fn traverse_multiple(s: &str) -> usize {
	let game = parse_full(s);
	let nb_steps = game.get_steps_to_all_goals(&ends_with("A"), &ends_with("Z"));
	nb_steps
}

#[cfg(test)]
mod test {
	use super::*;
	const SAMPLE_INPUT_SINGLE_PASS: &str = include_str!("../input_sample_a.txt");
	const SAMPLE_INPUT_REPEATED: &str = include_str!("../input_sample_b.txt");
	const SAMPLE_INPUT_MULTIPLE: &str = include_str!("../input_sample_multi.txt");

	#[test]
	fn test_sample_single() {
		assert_eq!(traverse_single(SAMPLE_INPUT_SINGLE_PASS), 2);
		assert_eq!(traverse_single(SAMPLE_INPUT_REPEATED), 6);
	}

	#[test]
	fn test_sample_multiple() {
		assert_eq!(traverse_multiple(SAMPLE_INPUT_SINGLE_PASS), 2);
		assert_eq!(traverse_multiple(SAMPLE_INPUT_REPEATED), 6);
		assert_eq!(traverse_multiple(SAMPLE_INPUT_MULTIPLE), 6);
	}
}

fn main() {
	let mut input = String::new();
	io::stdin()
		.read_to_string(&mut input)
		.expect("Failed to read input");

	println!("Single path (Part 1): {}", traverse_single(&input));
	println!("Multi-path (Part 2): {}", traverse_multiple(&input));
}
