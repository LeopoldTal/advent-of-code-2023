use board::{parse, Board};
use std::io::{self, Read};

mod board;

#[must_use]
fn sum_part_numbers(board: &Board) -> u32 {
	board.get_part_numbers().into_iter().sum()
}

#[cfg(test)]
mod test {
	use super::*;
	const SAMPLE_INPUT: &str = include_str!("../input_sample.txt");

	#[test]
	fn test_sum_part_numbers() {
		let board = parse(&SAMPLE_INPUT);
		assert_eq!(sum_part_numbers(&board), 4361);
	}
}

fn main() {
	let mut input = String::new();
	io::stdin()
		.read_to_string(&mut input)
		.expect("Failed to read input");

	let board = parse(&input);

	println!("Part numbers sum: {}", sum_part_numbers(&board));
}
