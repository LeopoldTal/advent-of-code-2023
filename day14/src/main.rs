use std::io::{self, Read};

use parse_input::parse_full;

mod board;
mod parse_input;
mod pretty;

#[must_use]
fn load_after_slide(input: &str, reporting: bool) -> usize {
	let mut board = parse_full(input);
	println!("{board}");
	board.slide_north(reporting);
	board.get_load()
}

#[must_use]
fn load_after_spins(input: &str, reporting: bool) -> usize {
	let mut board = parse_full(input);
	board.spin_many(reporting);
	board.get_load()
}

#[cfg(test)]
mod test {
	use super::*;
	const SAMPLE_INPUT: &str = include_str!("../input_sample.txt");
	const SAMPLE_1: &str = include_str!("../sample_after_1_spin.txt");
	const SAMPLE_2: &str = include_str!("../sample_after_2_spins.txt");
	const SAMPLE_3: &str = include_str!("../sample_after_3_spins.txt");

	#[test]
	fn test_sample_slide() {
		assert_eq!(load_after_slide(SAMPLE_INPUT, false), 136);
	}

	#[test]
	fn test_sample_first_spins() {
		let mut board = parse_full(SAMPLE_INPUT);

		let expected1 = parse_full(SAMPLE_1);
		board.spin_once(false);
		assert_eq!(board, expected1);

		let expected2 = parse_full(SAMPLE_2);
		board.spin_once(false);
		assert_eq!(board, expected2);

		let expected3 = parse_full(SAMPLE_3);
		board.spin_once(false);
		assert_eq!(board, expected3);
	}

	#[test]
	fn test_sample_spin_many() {
		assert_eq!(load_after_spins(SAMPLE_INPUT, false), 64);
	}
}

fn main() {
	let mut input = String::new();
	io::stdin()
		.read_to_string(&mut input)
		.expect("Failed to read input");

	println!("Part 1 — slide once: {}", load_after_slide(&input, true));
	println!(
		"Part 2 — spin a billion times: {}",
		load_after_spins(&input, true)
	);
}
