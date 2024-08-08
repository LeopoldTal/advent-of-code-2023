use crate::grid::Grid;

/// Reads one tile.
#[must_use]
fn read_tile(c: char) -> bool {
	match c {
		'.' => false,
		'#' => true,
		_ => unreachable!(),
	}
}

/// Reads a board from a string.
#[must_use]
fn parse_grid(s: &str) -> Grid {
	let tiles: Vec<Vec<bool>> = s
		.lines()
		.map(|line| line.chars().map(read_tile).collect())
		.collect();
	Grid {
		nb_rows: tiles.len(),
		nb_cols: tiles.first().expect("Empty grid").len(),
		tiles,
	}
}

/// Parses the whole input.
/// # Panics
/// On any parse error.
#[must_use]
pub fn parse_full(input: &str) -> Vec<Grid> {
	input.split("\n\n").map(parse_grid).collect()
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_parse() {
		let input = "...\n###\n\n.#\n";
		let expected = vec![
			Grid {
				nb_rows: 2,
				nb_cols: 3,
				tiles: vec![vec![false; 3], vec![true; 3]],
			},
			Grid {
				nb_rows: 1,
				nb_cols: 2,
				tiles: vec![vec![false, true]],
			},
		];
		assert_eq!(parse_full(input), expected);
	}
}
