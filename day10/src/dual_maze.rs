use std::collections::HashSet;

use crate::maze::Coords;

/// Dual of the other maze, where the main loop acts as a wall instead.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DualMaze {
	pub nb_rows: usize,
	pub nb_cols: usize,
	pub walls: HashSet<(Coords, Coords)>,
}

impl DualMaze {
	/// Builds a maze using the original maze and its loop as a wall.
	/// Each wall is to the south and east of the tile in the dual maze.
	pub fn from(nb_maze_rows: usize, nb_maze_cols: usize, path: &[Coords]) -> Self {
		let mut walls: HashSet<(Coords, Coords)> = HashSet::new();
		for index in 0..path.len() {
			let wall_from = path[index];
			let wall_to = path[(index + 1) % path.len()];
			walls.insert((wall_from.min(wall_to), wall_from.max(wall_to)));
		}
		Self {
			nb_rows: nb_maze_rows + 1,
			nb_cols: nb_maze_cols + 1,
			walls,
		}
	}

	/// Finds all tiles that connect to the given tile. Moves along walls, but not through them.
	#[must_use]
	pub fn get_neighbours(&self, (row, col): Coords) -> Vec<Coords> {
		let mut neighbours = vec![];

		// Walls block path perpendicular to them. For example:
		// - a wall between (0,1) and (0,2) prevents movement between (0,1) and (1,1)
		// - a wall between (0,1) and (1,1) prevents movement between (1,1) and (1,2)

		// Go north
		if row > 0 {
			let next = (row - 1, col);
			// At the western edge: no wall here.
			if col == 0 {
				neighbours.push(next);
			} else {
				let wall = ((row - 1, col - 1), (row - 1, col));
				if !self.walls.contains(&wall) {
					neighbours.push(next);
				}
			}
		}

		// Go south
		if row < self.nb_rows - 1 {
			let next: (usize, usize) = (row + 1, col);
			// At the western edge: no wall here.
			if col == 0 {
				neighbours.push(next);
			} else {
				let wall = ((row, col - 1), (row, col));
				if !self.walls.contains(&wall) {
					neighbours.push(next);
				}
			}
		}

		// Go west
		if col > 0 {
			let next: (usize, usize) = (row, col - 1);
			// At the northern edge: no wall here.
			if row == 0 {
				neighbours.push(next);
			} else {
				let wall = ((row - 1, col - 1), (row, col - 1));
				if !self.walls.contains(&wall) {
					neighbours.push(next);
				}
			}
		}

		// Go east
		if col < self.nb_cols - 1 {
			let next: (usize, usize) = (row, col + 1);
			// At the northern edge: no wall here.
			if row == 0 {
				neighbours.push(next);
			} else {
				let wall = ((row - 1, col), (row, col));
				if !self.walls.contains(&wall) {
					neighbours.push(next);
				}
			}
		}

		neighbours
	}

