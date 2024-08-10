use crate::constraints::Constraints;

use self::Direction::{Down, Left, Right, Up};
use std::{collections::HashSet, fmt};

/// Facing direction.
#[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Direction {
	Up,
	Down,
	Left,
	Right,
}

/// A state the mover can be in.
#[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct State {
	pub row: usize,
	pub col: usize,
	pub facing: Direction,
	pub must_turn_in: usize,
	pub can_turn_in: usize,
}

/// A 2D array of tiles.
#[derive(Debug, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Board {
	pub nb_rows: usize,
	pub nb_cols: usize,
	pub tiles: Vec<Vec<u32>>,
	pub constraints: Constraints,
}

impl Board {
	/// Builds a board from an 2D array of tiles.
	#[must_use]
	pub fn from(tiles: Vec<Vec<u32>>, constraints: Constraints) -> Self {
		Self {
			nb_rows: tiles.len(),
			nb_cols: tiles.first().expect("Empty grid").len(),
			tiles,
			constraints,
		}
	}

	/// Gets the states reachable after one step, and their costs.
	#[must_use]
	pub fn get_neighbours(&self, start: &State) -> HashSet<(State, u32)> {
		let mut neighbours = HashSet::new();

		if start.can_turn_in == 0 {
			let turn_directions = match start.facing {
				Up | Down => [Left, Right],
				Left | Right => [Up, Down],
			};
			for new_facing in turn_directions {
				self.add_neighbour_tile(
					&mut neighbours,
					start.row,
					start.col,
					new_facing,
					self.constraints.max_straight_line - 1,
					self.constraints.min_straight_line.saturating_sub(1),
				);
			}
		}

		if start.must_turn_in > 0 {
			self.add_neighbour_tile(
				&mut neighbours,
				start.row,
				start.col,
				start.facing,
				start.must_turn_in - 1,
				start.can_turn_in.saturating_sub(1),
			);
		}

		neighbours
	}

	/// Adds a neighbouring to a set if it exists.
	fn add_neighbour_tile(
		&self,
		neighbours: &mut HashSet<(State, u32)>,
		from_row: usize,
		from_col: usize,
		facing: Direction,
		must_turn_in: usize,
		can_turn_in: usize,
	) {
		if let Some((row, col)) = self.get_neighbour_tile(from_row, from_col, facing) {
			let new_state = State {
				row,
				col,
				facing,
				must_turn_in,
				can_turn_in,
			};
			let cost = self.tiles[row][col];
			neighbours.insert((new_state, cost));
		}
	}

	/// Gets the neighbouring tile in a direction if it exists.
	#[must_use]
	fn get_neighbour_tile(
		&self,
		row: usize,
		col: usize,
		direction: Direction,
	) -> Option<(usize, usize)> {
		match direction {
			Up => {
				if row > 0 {
					Some((row - 1, col))
				} else {
					None
				}
			}
			Down => {
				if row < self.nb_rows - 1 {
					Some((row + 1, col))
				} else {
					None
				}
			}
			Left => {
				if col > 0 {
					Some((row, col - 1))
				} else {
					None
				}
			}
			Right => {
				if col < self.nb_cols - 1 {
					Some((row, col + 1))
				} else {
					None
				}
			}
		}
	}
}

impl fmt::Display for Board {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let mut res = writeln!(f);
		for row in &self.tiles {
			for tile in row {
				_ = write!(f, "{tile}");
			}
			res = writeln!(f);
		}
		res
	}
}

#[cfg(test)]
mod test_get_neighbours {
	use super::*;

	const MAX_STRAIGHT_LINE: usize = 7;
	const MIN_STRAIGHT_LINE: usize = 3;

	fn get_test_board() -> Board {
		Board::from(
			vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
			Constraints {
				min_straight_line: MIN_STRAIGHT_LINE,
				max_straight_line: MAX_STRAIGHT_LINE,
			},
		)
	}

