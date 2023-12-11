use std::collections::BTreeMap;

pub type DimCount = BTreeMap<usize, usize>;

/// List of sorted galaxy positions.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Starfield {
	pub nb_galaxies: usize,
	pub nb_per_row: DimCount,
	pub nb_per_col: DimCount,
}

impl Starfield {
	/// Adjusts distances so unoccupied rows and columns are twice as wide.
	#[must_use]
	pub fn expand(&self, expand_factor: usize) -> Starfield {
		let rows = expand_axis_distances(&self.nb_per_row, expand_factor);
		let cols = expand_axis_distances(&self.nb_per_col, expand_factor);
		Starfield {
			nb_galaxies: self.nb_galaxies,
			nb_per_row: rows,
			nb_per_col: cols,
		}
	}

	/// Adds together all distances between pairs of galaxies.
	#[must_use]
	pub fn get_sum_distances(&self) -> i64 {
		get_sum_axis_distances(self.nb_galaxies, &self.nb_per_row)
			+ get_sum_axis_distances(self.nb_galaxies, &self.nb_per_col)
	}
}

/// Expands space along one dimension.
#[must_use]
fn expand_axis_distances(coords: &DimCount, expand_factor: usize) -> DimCount {
	coords
		.iter()
		.enumerate()
		.map(|(occupied_index, (&coord, &nb_galaxies))| {
			(
				expand_distance(coord, occupied_index, expand_factor),
				nb_galaxies,
			)
		})
		.collect()
}

/// Expands space between `0` and `index`.
#[must_use]
fn expand_distance(coord: usize, occupied_index: usize, expand_factor: usize) -> usize {
	let count_unoccupied = coord - occupied_index;
	coord + (expand_factor - 1) * count_unoccupied
}

/// Adds together all distances between pairs of galaxies along one dimension.
#[must_use]
pub fn get_sum_axis_distances(nb_galaxies: usize, coords: &DimCount) -> i64 {
	let mut total = 0;
	let mut weight = 1 - i64::try_from(nb_galaxies).expect("Too many galaxies");
	for (&coord, &nb_at_coord) in coords {
		for _ in 0..nb_at_coord {
			total += weight * i64::try_from(coord).expect("Grid too big");
			weight += 2;
		}
	}
	total
}

#[cfg(test)]
mod test_expand {
	use super::*;

	#[test]
	fn test_trivial() {
		let starfield = Starfield {
			nb_galaxies: 0,
			nb_per_row: BTreeMap::from([]),
			nb_per_col: BTreeMap::from([]),
		};
		let expected = Starfield {
			nb_galaxies: 0,
			nb_per_row: BTreeMap::from([]),
			nb_per_col: BTreeMap::from([]),
		};
		assert_eq!(starfield.expand(2), expected);
	}

	#[test]
	fn test_all_occupied() {
		let starfield = Starfield {
			nb_galaxies: 4,
			nb_per_row: BTreeMap::from([(0, 2), (1, 2)]),
			nb_per_col: BTreeMap::from([(0, 2), (1, 2)]),
		};
		let expected = Starfield {
			nb_galaxies: 4,
			nb_per_row: BTreeMap::from([(0, 2), (1, 2)]),
			nb_per_col: BTreeMap::from([(0, 2), (1, 2)]),
		};
		assert_eq!(starfield.expand(2), expected);
	}

	#[test]
	fn test_row_cols_occupied() {
		let starfield = Starfield {
			nb_galaxies: 2,
			nb_per_row: BTreeMap::from([(0, 1), (1, 1)]),
			nb_per_col: BTreeMap::from([(0, 1), (1, 1)]),
		};
		let expected = Starfield {
			nb_galaxies: 2,
			nb_per_row: BTreeMap::from([(0, 1), (1, 1)]),
			nb_per_col: BTreeMap::from([(0, 1), (1, 1)]),
		};
		assert_eq!(starfield.expand(2), expected);
	}

	#[test]
	fn test_empty_row() {
		let starfield = Starfield {
			nb_galaxies: 2,
			nb_per_row: BTreeMap::from([(0, 1), (2, 1)]),
			nb_per_col: BTreeMap::from([(0, 2)]),
		};
		let expected = Starfield {
			nb_galaxies: 2,
			nb_per_row: BTreeMap::from([(0, 1), (3, 1)]),
			nb_per_col: BTreeMap::from([(0, 2)]),
		};
		assert_eq!(starfield.expand(2), expected);
	}

	#[test]
	fn test_empty_col() {
		let starfield = Starfield {
			nb_galaxies: 2,
			nb_per_row: BTreeMap::from([(0, 2)]),
			nb_per_col: BTreeMap::from([(0, 1), (2, 1)]),
		};
		let expected = Starfield {
			nb_galaxies: 2,
			nb_per_row: BTreeMap::from([(0, 2)]),
			nb_per_col: BTreeMap::from([(0, 1), (3, 1)]),
		};
		assert_eq!(starfield.expand(2), expected);
	}

