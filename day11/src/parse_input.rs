use std::collections::BTreeMap;

use crate::starfield::{DimCount, Starfield};

/// Parses the whole input.
/// # Panics
/// On any parse error.
#[must_use]
pub fn parse_full(input: &str) -> Starfield {
	let mut nb_galaxies = 0;
	let mut nb_per_row: DimCount = BTreeMap::new();
	let mut nb_per_col: DimCount = BTreeMap::new();
	for (row, line) in input.lines().enumerate() {
		for (col, ch) in line.chars().enumerate() {
			if ch == '#' {
				nb_galaxies += 1;
				*nb_per_row.entry(row).or_insert(0) += 1;
				*nb_per_col.entry(col).or_insert(0) += 1;
			}
		}
	}
	Starfield {
		nb_galaxies,
		nb_per_row,
		nb_per_col,
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_parse() {
		let input = "#...\n.#..\n#..#\n";
		let starfield = parse_full(input);
		let expected = Starfield {
			nb_galaxies: 4,
			nb_per_row: BTreeMap::from([(0, 1), (1, 1), (2, 2)]),
			nb_per_col: BTreeMap::from([(0, 2), (1, 1), (3, 1)]),
		};
		assert_eq!(starfield, expected);
	}
}