	#[test]
	fn test_up() {
		let board = get_test_board();
		let start_state = State {
			row: 1,
			col: 1,
			facing: Up,
			must_turn_in: 3,
			can_turn_in: 0,
		};
		let next_states = board.get_neighbours(&start_state);
		let expected = HashSet::from([
			(
				State {
					row: 0,
					col: 1,
					facing: Up,
					must_turn_in: 2,
					can_turn_in: 0,
				},
				2,
			),
			(
				State {
					row: 1,
					col: 0,
					facing: Left,
					must_turn_in: MAX_STRAIGHT_LINE - 1,
					can_turn_in: MIN_STRAIGHT_LINE - 1,
				},
				4,
			),
			(
				State {
					row: 1,
					col: 2,
					facing: Right,
					must_turn_in: MAX_STRAIGHT_LINE - 1,
					can_turn_in: MIN_STRAIGHT_LINE - 1,
				},
				6,
			),
		]);
		assert_eq!(next_states, expected);
	}

	#[test]
	fn test_up_must_turn() {
		let board = get_test_board();
		let start_state = State {
			row: 1,
			col: 1,
			facing: Up,
			must_turn_in: 0,
			can_turn_in: 0,
		};
		let next_states = board.get_neighbours(&start_state);
		let expected = HashSet::from([
			(
				State {
					row: 1,
					col: 0,
					facing: Left,
					must_turn_in: MAX_STRAIGHT_LINE - 1,
					can_turn_in: MIN_STRAIGHT_LINE - 1,
				},
				4,
			),
			(
				State {
					row: 1,
					col: 2,
					facing: Right,
					must_turn_in: MAX_STRAIGHT_LINE - 1,
					can_turn_in: MIN_STRAIGHT_LINE - 1,
				},
				6,
			),
		]);
		assert_eq!(next_states, expected);
	}

	#[test]
	fn test_up_cant_turn() {
		let board = get_test_board();
		let start_state = State {
			row: 1,
			col: 1,
			facing: Up,
			must_turn_in: 5,
			can_turn_in: 3,
		};
		let next_states = board.get_neighbours(&start_state);
		let expected = HashSet::from([(
			State {
				row: 0,
				col: 1,
				facing: Up,
				must_turn_in: 4,
				can_turn_in: 2,
			},
			2,
		)]);
		assert_eq!(next_states, expected);
	}

	#[test]
	fn test_up_edge() {
		let board = get_test_board();
		let start_state = State {
			row: 0,
			col: 0,
			facing: Up,
			must_turn_in: 3,
			can_turn_in: 0,
		};
		let next_states = board.get_neighbours(&start_state);
		let expected = HashSet::from([(
			State {
				row: 0,
				col: 1,
				facing: Right,
				must_turn_in: MAX_STRAIGHT_LINE - 1,
				can_turn_in: MIN_STRAIGHT_LINE - 1,
			},
			2,
		)]);
		assert_eq!(next_states, expected);
	}

	#[test]
	fn test_down() {
		let board = get_test_board();
		let start_state = State {
			row: 1,
			col: 1,
			facing: Down,
			must_turn_in: 2,
			can_turn_in: 0,
		};
		let next_states = board.get_neighbours(&start_state);
		let expected = HashSet::from([
			(
				State {
					row: 2,
					col: 1,
					facing: Down,
					must_turn_in: 1,
					can_turn_in: 0,
				},
				8,
			),
			(
				State {
					row: 1,
					col: 0,
					facing: Left,
					must_turn_in: MAX_STRAIGHT_LINE - 1,
					can_turn_in: MIN_STRAIGHT_LINE - 1,
				},
				4,
			),
			(
				State {
					row: 1,
					col: 2,
					facing: Right,
					must_turn_in: MAX_STRAIGHT_LINE - 1,
					can_turn_in: MIN_STRAIGHT_LINE - 1,
				},
				6,
			),
		]);
		assert_eq!(next_states, expected);
	}

