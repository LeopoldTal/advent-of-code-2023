use crate::maze::{Maze, Tile};

/// Recognises one tile.
#[must_use]
pub fn parse_tile(input: char) -> Tile {
	match input {
		'.' => Tile::Wall,
		'|' => Tile::NorthSouth,
		'-' => Tile::EastWest,
		'L' => Tile::NorthEast,
		'J' => Tile::NorthWest,
		'7' => Tile::SouthWest,
		'F' => Tile::SouthEast,
		'S' => Tile::Bunny,
		_ => panic!("Unrecognised character {input}"),
	}
}

/// Parses the whole input.
/// # Panics
/// On any parse error.
#[must_use]
pub fn parse_full(input: &str) -> Maze {
	let tiles: Vec<Vec<Tile>> = input
		.lines()
		.map(|line| line.chars().map(parse_tile).collect())
		.collect();
	let nb_rows = tiles.len();
	let nb_cols = tiles.first().expect("Empty maze").len();

	Maze {
		nb_rows,
		nb_cols,
		tiles,
	}
}

#[cfg(test)]
mod test {
	use super::*;
	const SAMPLE_INPUT_SIMPLE_BARE: &str = include_str!("../input_sample_simple_bare.txt");
	const SAMPLE_INPUT_COMPLEX_CROWDED: &str = include_str!("../input_sample_complex_crowded.txt");

	#[test]
	fn test_parse_simple() {
		let maze = parse_full(SAMPLE_INPUT_SIMPLE_BARE);
		let expected = "
     
 ♞━┓ 
 ┃ ┃ 
 ┗━┛ 
     
";
		assert_eq!(format!("{}", maze), expected);
	}

	#[test]
	fn test_parse_complex() {
		let maze = parse_full(SAMPLE_INPUT_COMPLEX_CROWDED);
		let expected = "
┓━┏┓━
 ┏┛┃┓
♞┛┗┗┓
┃┏━━┛
┗┛ ┗┛
";
		assert_eq!(format!("{}", maze), expected);
	}
}
