use std::io::{self, Read};

use dual_maze::DualMaze;
use maze::{get_max_distance, Coords};
use pretty_maze::pretty_print;

use crate::parse_input::parse_full;

mod dual_maze;
mod maze;
mod parse_input;
mod pretty_maze;

#[must_use]
fn count_steps(input: &str, show: bool) -> (usize, usize) {
	let maze = parse_full(input);
	let path = maze.get_loop();
	let path_coords: Vec<Coords> = path.iter().map(|&tile| (tile.row, tile.col)).collect();

	let dual = DualMaze::from(maze.nb_rows, maze.nb_cols, &path_coords);
	let inside = dual.get_enclosed_tiles();

	if show {
		pretty_print(&maze, &path, &inside);
	}
	(get_max_distance(&path), inside.len())
}

#[cfg(test)]
mod test {
	use super::*;
	use samples::*;

	#[test]
	fn test_sample() {
		assert_eq!(count_steps(SAMPLE_INPUT_SIMPLE_BARE, false), (4, 1));
		assert_eq!(count_steps(SAMPLE_INPUT_SIMPLE_CROWDED, false), (4, 1));
		assert_eq!(count_steps(SAMPLE_INPUT_COMPLEX_BARE, false), (8, 1));
		assert_eq!(count_steps(SAMPLE_INPUT_COMPLEX_CROWDED, false), (8, 1));
		assert_eq!(count_steps(SAMPLE_INPUT_ENCLOSED_OPEN, false), (23, 4));
		assert_eq!(count_steps(SAMPLE_INPUT_ENCLOSED_NARROW, false), (22, 4));
		assert_eq!(count_steps(SAMPLE_INPUT_ENCLOSED_MEDIUM, false), (70, 8));
		assert_eq!(count_steps(SAMPLE_INPUT_ENCLOSED_CROWDED, false), (80, 10));
	}
}

fn main() {
	let mut input = String::new();
	io::stdin()
		.read_to_string(&mut input)
		.expect("Failed to read input");

	let (distance, area) = count_steps(&input, true);
	println!("Steps: {distance}");
	println!("Enclosed area: {area}");
}

#[cfg(test)]
mod samples {
	pub const SAMPLE_INPUT_SIMPLE_BARE: &str = include_str!("../input_sample_simple_bare.txt");
	pub const SAMPLE_INPUT_SIMPLE_CROWDED: &str =
		include_str!("../input_sample_simple_crowded.txt");
	pub const SAMPLE_INPUT_COMPLEX_BARE: &str = include_str!("../input_sample_complex_bare.txt");
	pub const SAMPLE_INPUT_COMPLEX_CROWDED: &str =
		include_str!("../input_sample_complex_crowded.txt");
	pub const SAMPLE_INPUT_ENCLOSED_OPEN: &str = include_str!("../input_sample_enclosed_open.txt");
	pub const SAMPLE_INPUT_ENCLOSED_NARROW: &str =
		include_str!("../input_sample_enclosed_narrow.txt");
	pub const SAMPLE_INPUT_ENCLOSED_MEDIUM: &str =
		include_str!("../input_sample_enclosed_medium.txt");
	pub const SAMPLE_INPUT_ENCLOSED_CROWDED: &str =
		include_str!("../input_sample_enclosed_crowded.txt");
}
