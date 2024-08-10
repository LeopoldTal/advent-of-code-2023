use std::io::{self, Read};

use board::Board;
use constraints::{Constraints, CONSTRAINTS_PART_1, CONSTRAINTS_PART_2};
use parse_input::parse_full;
use pathfinding::find_path;

mod board;
mod constraints;
mod parse_input;
mod pathfinding;

#[must_use]
fn get_distance(input: &str, constraints: Constraints) -> u32 {
	let tiles = parse_full(input);
	let board = Board::from(tiles, constraints);
	find_path(&board, (0, 0), (board.nb_rows - 1, board.nb_cols - 1))
}

#[must_use]
fn get_distance_part_1(input: &str) -> u32 {
	get_distance(input, CONSTRAINTS_PART_1)
}

#[must_use]
fn get_distance_part_2(input: &str) -> u32 {
	get_distance(input, CONSTRAINTS_PART_2)
}

#[cfg(test)]
mod test {
	use super::*;
	const SAMPLE_INPUT: &str = include_str!("../input_sample.txt");
	const SAMPLE_INPUT_PART_2: &str = include_str!("../input_sample_part2.txt");

	#[test]
	fn test_sample() {
		assert_eq!(get_distance_part_1(SAMPLE_INPUT), 102);
		assert_eq!(get_distance_part_2(SAMPLE_INPUT), 94);
	}

	#[test]
	fn test_force_bad_turn() {
		assert_eq!(get_distance_part_2(SAMPLE_INPUT_PART_2), 71);
	}
}

fn main() {
	let mut input = String::new();
	io::stdin()
		.read_to_string(&mut input)
		.expect("Failed to read input");

	println!(
		"Part 1 — straight line max 3, no min: {}",
		get_distance_part_1(&input)
	);
	println!(
		"Part 2 — straight line max 10, min 4: {}",
		get_distance_part_2(&input)
	);
}
