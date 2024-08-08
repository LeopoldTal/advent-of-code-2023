/// A 2D bool array.
#[derive(Debug, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Grid {
	pub nb_rows: usize,
	pub nb_cols: usize,
	pub tiles: Vec<Vec<bool>>,
}

/// Tests whether the vector is reflected just before the given index, ignoring extra items on either side.
#[must_use]
fn is_reflected_at(row: &[bool], mirror_index: usize) -> bool {
	let max_distance = mirror_index.min(row.len() - mirror_index);
	for distance in 0..max_distance {
		let before = row[mirror_index - distance - 1];
		let after = row[mirror_index + distance];
		if before != after {
			return false;
		}
	}
	true
}

#[cfg(test)]
mod test_is_reflected_at {
	use super::*;

	#[test]
	fn test_left_edge_match() {
		let input = vec![false, false, true, true, true];
		assert!(is_reflected_at(&input, 1));
	}

	#[test]
	fn test_left_edge_mismatch() {
		let input = vec![false, true, true, true, true];
		assert!(!is_reflected_at(&input, 1));
	}

	#[test]
	fn test_right_edge_match() {
		let input = vec![false, false, true, true, true];
		assert!(is_reflected_at(&input, 4));
	}

	#[test]
	fn test_lean_left_match() {
		let input = vec![
			false, false, true, true, false, false, true, true, false, false, true, false,
		];
		assert!(is_reflected_at(&input, 5));
		use super::*;
	}

	#[test]
	fn test_lean_left_mismatch() {
		let input = vec![
			false, false, true, true, false, false, true, true, true, false, true, false,
		];
		assert!(!is_reflected_at(&input, 5));
		use super::*;
	}

	#[test]
	fn test_lean_right_match() {
		let input = vec![false, false, false, false, false, true, true, false];
		assert!(is_reflected_at(&input, 6));
		use super::*;
	}

	#[test]
	fn test_lean_right_mismatch() {
		let input = vec![false, false, false, false, true, true, true, false];
		assert!(!is_reflected_at(&input, 6));
		use super::*;
	}

	#[test]
	fn test_middle_match() {
		let input = vec![false, true, false, false, true, false];
		assert!(is_reflected_at(&input, 3));
		use super::*;
	}

	#[test]
	fn test_middle_mismatch() {
		let input = vec![true, true, false, false, true, false];
		assert!(!is_reflected_at(&input, 3));
		use super::*;
	}
}

/// Gets all indices forming a vertical reflection in the grid.
#[must_use]
pub fn get_reflects_vert(grid: &Grid) -> Vec<usize> {
	let mut reflect_indices: Vec<usize> = vec![];
	for mirror_index in 1..grid.nb_cols {
		if grid
			.tiles
			.iter()
			.all(|row| is_reflected_at(row, mirror_index))
		{
			reflect_indices.push(mirror_index);
		}
	}
	reflect_indices
}

#[cfg(test)]
mod test_reflect_vert {
	use super::*;

	#[test]
	fn test_reflect_vert_middle() {
		let input = Grid {
			nb_rows: 5,
			nb_cols: 4,
			tiles: vec![
				vec![false, true, true, false],
				vec![true, false, false, true],
				vec![false; 4],
				vec![true; 4],
				vec![true; 4],
			],
		};
		assert_eq!(get_reflects_vert(&input), vec![2]);
	}

	#[test]
	fn test_reflect_vert_left() {
		let input = Grid {
			nb_rows: 2,
			nb_cols: 3,
			tiles: vec![vec![true, true, false], vec![true; 3]],
		};
		assert_eq!(get_reflects_vert(&input), vec![1]);
	}

	#[test]
	fn test_reflect_vert_right() {
		let input = Grid {
			nb_rows: 3,
			nb_cols: 5,
			tiles: vec![
				vec![false, false, true, true, false],
				vec![false; 5],
				vec![false, true, false, false, true],
			],
		};
		assert_eq!(get_reflects_vert(&input), vec![3]);
	}

	#[test]
	fn test_reflect_vert_none() {
		let input = Grid {
			nb_rows: 1,
			nb_cols: 2,
			tiles: vec![vec![false, true]],
		};
		assert_eq!(get_reflects_vert(&input), vec![]);
	}
}

/// Gets all indices forming a horizontal reflection in the grid.
#[must_use]
pub fn get_reflects_horiz(grid: &Grid) -> Vec<usize> {
	let cols: Vec<Vec<bool>> = (0..grid.nb_cols)
		.map(|col_index| grid.tiles.iter().map(|row| row[col_index]).collect())
		.collect();
	let mut reflect_indices: Vec<usize> = vec![];
	for mirror_index in 1..grid.nb_rows {
		if cols.iter().all(|col| is_reflected_at(col, mirror_index)) {
			reflect_indices.push(mirror_index);
		}
	}
	reflect_indices
}

