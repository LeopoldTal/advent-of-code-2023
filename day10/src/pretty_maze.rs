use std::collections::HashMap;

use crate::maze::{get_max_distance, Coords, Maze, Step, Tile};

type AnsiColour = u8;

/// Applies a pretty terminal colour.
#[must_use]
pub fn colourise(s: &str, background: AnsiColour, foreground: AnsiColour) -> String {
	format!("\x1b[38;5;{foreground};48;5;{background}m{s}\x1b[0m")
}

/// Displays a maze with the given path highlighted.
pub fn pretty_print(maze: &Maze, path: &[Step]) {
	let max_distance = get_max_distance(path) + 1;
	let path: HashMap<Coords, usize> = path
		.iter()
		.map(|step| ((step.row, step.col), step.distance))
		.collect();

	println!();
	for row in 0..maze.nb_rows {
		for col in 0..maze.nb_cols {
			print!(
				"{}",
				pretty_tile((row, col), maze.tiles[row][col], &path, max_distance)
			);
		}
		println!();
	}
	println!();
}

/// Formats a single tile, with a highlight colour if it's on the path.
fn pretty_tile(
	coords: Coords,
	tile: Tile,
	path: &HashMap<Coords, usize>,
	max_distance: usize,
) -> String {
	let symbol = format!("{tile}");
	if let Some(distance) = path.get(&coords) {
		let colours: Vec<AnsiColour> = (172..=177).collect();
		let scaled_index = colours.len() * distance / max_distance;
		let foreground = colours[scaled_index];
		let background = if tile == Tile::Bunny { 124 } else { 231 };
		colourise(&symbol, background, foreground)
	} else {
		colourise(&symbol, 255, 0)
	}
}
