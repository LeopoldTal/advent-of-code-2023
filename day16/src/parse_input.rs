use crate::board::{Board, Optics, Tile};

/// Reads one tile.
#[must_use]
fn read_tile(c: char) -> Tile {
	let optics = match c {
		'.' => Optics::Empty,
		'/' => Optics::MirrorΓ,
		'\\' => Optics::MirrorL,
		'-' => Optics::SplitterH,
		'|' => Optics::SplitterV,
		_ => unreachable!(),
	};
	Tile::from(optics)
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
	use crate::parse_input::Optics::{Empty, MirrorL, MirrorΓ, SplitterH, SplitterV};

	#[test]
	fn test_parse() {
		let input = ".\\/\n.|-\n";
		let board = parse_full(input);

		assert_eq!(board.nb_rows, 2);
		assert_eq!(board.nb_cols, 3);

		assert_eq!(board.tiles[0][0].optics, Empty);
		assert_eq!(board.tiles[0][1].optics, MirrorL);
		assert_eq!(board.tiles[0][2].optics, MirrorΓ);

		assert_eq!(board.tiles[1][0].optics, Empty);
		assert_eq!(board.tiles[1][1].optics, SplitterV);
		assert_eq!(board.tiles[1][2].optics, SplitterH);

		let expected = Board::from(vec![
			vec![Tile::from(Empty), Tile::from(MirrorL), Tile::from(MirrorΓ)],
			vec![
				Tile::from(Empty),
				Tile::from(SplitterV),
				Tile::from(SplitterH),
			],
		]);
		assert_eq!(parse_full(input), expected);
	}
}
