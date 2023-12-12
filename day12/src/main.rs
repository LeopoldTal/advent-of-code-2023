use std::io::{self, Read};

use picross::PointRow;

use crate::parse_input::parse_full;

mod parse_input;
mod picross;

#[must_use]
fn get_total_arrangements(rows: &[PointRow]) -> usize {
	rows.iter().map(PointRow::get_arrangements_count).sum()
}

#[must_use]
fn get_total_unfolded_arrangements(rows: &[PointRow]) -> usize {
	let unfolded: Vec<PointRow> = rows.iter().map(PointRow::unfold).collect();
	get_total_arrangements(&unfolded)
}

#[cfg(test)]
mod test {
	use super::*;
	const SAMPLE_INPUT: &str = include_str!("../input_sample.txt");

	#[test]
	fn test_sample_lines() {
		let rows = parse_full(SAMPLE_INPUT);
		let counts: Vec<usize> = rows
			.into_iter()
			.map(|row| row.get_arrangements_count())
			.collect();
		let expected = vec![1, 4, 1, 1, 4, 10];
		assert_eq!(counts, expected);
	}

	#[test]
	fn test_sample_folded() {
		assert_eq!(get_total_arrangements(&parse_full(SAMPLE_INPUT)), 21);
	}

	// #[test]
	// fn test_sample_unfolded() {
	// 	assert_eq!(
	// 		get_total_unfolded_arrangements(&parse_full(SAMPLE_INPUT)),
	// 		525152
	// 	);
	// }
}

fn main() {
	let mut input = String::new();
	io::stdin()
		.read_to_string(&mut input)
		.expect("Failed to read input");

	let folded = parse_full(&input);
	println!("Folded: {}", get_total_arrangements(&folded));
	println!("Unfolded: {}", get_total_unfolded_arrangements(&folded));
}
