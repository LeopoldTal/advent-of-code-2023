use crate::starfield::{Coords, Starfield};
use std::collections::HashSet;

/// Parses the whole input.
/// # Panics
/// On any parse error.
#[must_use]
pub fn parse_full(input: &str) -> Starfield {
	let mut galaxies: HashSet<Coords> = HashSet::new();
	for (row, line) in input.lines().enumerate() {
		for (col, ch) in line.chars().enumerate() {
			if ch == '#' {
				galaxies.insert((row, col));
			}
		}
	}
	Starfield::from(galaxies)
}

#[cfg(test)]
mod test {

	use super::*;

	#[test]
	fn test_parse() {
		let input = "#...\n.#..\n...#\n";
		let starfield = parse_full(input);
		let coords = HashSet::from([(0, 0), (1, 1), (2, 3)]);
		assert_eq!(starfield.galaxies, coords);
	}
}