	#[must_use]
	pub fn get_enclosed_tiles(&self) -> HashSet<Coords> {
		// Flood fill the outside of the loop.
		let mut outside: HashSet<Coords> = HashSet::new();
		let mut border: Vec<Coords> = vec![(0, 0)];
		while let Some(node) = border.pop() {
			// Don't re-visit same node.
			if outside.contains(&node) {
				continue;
			}
			outside.insert(node);
			border.extend(self.get_neighbours(node));
		}

		// Find the tiles inside the loop.
		let mut inside: HashSet<Coords> = HashSet::new();
		for row in 0..self.nb_rows - 1 {
			for col in 0..self.nb_cols - 1 {
				let here = (row, col);
				// If it's not outside…
				if !outside.contains(&here) {
					// …and not occupied by a wall…
					let walls_around = [
						((row - 1, col), here), // north
						(here, (row + 1, col)), // south
						((row, col - 1), here), // west
						(here, (row, col + 1)), // east
					];
					if walls_around.iter().all(|wall| !self.walls.contains(wall)) {
						// …then it must be inside.
						inside.insert(here);
					}
				}
			}
		}
		inside
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::{parse_input::parse_full, samples::SAMPLE_INPUT_SIMPLE_CROWDED};

	#[test]
	fn test_from() {
		let maze = parse_full(SAMPLE_INPUT_SIMPLE_CROWDED);
		let path = maze.get_loop_coords();
		let dual = DualMaze::from(maze.nb_rows, maze.nb_cols, &path);

		// 01234
		//      0
		//  ┏━┓ 1
		//  ┃ ┃ 2
		//  ┗━┛ 3
		//      4

		assert_eq!(dual.nb_rows, 6);
		assert_eq!(dual.nb_cols, 6);

		let expected = HashSet::from([
			((1, 1), (1, 2)),
			((1, 2), (1, 3)),
			((1, 3), (2, 3)),
			((2, 3), (3, 3)),
			((3, 2), (3, 3)),
			((3, 1), (3, 2)),
			((2, 1), (3, 1)),
			((1, 1), (2, 1)),
		]);
		assert_eq!(dual.walls, expected);
	}

	mod test_neighbours {
		use crate::samples::{SAMPLE_INPUT_COMPLEX_CROWDED, SAMPLE_INPUT_ENCLOSED_NARROW};

		use super::*;

		#[test]
		fn test_simple() {
			let maze = parse_full(SAMPLE_INPUT_SIMPLE_CROWDED);
			let path = maze.get_loop_coords();
			let dual = DualMaze::from(maze.nb_rows, maze.nb_cols, &path);

			// 01234
			//      0
			//  ┏━┓ 1
			//  ┃ ┃ 2
			//  ┗━┛ 3
			//      4

			assert_eq!(dual.get_neighbours((0, 0)), vec![(1, 0), (0, 1)]);
			assert_eq!(dual.get_neighbours((0, 1)), vec![(1, 1), (0, 0), (0, 2)]);
			assert_eq!(dual.get_neighbours((1, 0)), vec![(0, 0), (2, 0), (1, 1)]);
			assert_eq!(
				dual.get_neighbours((1, 1)),
				vec![(0, 1), (2, 1), (1, 0), (1, 2)]
			);
			assert_eq!(dual.get_neighbours((1, 2)), vec![(0, 2), (1, 1), (1, 3)]);
			assert_eq!(dual.get_neighbours((2, 1)), vec![(1, 1), (3, 1), (2, 0)]);
			assert_eq!(dual.get_neighbours((2, 2)), vec![(3, 2), (2, 3)]);
			assert_eq!(dual.get_neighbours((2, 3)), vec![(3, 3), (2, 2)]);
			assert_eq!(dual.get_neighbours((3, 2)), vec![(2, 2), (3, 3)]);
			assert_eq!(dual.get_neighbours((3, 3)), vec![(2, 3), (3, 2)]);
			assert_eq!(dual.get_neighbours((3, 4)), vec![(2, 4), (4, 4), (3, 5)]);
			assert_eq!(dual.get_neighbours((4, 3)), vec![(5, 3), (4, 2), (4, 4)]);
			assert_eq!(
				dual.get_neighbours((4, 4)),
				vec![(3, 4), (5, 4), (4, 3), (4, 5)]
			);
			assert_eq!(dual.get_neighbours((5, 5)), vec![(4, 5), (5, 4)]);
		}

		#[test]
		fn test_complex() {
			let maze = parse_full(SAMPLE_INPUT_COMPLEX_CROWDED);
			let path = maze.get_loop_coords();
			let dual = DualMaze::from(maze.nb_rows, maze.nb_cols, &path);

			// 01234
			//   ┏┓  0
			//  ┏┛┃  1
			// ┏┛ ┗┓ 2
			// ┃┏━━┛ 3
			// ┗┛    4

			assert_eq!(dual.get_neighbours((0, 0)), vec![(1, 0), (0, 1)]);
			assert_eq!(dual.get_neighbours((1, 2)), vec![(0, 2), (1, 1)]);
			assert_eq!(dual.get_neighbours((1, 3)), vec![(2, 3)]);
			assert_eq!(dual.get_neighbours((3, 1)), vec![(4, 1), (3, 2)]);
			assert_eq!(dual.get_neighbours((3, 4)), vec![(3, 3)]);
			assert_eq!(dual.get_neighbours((4, 1)), vec![(3, 1)]);
		}

		#[test]
		fn test_enclosed_narrow() {
			let maze = parse_full(SAMPLE_INPUT_ENCLOSED_NARROW);
			let path = maze.get_loop_coords();
			let dual = DualMaze::from(maze.nb_rows, maze.nb_cols, &path);

			//  0123456789
			// 0          0
			// 1 ┏━━━━━━┓ 1
			// 2 ┃┏━━━━┓┃ 2
			// 3 ┃┃    ┃┃ 3
			// 4 ┃┃    ┃┃ 4
			// 5 ┃┗━┓┏━┛┃ 5
			// 6 ┃  ┃┃  ┃ 6
			// 7 ┗━━┛┗━━┛ 7
			// 8          8
			//  0123456789

			assert_eq!(dual.get_neighbours((0, 0)), vec![(1, 0), (0, 1)]);
			assert_eq!(dual.get_neighbours((2, 2)), vec![(3, 2), (2, 3)]);
			assert_eq!(dual.get_neighbours((2, 3)), vec![(2, 2), (2, 4)]);
			assert_eq!(dual.get_neighbours((4, 2)), vec![(3, 2), (5, 2)]);
			assert_eq!(dual.get_neighbours((5, 3)), vec![(4, 3), (5, 4)]);
			assert_eq!(dual.get_neighbours((5, 4)), vec![(4, 4), (5, 3), (5, 5)]);
			assert_eq!(
				dual.get_neighbours((5, 5)),
				vec![(4, 5), (6, 5), (5, 4), (5, 6)]
			);
			assert_eq!(dual.get_neighbours((5, 6)), vec![(4, 6), (5, 5), (5, 7)]);
			assert_eq!(dual.get_neighbours((5, 7)), vec![(4, 7), (5, 6)]);
			assert_eq!(dual.get_neighbours((6, 4)), vec![(7, 4), (6, 3)]);
			assert_eq!(dual.get_neighbours((6, 5)), vec![(5, 5), (7, 5)]);
			assert_eq!(dual.get_neighbours((6, 6)), vec![(7, 6), (6, 7)]);
			assert_eq!(dual.get_neighbours((7, 4)), vec![(6, 4), (7, 3)]);
			assert_eq!(dual.get_neighbours((7, 5)), vec![(6, 5), (8, 5)]);
			assert_eq!(dual.get_neighbours((7, 6)), vec![(6, 6), (7, 7)]);
		}
	}

	mod test_enclosed {
		use crate::samples::{
			SAMPLE_INPUT_COMPLEX_CROWDED, SAMPLE_INPUT_ENCLOSED_NARROW, SAMPLE_INPUT_ENCLOSED_OPEN,
		};

		use super::*;

		#[test]
		fn test_simple() {
			let maze = parse_full(SAMPLE_INPUT_SIMPLE_CROWDED);
			let path = maze.get_loop_coords();
			let dual = DualMaze::from(maze.nb_rows, maze.nb_cols, &path);

			// 01234
			//      0
			//  ┏━┓ 1
			//  ┃ ┃ 2
			//  ┗━┛ 3
			//      4

			assert_eq!(dual.get_enclosed_tiles(), HashSet::from([(2, 2)]));
		}

		#[test]
		fn test_complex() {
			let maze = parse_full(SAMPLE_INPUT_COMPLEX_CROWDED);
			let path = maze.get_loop_coords();
			let dual = DualMaze::from(maze.nb_rows, maze.nb_cols, &path);

			// 01234
			//   ┏┓  0
			//  ┏┛┃  1
			// ┏┛ ┗┓ 2
			// ┃┏━━┛ 3
			// ┗┛    4

			assert_eq!(dual.get_enclosed_tiles(), HashSet::from([(2, 2)]));
		}

		#[test]
		fn test_enclosed_open() {
			let maze = parse_full(SAMPLE_INPUT_ENCLOSED_OPEN);
			let path = maze.get_loop_coords();
			let dual = DualMaze::from(maze.nb_rows, maze.nb_cols, &path);

			//  01234567890
			// 0           0
			// 1 ┏━━━━━━━┓ 1
			// 2 ┃┏━━━━━┓┃ 2
			// 3 ┃┃     ┃┃ 3
			// 4 ┃┃     ┃┃ 4
			// 5 ┃┗━┓ ┏━┛┃ 5
			// 6 ┃  ┃ ┃  ┃ 6
			// 7 ┗━━┛ ┗━━┛ 7
			// 8           8
			//  01234567890

			let expected = HashSet::from([(6, 2), (6, 3), (6, 7), (6, 8)]);
			assert_eq!(dual.get_enclosed_tiles(), expected);
		}

		#[test]
		fn test_enclosed_narrow() {
			let maze = parse_full(SAMPLE_INPUT_ENCLOSED_NARROW);
			let path = maze.get_loop_coords();
			let dual = DualMaze::from(maze.nb_rows, maze.nb_cols, &path);

			//  0123456789
			// 0          0
			// 1 ┏━━━━━━┓ 1
			// 2 ┃┏━━━━┓┃ 2
			// 3 ┃┃    ┃┃ 3
			// 4 ┃┃    ┃┃ 4
			// 5 ┃┗━┓┏━┛┃ 5
			// 6 ┃  ┃┃  ┃ 6
			// 7 ┗━━┛┗━━┛ 7
			// 8          8
			//  0123456789

			let expected = HashSet::from([(6, 2), (6, 3), (6, 6), (6, 7)]);
			assert_eq!(dual.get_enclosed_tiles(), expected);
		}
	}
}
