#![allow(mixed_script_confusables)] // Using Γ to name a mirror type

use std::io::{self, Read};

use board::{Beam, Board, Direction};
use parse_input::parse_full;

mod board;
mod parse_input;
mod pretty;

#[must_use]
fn find_most_lit(initial_board: &Board, beams: &[Beam]) -> (Board, usize) {
	let mut best_board = initial_board.clone();
	let mut best_lit = 0;

	for &beam in beams {
		let mut board = initial_board.clone();
		board.trace(beam);
		let lit = board.count_lit_tiles();
		if lit > best_lit {
			best_board = board;
			best_lit = lit;
		}
	}

	(best_board, best_lit)
}

#[must_use]
fn get_lit_from_top_left(input: &str) -> usize {
	let mut board = parse_full(input);
	let initial_beam = Beam {
		row: 0,
		col: 0,
		direction: Direction::Right,
	};
	board.trace(initial_beam);
	println!("{board}");
	board.count_lit_tiles()
}

#[must_use]
fn get_most_lit(input: &str) -> usize {
	let board = parse_full(input);

	let right_beams = (0..board.nb_rows).map(|row| Beam {
		row,
		col: 0,
		direction: Direction::Right,
	});
	let left_beams = (0..board.nb_rows).map(|row| Beam {
		row,
		col: board.nb_cols - 1,
		direction: Direction::Left,
	});
	let down_beams = (0..board.nb_cols).map(|col| Beam {
		row: 0,
		col,
		direction: Direction::Down,
	});
	let up_beams = (0..board.nb_cols).map(|col| Beam {
		row: board.nb_rows - 1,
		col,
		direction: Direction::Up,
	});

	let beams: Vec<Beam> = up_beams
		.chain(down_beams)
		.chain(left_beams)
		.chain(right_beams)
		.collect();

	let (best_board, best_lit) = find_most_lit(&board, &beams);
	println!("{best_board}");
	best_lit
}

#[cfg(test)]
mod test {
	use super::*;
	const SAMPLE_INPUT: &str = include_str!("../input_sample.txt");

	#[test]
	fn test_sample() {
		assert_eq!(get_lit_from_top_left(SAMPLE_INPUT), 46);
		assert_eq!(get_most_lit(SAMPLE_INPUT), 51);
	}
}

fn main() {
	let mut input = String::new();
	io::stdin()
		.read_to_string(&mut input)
		.expect("Failed to read input");

	println!("Part 1 — from top left: {}", get_lit_from_top_left(&input));
	println!("Part 2 — most lit: {}", get_most_lit(&input));
}
