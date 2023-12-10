use std::{collections::HashSet, fmt, usize};

/// Coords on a 2D grid.
pub type Coords = (usize, usize);

/// Tile with connections.
#[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Tile {
	Wall,
	NorthSouth,
	EastWest,
	NorthWest,
	NorthEast,
	SouthWest,
	SouthEast,
	Bunny,
}

impl fmt::Display for Tile {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let pretty = match self {
			Tile::Wall => " ",
			Tile::NorthSouth => "┃",
			Tile::EastWest => "━",
			Tile::NorthWest => "┛",
			Tile::NorthEast => "┗",
			Tile::SouthWest => "┓",
			Tile::SouthEast => "┏",
			Tile::Bunny => "♞",
		};
		write!(f, "{pretty}")
	}
}

/// Step along a path.
#[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Step {
	pub row: usize,
	pub col: usize,
	pub distance: usize,
}

impl Step {
	/// Shorthand constructor.
	pub fn from(row: usize, col: usize, distance: usize) -> Self {
		Self { row, col, distance }
	}
}

/// Rectangular maze of tiles.
#[derive(Debug, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Maze {
	pub nb_rows: usize,
	pub nb_cols: usize,
	pub tiles: Vec<Vec<Tile>>,
}

impl fmt::Display for Maze {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let mut lastwrite = writeln!(f);
		for row in 0..self.nb_rows {
			for col in 0..self.nb_cols {
				let _ = write!(f, "{}", self.tiles[row][col]);
			}
			lastwrite = writeln!(f);
		}
		lastwrite
	}
}

impl Maze {
	/// Finds all tiles on the loop the bunny traces.
	#[must_use]
	pub fn get_loop(&self) -> Vec<Step> {
		let coords = self.get_loop_coords();
		let loop_length = coords.len();
		coords
			.into_iter()
			.enumerate()
			.map(|(index, (row, col))| Step::from(row, col, index.min(loop_length - index)))
			.collect()
	}

	/// Finds all tiles that connect to the given tile. Pipes must line up at both ends.
	#[must_use]
	pub fn get_neighbours(&self, (row, col): Coords) -> Vec<Coords> {
		let mut neighbours = vec![];
		let here = self.tiles[row][col];

		let connected_to_north = [
			Tile::NorthSouth,
			Tile::NorthEast,
			Tile::NorthWest,
			Tile::Bunny,
		];
		let connected_to_south = [
			Tile::NorthSouth,
			Tile::SouthEast,
			Tile::SouthWest,
			Tile::Bunny,
		];
		let connected_to_west = [
			Tile::EastWest,
			Tile::NorthWest,
			Tile::SouthWest,
			Tile::Bunny,
		];
		let connected_to_east = [
			Tile::EastWest,
			Tile::NorthEast,
			Tile::SouthEast,
			Tile::Bunny,
		];

		// Go north
		if row > 0 && connected_to_north.contains(&here) {
			self.try_connect(&mut neighbours, row - 1, col, &connected_to_south);
		}

		// Go south
		if row < self.nb_rows - 1 && connected_to_south.contains(&here) {
			self.try_connect(&mut neighbours, row + 1, col, &connected_to_north);
		}

		// Go west
		if col > 0 && connected_to_west.contains(&here) {
			self.try_connect(&mut neighbours, row, col - 1, &connected_to_east);
		}

		// Go east
		if col < self.nb_cols - 1 && connected_to_east.contains(&here) {
			self.try_connect(&mut neighbours, row, col + 1, &connected_to_west);
		}

		neighbours
	}

	/// Add a neighbouring tile to the list if it connects.
	fn try_connect(
		&self,
		neighbours: &mut Vec<Coords>,
		new_row: usize,
		new_col: usize,
		connections: &[Tile],
	) {
		let neighbour = self.tiles[new_row][new_col];
		if connections.contains(&neighbour) {
			neighbours.push((new_row, new_col));
		}
	}

	/// Finds the bunny's coordinates.
	#[must_use]
	fn get_bunny(&self) -> Coords {
		for row in 0..self.nb_rows {
			for col in 0..self.nb_cols {
				if self.tiles[row][col] == Tile::Bunny {
					return (row, col);
				}
			}
		}
		panic!("No bunny!")
	}

	#[must_use]
	fn get_loop_coords(&self) -> Vec<Coords> {
		let bunny = self.get_bunny();

		let mut steps = vec![bunny];
		let mut visited = HashSet::from([bunny]);

		let mut current = self.get_neighbours(bunny)[0];
		loop {
			steps.push(current);
			visited.insert(current);
			let next_step = self
				.get_neighbours(current)
				.into_iter()
				.find(|neighbour| !visited.contains(neighbour));
			if let Some(next_step) = next_step {
				current = next_step;
			} else {
				return steps;
			}
		}
	}
}

