use crate::board::{Board, Tile};

/// Reads one tile.
#[must_use]
fn read_tile(c: char) -> Tile {
	match c {
		'.' => Tile::Empty,
		'#' => Tile::Wall,
		'O' => Tile::Movable,
		_ => unreachable!(),
	}
}

/// Parses the whole input.
/// # Panics
/// On any parse error.
#[must_use]
pub fn parse_full(input: &str) -> Board {
	let tiles: Vec<Vec<Tile>> = input
		.lines()
		.map(|line| line.chars().map(read_tile).collect())
		.collect();
	Board::from(tiles)
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::parse_input::Tile::{Empty, Movable, Wall};

	#[test]
	fn test_parse() {
		let input = "...\n#O#\n";
		let expected = Board::from(vec![vec![Empty; 3], vec![Wall, Movable, Wall]]);
		assert_eq!(parse_full(input), expected);
	}
}