#[cfg(test)]
mod test_reflect_horiz {
	use super::*;

	#[test]
	fn test_reflect_horiz_middle() {
		let input = Grid {
			nb_rows: 4,
			nb_cols: 5,
			tiles: vec![vec![false; 5], vec![true; 5], vec![true; 5], vec![false; 5]],
		};
		assert_eq!(get_reflects_horiz(&input), vec![2]);
	}

	#[test]
	fn test_reflect_horiz_top() {
		let input = Grid {
			nb_rows: 3,
			nb_cols: 2,
			tiles: vec![vec![true, false], vec![true, false], vec![false, false]],
		};
		assert_eq!(get_reflects_horiz(&input), vec![1]);
	}

	#[test]
	fn test_reflect_horiz_bottom() {
		let input = Grid {
			nb_rows: 5,
			nb_cols: 2,
			tiles: vec![
				vec![true, true],
				vec![false, true],
				vec![false, false],
				vec![true, false],
				vec![true, false],
			],
		};
		assert_eq!(get_reflects_horiz(&input), vec![4]);
	}

	#[test]
	fn test_reflect_horiz_none() {
		let input = Grid {
			nb_rows: 4,
			nb_cols: 2,
			tiles: vec![
				vec![true, true],
				vec![false, true],
				vec![false, false],
				vec![true, false],
			],
		};
		assert_eq!(get_reflects_horiz(&input), vec![]);
	}
}

/// Gets all indices forming a reflection in the grid (×100 if horizontal).
#[must_use]
pub fn get_reflects(grid: &Grid) -> Vec<usize> {
	let indices_vert = get_reflects_vert(grid);
	let indices_horiz = get_reflects_horiz(grid);

	indices_horiz
		.into_iter()
		.map(|index| 100 * index)
		.chain(indices_vert)
		.collect()
}

/// Gets the unique reflection index of a grid (×100 if horizontal).
#[must_use]
pub fn get_reflect(grid: &Grid) -> usize {
	let reflects = get_reflects(grid);
	assert_eq!(reflects.len(), 1, "Grid should have unique reflection");
	reflects[0]
}

/// Clones the grid and flips one tile in the clone.
#[must_use]
fn flip_at(grid: &Grid, row_index: usize, col_index: usize) -> Grid {
	let mut new_grid = grid.clone();
	new_grid.tiles[row_index][col_index] = !new_grid.tiles[row_index][col_index];
	new_grid
}

/// Iterates all variations of the grid with one tile flipped.
fn flip_one(grid: &Grid) -> impl Iterator<Item = Grid> + '_ {
	(0..grid.nb_rows).flat_map(move |row_index| {
		(0..grid.nb_cols).map(move |col_index| flip_at(grid, row_index, col_index))
	})
}

#[cfg(test)]
mod test_flip_one {
	use super::*;

	#[test]
	fn test_trivial() {
		let input = Grid {
			nb_rows: 0,
			nb_cols: 0,
			tiles: vec![],
		};
		let flipped: Vec<Grid> = flip_one(&input).collect();
		assert!(flipped.is_empty());
	}

	#[test]
	fn test_flips() {
		let input = Grid {
			nb_rows: 2,
			nb_cols: 2,
			tiles: vec![vec![true, false], vec![false, true]],
		};
		let flipped: Vec<Grid> = flip_one(&input).collect();
		let expected = vec![
			Grid {
				nb_rows: 2,
				nb_cols: 2,
				tiles: vec![vec![false, false], vec![false, true]],
			},
			Grid {
				nb_rows: 2,
				nb_cols: 2,
				tiles: vec![vec![true, true], vec![false, true]],
			},
			Grid {
				nb_rows: 2,
				nb_cols: 2,
				tiles: vec![vec![true, false], vec![true, true]],
			},
			Grid {
				nb_rows: 2,
				nb_cols: 2,
				tiles: vec![vec![true, false], vec![false, false]],
			},
		];
		assert_eq!(flipped, expected);
	}
}

/// Gets a new reflection index of a grid (×100 if horizontal, None if none exists), excluding the old index.
#[must_use]
pub fn get_new_reflect(new_grid: &Grid, old_reflect: usize) -> Option<usize> {
	let mut reflects = get_reflects(new_grid);
	// The flip must make a difference
	reflects.retain(|&new_reflect| new_reflect != old_reflect);
	assert!(reflects.len() < 2, "Too many new reflections");
	if reflects.is_empty() {
		None
	} else {
		Some(reflects[0])
	}
}

/// Tries all flips on a grid, finds the one with a unique reflection, and returns its index.
#[must_use]
pub fn get_reflect_with_flip(grid: &Grid) -> usize {
	let old_reflect = get_reflect(grid);
	for flipped in flip_one(grid) {
		let reflect = get_new_reflect(&flipped, old_reflect);
		if let Some(index) = reflect {
			return index;
		}
	}
	panic!("Grid has no acceptable flip!");
}