/// Gets the maximum distance on a path.
pub fn get_max_distance(path: &[Step]) -> usize {
	path.iter()
		.map(|step| step.distance)
		.max()
		.expect("Loop is empty")
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::parse_input::parse_full;

	const SAMPLE_INPUT_SIMPLE_BARE: &str = include_str!("../input_sample_simple_bare.txt");
	const SAMPLE_INPUT_COMPLEX_CROWDED: &str = include_str!("../input_sample_complex_crowded.txt");

	mod test_neighbours {
		use super::*;

		#[test]
		fn test_simple() {
			let maze = parse_full(SAMPLE_INPUT_SIMPLE_BARE);

			// 01234
			//      0
			//  ♞━┓ 1
			//  ┃ ┃ 2
			//  ┗━┛ 3
			//      4

			assert_eq!(maze.get_neighbours((0, 0)), vec![]);
			assert_eq!(maze.get_neighbours((1, 1)), vec![(2, 1), (1, 2)]);
			assert_eq!(maze.get_neighbours((1, 2)), vec![(1, 1), (1, 3)]);
			assert_eq!(maze.get_neighbours((1, 3)), vec![(2, 3), (1, 2)]);
			assert_eq!(maze.get_neighbours((2, 3)), vec![(1, 3), (3, 3)]);
			assert_eq!(maze.get_neighbours((3, 3)), vec![(2, 3), (3, 2)]);
			assert_eq!(maze.get_neighbours((3, 2)), vec![(3, 1), (3, 3)]);
			assert_eq!(maze.get_neighbours((3, 1)), vec![(2, 1), (3, 2)]);
			assert_eq!(maze.get_neighbours((2, 1)), vec![(1, 1), (3, 1)]);
			assert_eq!(maze.get_neighbours((2, 2)), vec![]);
		}

		#[test]
		fn test_complex() {
			let maze = parse_full(SAMPLE_INPUT_COMPLEX_CROWDED);

			// 01234
			// ┓━┏┓━ 0
			//  ┏┛┃┓ 1
			// ♞┛┗┗┓ 2
			// ┃┏━━┛ 3
			// ┗┛ ┗┛ 4

			assert_eq!(maze.get_neighbours((0, 0)), vec![]);
			assert_eq!(maze.get_neighbours((2, 0)), vec![(3, 0), (2, 1)]);
		}
	}

	mod test_loop {
		use crate::maze::Step;

		use super::*;

		#[test]
		fn test_simple() {
			let maze = parse_full(SAMPLE_INPUT_SIMPLE_BARE);

			// 01234
			//      0
			//  ♞━┓ 1
			//  ┃ ┃ 2
			//  ┗━┛ 3
			//      4

			let expected = vec![
				Step::from(1, 1, 0),
				Step::from(2, 1, 1),
				Step::from(3, 1, 2),
				Step::from(3, 2, 3),
				Step::from(3, 3, 4),
				Step::from(2, 3, 3),
				Step::from(1, 3, 2),
				Step::from(1, 2, 1),
			];

			assert_eq!(maze.get_loop(), expected);
		}

		#[test]
		fn test_complex() {
			let maze = parse_full(SAMPLE_INPUT_COMPLEX_CROWDED);

			// 01234
			// ┓━┏┓━ 0
			//  ┏┛┃┓ 1
			// ♞┛┗┗┓ 2
			// ┃┏━━┛ 3
			// ┗┛ ┗┛ 4

			let expected = vec![
				Step::from(2, 0, 0),
				Step::from(3, 0, 1),
				Step::from(4, 0, 2),
				Step::from(4, 1, 3),
				Step::from(3, 1, 4),
				Step::from(3, 2, 5),
				Step::from(3, 3, 6),
				Step::from(3, 4, 7),
				Step::from(2, 4, 8),
				Step::from(2, 3, 7),
				Step::from(1, 3, 6),
				Step::from(0, 3, 5),
				Step::from(0, 2, 4),
				Step::from(1, 2, 3),
				Step::from(1, 1, 2),
				Step::from(2, 1, 1),
			];

			assert_eq!(maze.get_loop(), expected)
		}
	}

	#[test]
	fn test_max_distance() {
		let path = vec![
			Step::from(42, 23, 100),
			Step::from(43, 23, 101),
			Step::from(43, 21, 100),
		];
		assert_eq!(get_max_distance(&path), 101);
	}
}
