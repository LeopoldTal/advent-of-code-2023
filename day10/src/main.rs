use std::io::{self, Read};

use maze::get_max_distance;
use pretty_maze::pretty_print;

use crate::parse_input::parse_full;

mod maze;
mod parse_input;
mod pretty_maze;

#[must_use]
fn count_steps(input: &str, show: bool) -> usize {
	let maze = parse_full(input);
	let path = maze.get_loop();
	if show {
		pretty_print(&maze, &path);
	}
	get_max_distance(&path)
}

#[cfg(test)]
mod test {
	use super::*;
	const SAMPLE_INPUT_SIMPLE_BARE: &str = include_str!("../input_sample_simple_bare.txt");
	const SAMPLE_INPUT_SIMPLE_CROWDED: &str = include_str!("../input_sample_simple_crowded.txt");
	const SAMPLE_INPUT_COMPLEX_BARE: &str = include_str!("../input_sample_complex_bare.txt");
	const SAMPLE_INPUT_COMPLEX_CROWDED: &str = include_str!("../input_sample_complex_crowded.txt");

	#[test]
	fn test_sample() {
		assert_eq!(count_steps(SAMPLE_INPUT_SIMPLE_BARE, false), 4);
		assert_eq!(count_steps(SAMPLE_INPUT_SIMPLE_CROWDED, false), 4);
		assert_eq!(count_steps(SAMPLE_INPUT_COMPLEX_BARE, false), 8);
		assert_eq!(count_steps(SAMPLE_INPUT_COMPLEX_CROWDED, false), 8);
	}
}

fn main() {
	let mut input = String::new();
	io::stdin()
		.read_to_string(&mut input)
		.expect("Failed to read input");

	let distance = count_steps(&input, true);
	println!("Steps: {distance}");
}