	#[test]
	fn test_empty_row_col() {
		let starfield = Starfield {
			nb_galaxies: 2,
			nb_per_row: BTreeMap::from([(0, 1), (2, 1)]),
			nb_per_col: BTreeMap::from([(0, 1), (2, 1)]),
		};
		let expected = Starfield {
			nb_galaxies: 2,
			nb_per_row: BTreeMap::from([(0, 1), (3, 1)]),
			nb_per_col: BTreeMap::from([(0, 1), (3, 1)]),
		};
		assert_eq!(starfield.expand(2), expected);
	}

	#[test]
	fn test_long_gap() {
		let starfield = Starfield {
			nb_galaxies: 2,
			nb_per_row: BTreeMap::from([(0, 2)]),
			nb_per_col: BTreeMap::from([(0, 1), (11, 1)]),
		};
		let expected = Starfield {
			nb_galaxies: 2,
			nb_per_row: BTreeMap::from([(0, 2)]),
			nb_per_col: BTreeMap::from([(0, 1), (21, 1)]),
		};
		assert_eq!(starfield.expand(2), expected);
	}

	#[test]
	fn test_multiple() {
		let starfield = Starfield {
			nb_galaxies: 4,
			nb_per_row: BTreeMap::from([(0, 1), (3, 1), (4, 1), (6, 1)]),
			nb_per_col: BTreeMap::from([(0, 4)]),
		};
		// 0 1 2 3 4 5 6
		// *     * *   *
		// 0 12345 6 789
		let expected = Starfield {
			nb_galaxies: 4,
			nb_per_row: BTreeMap::from([(0, 1), (5, 1), (6, 1), (9, 1)]),
			nb_per_col: BTreeMap::from([(0, 4)]),
		};
		assert_eq!(starfield.expand(2), expected);
	}

	#[test]
	fn test_big_factor() {
		let starfield = Starfield {
			nb_galaxies: 2,
			nb_per_row: BTreeMap::from([(0, 2)]),
			nb_per_col: BTreeMap::from([(0, 1), (2, 1)]),
		};
		let expected = Starfield {
			nb_galaxies: 2,
			nb_per_row: BTreeMap::from([(0, 2)]),
			nb_per_col: BTreeMap::from([(0, 1), (101, 1)]),
		};
		assert_eq!(starfield.expand(100), expected);
	}
}

#[cfg(test)]
mod test_get_sum_distances {
	use super::*;

	#[test]
	fn test_trivial() {
		let starfield = Starfield {
			nb_galaxies: 0,
			nb_per_row: BTreeMap::from([]),
			nb_per_col: BTreeMap::from([]),
		};
		assert_eq!(starfield.get_sum_distances(), 0);
	}

	#[test]
	fn test_all_occupied() {
		let starfield = Starfield {
			nb_galaxies: 4,
			nb_per_row: BTreeMap::from([(0, 2), (1, 2)]),
			nb_per_col: BTreeMap::from([(0, 2), (1, 2)]),
		};
		//  01
		// 0**
		// 1**
		assert_eq!(starfield.get_sum_distances(), 1 + 1 + 2 + 1 + 2 + 1);
	}

	#[test]
	fn test_row_cols_occupied() {
		let starfield = Starfield {
			nb_galaxies: 2,
			nb_per_row: BTreeMap::from([(0, 1), (1, 1)]),
			nb_per_col: BTreeMap::from([(0, 1), (1, 1)]),
		};
		//  01
		// 0*.
		// 1.*
		assert_eq!(starfield.get_sum_distances(), 2);
	}

	#[test]
	fn test_empty_row() {
		let starfield = Starfield {
			nb_galaxies: 2,
			nb_per_row: BTreeMap::from([(0, 1), (2, 1)]),
			nb_per_col: BTreeMap::from([(0, 2)]),
		};
		// 012
		// *.*
		assert_eq!(starfield.get_sum_distances(), 2);
	}

	#[test]
	fn test_empty_col() {
		let starfield = Starfield {
			nb_galaxies: 2,
			nb_per_row: BTreeMap::from([(0, 2)]),
			nb_per_col: BTreeMap::from([(0, 1), (2, 1)]),
		};
		assert_eq!(starfield.get_sum_distances(), 2);
	}

	#[test]
	fn test_empty_row_col() {
		let starfield = Starfield {
			nb_galaxies: 2,
			nb_per_row: BTreeMap::from([(0, 1), (2, 1)]),
			nb_per_col: BTreeMap::from([(0, 1), (2, 1)]),
		};
		//  012
		// 0*..
		// 1...
		// 2..*
		assert_eq!(starfield.get_sum_distances(), 4);
	}

	#[test]
	fn test_long_gap() {
		let starfield = Starfield {
			nb_galaxies: 2,
			nb_per_row: BTreeMap::from([(0, 2)]),
			nb_per_col: BTreeMap::from([(0, 1), (11, 1)]),
		};
		// 012345678901
		// *..........*
		assert_eq!(starfield.get_sum_distances(), 11);
	}

	#[test]
	fn test_zigzag() {
		let starfield = Starfield {
			nb_galaxies: 3,
			nb_per_row: BTreeMap::from([(0, 2), (1, 1)]),
			nb_per_col: BTreeMap::from([(0, 1), (1, 1), (2, 1)]),
		};
		//  012
		// 0*.*
		// 1.*.
		assert_eq!(starfield.get_sum_distances(), 6);
	}
}
