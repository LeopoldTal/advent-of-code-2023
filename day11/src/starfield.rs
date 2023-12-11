use std::collections::HashSet;

pub type Coords = (usize, usize);

/// List of galaxy positions.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Starfield {
	pub galaxies: HashSet<Coords>,
	occupied_rows: Vec<usize>,
	occupied_cols: Vec<usize>,
}

impl Starfield {
	/// Constructs from a list of coords.
	#[must_use]
	pub fn from(galaxies: HashSet<Coords>) -> Self {
		let occupied_rows: Vec<usize> = to_unique(galaxies.iter().map(|&(row, _)| row));
		let occupied_cols: Vec<usize> = to_unique(galaxies.iter().map(|&(_, col)| col));

		Self {
			galaxies,
			occupied_rows,
			occupied_cols,
		}
	}

	/// Adjusts distances so unoccupied rows and columns are twice as wide.
	#[must_use]
	pub fn expand(&self, expand_factor: usize) -> HashSet<Coords> {
		self.galaxies
			.iter()
			.map(|&(row, col)| {
				(
					expand_distance(row, expand_factor, &self.occupied_rows),
					expand_distance(col, expand_factor, &self.occupied_cols),
				)
			})
			.collect()
	}
}

#[cfg(test)]
mod test_from {
	use super::*;

	#[test]
	fn test_dense() {
		let galaxies = HashSet::from([(0, 0), (0, 1), (1, 0), (1, 1)]);
		let starfield = Starfield::from(galaxies);
		let expected_rows = vec![0, 1];
		let expected_cols = vec![0, 1];
		assert_eq!(starfield.occupied_rows, expected_rows);
		assert_eq!(starfield.occupied_cols, expected_cols);
	}

	#[test]
	fn test_gaps() {
		let galaxies = HashSet::from([(0, 0), (23, 42), (24, 1)]);
		let starfield = Starfield::from(galaxies);
		let expected_rows = vec![0, 23, 24];
		let expected_cols = vec![0, 1, 42];
		assert_eq!(starfield.occupied_rows, expected_rows);
		assert_eq!(starfield.occupied_cols, expected_cols);
	}
}

#[cfg(test)]
mod test_expand {
	use super::*;

	#[test]
	fn test_trivial() {
		let galaxies = HashSet::from([]);
		let starfield = Starfield::from(galaxies);
		let expected = HashSet::from([]);
		assert_eq!(starfield.expand(2), expected);
	}

	#[test]
	fn test_all_occupied() {
		let galaxies = HashSet::from([(0, 0), (0, 1), (1, 0), (1, 1)]);
		let starfield = Starfield::from(galaxies);
		let expected = HashSet::from([(0, 0), (0, 1), (1, 0), (1, 1)]);
		assert_eq!(starfield.expand(2), expected);
	}

	#[test]
	fn test_row_cols_occupied() {
		let galaxies = HashSet::from([(0, 0), (1, 1)]);
		let starfield = Starfield::from(galaxies);
		let expected = HashSet::from([(0, 0), (1, 1)]);
		assert_eq!(starfield.expand(2), expected);
	}

	#[test]
	fn test_empty_row() {
		let galaxies = HashSet::from([(0, 0), (2, 0)]);
		let starfield = Starfield::from(galaxies);
		let expected = HashSet::from([(0, 0), (3, 0)]);
		assert_eq!(starfield.expand(2), expected);
	}

	#[test]
	fn test_empty_col() {
		let galaxies = HashSet::from([(0, 0), (0, 2)]);
		let starfield = Starfield::from(galaxies);
		let expected = HashSet::from([(0, 0), (0, 3)]);
		assert_eq!(starfield.expand(2), expected);
	}

	#[test]
	fn test_empty_row_col() {
		let galaxies = HashSet::from([(0, 0), (2, 2)]);
		let starfield = Starfield::from(galaxies);
		let expected = HashSet::from([(0, 0), (3, 3)]);
		assert_eq!(starfield.expand(2), expected);
	}

	#[test]
	fn test_long_gap() {
		let galaxies = HashSet::from([(0, 0), (11, 0)]);
		let starfield = Starfield::from(galaxies);
		let expected = HashSet::from([(0, 0), (21, 0)]);
		assert_eq!(starfield.expand(2), expected);
	}

	#[test]
	fn test_multiple() {
		let galaxies = HashSet::from([(0, 0), (3, 0), (4, 0), (6, 0)]);
		let starfield = Starfield::from(galaxies);
		// 0 1 2 3 4 5 6
		// *     * *   *
		// 0 12345 6 789
		let expected = HashSet::from([(0, 0), (5, 0), (6, 0), (9, 0)]);
		assert_eq!(starfield.expand(2), expected);
	}

	#[test]
	fn test_big_factor() {
		let galaxies = HashSet::from([(0, 0), (2, 0)]);
		let starfield = Starfield::from(galaxies);
		let expected = HashSet::from([(0, 0), (101, 0)]);
		assert_eq!(starfield.expand(100), expected);
	}
}

/// Collects values into a sorted, deduped vec.
fn to_unique(indices: impl Iterator<Item = usize>) -> Vec<usize> {
	let unique: HashSet<usize> = indices.collect();
	let mut collected: Vec<usize> = unique.into_iter().collect();
	collected.sort_unstable();
	collected
}

/// Counts occupied rows and columns before `end` exclusive.
/// Assumes `occupied` is sorted.
fn count_occupied_before(end: usize, occupied: &[usize]) -> usize {
	occupied.partition_point(|&x| x <= end)
}

/// Expands space between `0` and `index`.
fn expand_distance(index: usize, expand_factor: usize, occupied: &[usize]) -> usize {
	let count_unoccupied = index + 1 - count_occupied_before(index, occupied);
	index + (expand_factor - 1) * count_unoccupied
}

#[cfg(test)]
mod test_count_occupied_before {
	use super::*;

	#[test]
	fn test_all_empty() {
		let count = count_occupied_before(4, &[]);
		assert_eq!(count, 0);
	}

	#[test]
	fn test_dense() {
		let count = count_occupied_before(1, &[0, 1]);
		assert_eq!(count, 2);
	}

	#[test]
	fn test_endpoints_only() {
		let count = count_occupied_before(4, &[0, 4]);
		assert_eq!(count, 2);
	}

	#[test]
	fn test_between() {
		let occupied = &[0, 1, 2, 3, 4, 5];
		let count = count_occupied_before(4, occupied);
		assert_eq!(count, 5);
	}

	#[test]
	fn test_before_start() {
		let occupied = &[4, 5, 7];
		let count = count_occupied_before(2, occupied);
		assert_eq!(count, 0);
	}

	#[test]
	fn test_past_end() {
		let occupied = &[0, 1, 4, 5, 7];
		let count = count_occupied_before(10, occupied);
		assert_eq!(count, 5);
	}
}