	#[test]
	fn test_down_must_turn() {
		let board = get_test_board();
		let start_state = State {
			row: 1,
			col: 1,
			facing: Down,
			must_turn_in: 0,
			can_turn_in: 0,
		};
		let next_states = board.get_neighbours(&start_state);
		let expected = HashSet::from([
			(
				State {
					row: 1,
					col: 0,
					facing: Left,
					must_turn_in: MAX_STRAIGHT_LINE - 1,
					can_turn_in: MIN_STRAIGHT_LINE - 1,
				},
				4,
			),
			(
				State {
					row: 1,
					col: 2,
					facing: Right,
					must_turn_in: MAX_STRAIGHT_LINE - 1,
					can_turn_in: MIN_STRAIGHT_LINE - 1,
				},
				6,
			),
		]);
		assert_eq!(next_states, expected);
	}

	#[test]
	fn test_down_cant_turn() {
		let board = get_test_board();
		let start_state = State {
			row: 1,
			col: 1,
			facing: Down,
			must_turn_in: 2,
			can_turn_in: 1,
		};
		let next_states = board.get_neighbours(&start_state);
		let expected = HashSet::from([(
			State {
				row: 2,
				col: 1,
				facing: Down,
				must_turn_in: 1,
				can_turn_in: 0,
			},
			8,
		)]);
		assert_eq!(next_states, expected);
	}

	#[test]
	fn test_down_edge() {
		let board = get_test_board();
		let start_state = State {
			row: 2,
			col: 2,
			facing: Down,
			must_turn_in: 3,
			can_turn_in: 0,
		};
		let next_states = board.get_neighbours(&start_state);
		let expected = HashSet::from([(
			State {
				row: 2,
				col: 1,
				facing: Left,
				must_turn_in: MAX_STRAIGHT_LINE - 1,
				can_turn_in: MIN_STRAIGHT_LINE - 1,
			},
			8,
		)]);
		assert_eq!(next_states, expected);
	}

	#[test]
	fn test_left() {
		let board = get_test_board();
		let start_state = State {
			row: 1,
			col: 1,
			facing: Left,
			must_turn_in: 1,
			can_turn_in: 0,
		};
		let next_states = board.get_neighbours(&start_state);
		let expected = HashSet::from([
			(
				State {
					row: 1,
					col: 0,
					facing: Left,
					must_turn_in: 0,
					can_turn_in: 0,
				},
				4,
			),
			(
				State {
					row: 0,
					col: 1,
					facing: Up,
					must_turn_in: MAX_STRAIGHT_LINE - 1,
					can_turn_in: MIN_STRAIGHT_LINE - 1,
				},
				2,
			),
			(
				State {
					row: 2,
					col: 1,
					facing: Down,
					must_turn_in: MAX_STRAIGHT_LINE - 1,
					can_turn_in: MIN_STRAIGHT_LINE - 1,
				},
				8,
			),
		]);
		assert_eq!(next_states, expected);
	}

	#[test]
	fn test_right() {
		let board = get_test_board();
		let start_state = State {
			row: 1,
			col: 1,
			facing: Right,
			must_turn_in: 42,
			can_turn_in: 0,
		};
		let next_states = board.get_neighbours(&start_state);
		let expected = HashSet::from([
			(
				State {
					row: 1,
					col: 2,
					facing: Right,
					must_turn_in: 41,
					can_turn_in: 0,
				},
				6,
			),
			(
				State {
					row: 0,
					col: 1,
					facing: Up,
					must_turn_in: MAX_STRAIGHT_LINE - 1,
					can_turn_in: MIN_STRAIGHT_LINE - 1,
				},
				2,
			),
			(
				State {
					row: 2,
					col: 1,
					facing: Down,
					must_turn_in: MAX_STRAIGHT_LINE - 1,
					can_turn_in: MIN_STRAIGHT_LINE - 1,
				},
				8,
			),
		]);
		assert_eq!(next_states, expected);
	}
}
