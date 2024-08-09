use std::{collections::HashSet, fmt};

use self::Direction::{Down, Left, Right, Up};
use crate::pretty::colourise;

/// Direction the beam can move in.
#[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Direction {
	Up,
	Down,
	Left,
	Right,
}

/// A beam entering a position.
#[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Beam {
	pub direction: Direction,
	pub row: usize,
	pub col: usize,
}

/// Optical component in a tule.
#[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Optics {
	Empty,
	MirrorL,
	MirrorΓ,
	SplitterH,
	SplitterV,
}

/// A tile the beam can traverse.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Tile {
	/// Optical component in the tile.
	pub optics: Optics,
	/// Directions of beams entering the tile.
	entering_beams: HashSet<Direction>,
}

impl Tile {
	/// Builds a new tile containing an optical component.
	#[must_use]
	pub fn from(optics: Optics) -> Self {
		Self {
			optics,
			entering_beams: HashSet::new(),
		}
	}

	/// Checks if any beam goes through the tile.
	#[must_use]
	pub fn is_lit(&self) -> bool {
		!self.entering_beams.is_empty()
	}

	/// Propagates a beam by one step. Returns direction of beam(s) exiting the tile.
	#[must_use]
	pub fn propagate(&mut self, direction: Direction) -> HashSet<Direction> {
		if self.entering_beams.contains(&direction) {
			return HashSet::new();
		}
		self.entering_beams.insert(direction);
		match self.optics {
			Optics::Empty => HashSet::from([direction]),
			Optics::MirrorL => HashSet::from([match direction {
				Up => Left,
				Down => Right,
				Left => Up,
				Right => Down,
			}]),
			Optics::MirrorΓ => HashSet::from([match direction {
				Up => Right,
				Down => Left,
				Left => Down,
				Right => Up,
			}]),
			Optics::SplitterH => match direction {
				Up | Down => HashSet::from([Left, Right]),
				Left | Right => HashSet::from([direction]),
			},
			Optics::SplitterV => match direction {
				Up | Down => HashSet::from([direction]),
				Left | Right => HashSet::from([Up, Down]),
			},
		}
	}
}

impl fmt::Display for Tile {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let symbol = match self.optics {
			Optics::Empty => ' ',
			Optics::MirrorL => '╲',
			Optics::MirrorΓ => '╱',
			Optics::SplitterH => '─',
			Optics::SplitterV => '│',
		};
		let (background, foreground) = if self.is_lit() { (220, 209) } else { (0, 248) };
		write!(f, "{}", colourise(symbol, background, foreground))
	}
}

/// A 2D array of tiles.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Board {
	pub nb_rows: usize,
	pub nb_cols: usize,
	pub tiles: Vec<Vec<Tile>>,
}

impl Board {
	/// Builds a board from an 2D array of tiles.
	#[must_use]
	pub fn from(tiles: Vec<Vec<Tile>>) -> Self {
		Self {
			nb_rows: tiles.len(),
			nb_cols: tiles.first().expect("Empty grid").len(),
			tiles,
		}
	}

	/// Counts how many tiles are lit.
	#[must_use]
	pub fn count_lit_tiles(&self) -> usize {
		self.tiles
			.iter()
			.map(|row| row.iter().filter(|tile| tile.is_lit()).count())
			.sum()
	}

	/// Propagates a beam by one step. Returns beam(s) at the next step.
	#[must_use]
	pub fn propagate(&mut self, beam: Beam) -> HashSet<Beam> {
		let exit_dirs = self.tiles[beam.row][beam.col].propagate(beam.direction);
		let mut exit_beams = HashSet::new();
		for direction in exit_dirs {
			match direction {
				Up => {
					if beam.row > 0 {
						exit_beams.insert(Beam {
							row: beam.row - 1,
							col: beam.col,
							direction,
						});
					}
				}
				Down => {
					if beam.row < self.nb_rows - 1 {
						exit_beams.insert(Beam {
							row: beam.row + 1,
							col: beam.col,
							direction,
						});
					}
				}
				Left => {
					if beam.col > 0 {
						exit_beams.insert(Beam {
							row: beam.row,
							col: beam.col - 1,
							direction,
						});
					}
				}
				Right => {
					if beam.col < self.nb_cols - 1 {
						exit_beams.insert(Beam {
							row: beam.row,
							col: beam.col + 1,
							direction,
						});
					}
				}
			}
		}
		exit_beams
	}

	/// Propagates a beam until no more new beams are produced.
	pub fn trace(&mut self, initial_beam: Beam) {
		for exit_beam in self.propagate(initial_beam) {
			self.trace(exit_beam);
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
mod test_is_lit {
	use super::*;

	#[test]
	fn test_off() {
		let tile = Tile::from(Optics::Empty);
		assert!(!tile.is_lit());
	}

	#[test]
	fn test_on() {
		let tile = Tile {
			optics: Optics::Empty,
			entering_beams: HashSet::from([Up]),
		};
		assert!(tile.is_lit());
	}
}

#[cfg(test)]
mod test_propagate {
	use super::*;
	use super::{Down, Left, Right, Up};
	use crate::parse_input::parse_full;

	#[test]
	fn test_up_through_empty() {
		let mut board = parse_full("...\n...\n...\n");
		let beam_in = Beam {
			row: 1,
			col: 1,
			direction: Up,
		};
		let beams_out = board.propagate(beam_in);

		assert_eq!(
			beams_out,
			HashSet::from([Beam {
				row: 0,
				col: 1,
				direction: Up
			}])
		);
		assert!(board.tiles[1][1].is_lit());
	}

	#[test]
	fn test_down_through_empty() {
		let mut board = parse_full("...\n...\n...\n");
		let beam_in = Beam {
			row: 1,
			col: 1,
			direction: Down,
		};
		let beams_out = board.propagate(beam_in);

		assert_eq!(
			beams_out,
			HashSet::from([Beam {
				row: 2,
				col: 1,
				direction: Down
			}])
		);
		assert!(board.tiles[1][1].is_lit());
	}

	#[test]
	fn test_left_through_empty() {
		let mut board = parse_full("...\n...\n...\n");
		let beam_in = Beam {
			row: 1,
			col: 1,
			direction: Left,
		};
		let beams_out = board.propagate(beam_in);

		assert_eq!(
			beams_out,
			HashSet::from([Beam {
				row: 1,
				col: 0,
				direction: Left
			}])
		);
		assert!(board.tiles[1][1].is_lit());
	}

	#[test]
	fn test_right_through_empty() {
		let mut board = parse_full("...\n...\n...\n");
		let beam_in = Beam {
			row: 1,
			col: 1,
			direction: Right,
		};
		let beams_out = board.propagate(beam_in);

		assert_eq!(
			beams_out,
			HashSet::from([Beam {
				row: 1,
				col: 2,
				direction: Right
			}])
		);
		assert!(board.tiles[1][1].is_lit());
	}

	#[test]
	fn test_up_into_edge() {
		let mut board = parse_full("..\n..\n");
		let beam_in = Beam {
			row: 0,
			col: 0,
			direction: Up,
		};
		let beams_out = board.propagate(beam_in);

		assert_eq!(beams_out, HashSet::new());
		assert!(board.tiles[0][0].is_lit());
	}

	#[test]
	fn test_down_into_edge() {
		let mut board = parse_full("..\n..\n");
		let beam_in = Beam {
			row: 1,
			col: 0,
			direction: Down,
		};
		let beams_out = board.propagate(beam_in);

		assert_eq!(beams_out, HashSet::new());
		assert!(board.tiles[1][0].is_lit());
	}

	#[test]
	fn test_left_into_edge() {
		let mut board = parse_full("..\n..\n");
		let beam_in = Beam {
			row: 1,
			col: 0,
			direction: Left,
		};
		let beams_out = board.propagate(beam_in);

		assert_eq!(beams_out, HashSet::new());
		assert!(board.tiles[1][0].is_lit());
	}

	#[test]
	fn test_right_into_edge() {
		let mut board = parse_full("..\n..\n");
		let beam_in = Beam {
			row: 0,
			col: 1,
			direction: Right,
		};
		let beams_out = board.propagate(beam_in);

		assert_eq!(beams_out, HashSet::new());
		assert!(board.tiles[0][1].is_lit());
	}

	#[test]
	fn test_reflect_up_to_left() {
		let mut board = parse_full("...\n.\\.\n...\n");
		let beam_in = Beam {
			row: 1,
			col: 1,
			direction: Up,
		};
		let beams_out = board.propagate(beam_in);

		assert_eq!(
			beams_out,
			HashSet::from([Beam {
				row: 1,
				col: 0,
				direction: Left
			}])
		);
		assert!(board.tiles[1][1].is_lit());
	}

	#[test]
	fn test_reflect_up_to_right() {
		let mut board = parse_full("...\n./.\n...\n");
		let beam_in = Beam {
			row: 1,
			col: 1,
			direction: Up,
		};
		let beams_out = board.propagate(beam_in);

		assert_eq!(
			beams_out,
			HashSet::from([Beam {
				row: 1,
				col: 2,
				direction: Right
			}])
		);
		assert!(board.tiles[1][1].is_lit());
	}

	#[test]
	fn test_reflect_down_to_left() {
		let mut board = parse_full("...\n./.\n...\n");
		let beam_in = Beam {
			row: 1,
			col: 1,
			direction: Down,
		};
		let beams_out = board.propagate(beam_in);

		assert_eq!(
			beams_out,
			HashSet::from([Beam {
				row: 1,
				col: 0,
				direction: Left
			}])
		);
		assert!(board.tiles[1][1].is_lit());
	}

	#[test]
	fn test_reflect_down_to_right() {
		let mut board = parse_full("...\n.\\.\n...\n");
		let beam_in = Beam {
			row: 1,
			col: 1,
			direction: Down,
		};
		let beams_out = board.propagate(beam_in);

		assert_eq!(
			beams_out,
			HashSet::from([Beam {
				row: 1,
				col: 2,
				direction: Right
			}])
		);
		assert!(board.tiles[1][1].is_lit());
	}

	#[test]
	fn test_reflect_left_to_up() {
		let mut board = parse_full("...\n.\\.\n...\n");
		let beam_in = Beam {
			row: 1,
			col: 1,
			direction: Left,
		};
		let beams_out = board.propagate(beam_in);

		assert_eq!(
			beams_out,
			HashSet::from([Beam {
				row: 0,
				col: 1,
				direction: Up
			}])
		);
		assert!(board.tiles[1][1].is_lit());
	}

	#[test]
	fn test_reflect_left_to_down() {
		let mut board = parse_full("...\n./.\n...\n");
		let beam_in = Beam {
			row: 1,
			col: 1,
			direction: Left,
		};
		let beams_out = board.propagate(beam_in);

		assert_eq!(
			beams_out,
			HashSet::from([Beam {
				row: 2,
				col: 1,
				direction: Down
			}])
		);
		assert!(board.tiles[1][1].is_lit());
	}

	#[test]
	fn test_reflect_right_to_up() {
		let mut board = parse_full("...\n./.\n...\n");
		let beam_in = Beam {
			row: 1,
			col: 1,
			direction: Right,
		};
		let beams_out = board.propagate(beam_in);

		assert_eq!(
			beams_out,
			HashSet::from([Beam {
				row: 0,
				col: 1,
				direction: Up
			}])
		);
		assert!(board.tiles[1][1].is_lit());
	}

	#[test]
	fn test_reflect_right_to_down() {
		let mut board = parse_full("...\n.\\.\n...\n");
		let beam_in = Beam {
			row: 1,
			col: 1,
			direction: Right,
		};
		let beams_out = board.propagate(beam_in);

		assert_eq!(
			beams_out,
			HashSet::from([Beam {
				row: 2,
				col: 1,
				direction: Down
			}])
		);
		assert!(board.tiles[1][1].is_lit());
	}

	#[test]
	fn test_up_ignore_splitter_vert() {
		let mut board = parse_full("...\n.|.\n...\n");
		let beam_in = Beam {
			row: 1,
			col: 1,
			direction: Up,
		};
		let beams_out = board.propagate(beam_in);

		assert_eq!(
			beams_out,
			HashSet::from([Beam {
				row: 0,
				col: 1,
				direction: Up
			}])
		);
		assert!(board.tiles[1][1].is_lit());
	}

	#[test]
	fn test_down_ignore_splitter_vert() {
		let mut board = parse_full("...\n.|.\n...\n");
		let beam_in = Beam {
			row: 1,
			col: 1,
			direction: Down,
		};
		let beams_out = board.propagate(beam_in);

		assert_eq!(
			beams_out,
			HashSet::from([Beam {
				row: 2,
				col: 1,
				direction: Down
			}])
		);
		assert!(board.tiles[1][1].is_lit());
	}

	#[test]
	fn test_left_ignore_splitter_horiz() {
		let mut board = parse_full("...\n.-.\n...\n");
		let beam_in = Beam {
			row: 1,
			col: 1,
			direction: Left,
		};
		let beams_out = board.propagate(beam_in);

		assert_eq!(
			beams_out,
			HashSet::from([Beam {
				row: 1,
				col: 0,
				direction: Left
			}])
		);
		assert!(board.tiles[1][1].is_lit());
	}

	#[test]
	fn test_right_ignore_splitter_horiz() {
		let mut board = parse_full("...\n.-.\n...\n");
		let beam_in = Beam {
			row: 1,
			col: 1,
			direction: Right,
		};
		let beams_out = board.propagate(beam_in);

		assert_eq!(
			beams_out,
			HashSet::from([Beam {
				row: 1,
				col: 2,
				direction: Right
			}])
		);
		assert!(board.tiles[1][1].is_lit());
	}

	#[test]
	fn test_up_split_horiz() {
		let mut board = parse_full("...\n.-.\n...\n");
		let beam_in = Beam {
			row: 1,
			col: 1,
			direction: Up,
		};
		let beams_out = board.propagate(beam_in);

		assert_eq!(
			beams_out,
			HashSet::from([
				Beam {
					row: 1,
					col: 0,
					direction: Left
				},
				Beam {
					row: 1,
					col: 2,
					direction: Right
				}
			])
		);
		assert!(board.tiles[1][1].is_lit());
	}

	#[test]
	fn test_down_split_horiz() {
		let mut board = parse_full("...\n.-.\n...\n");
		let beam_in = Beam {
			row: 1,
			col: 1,
			direction: Down,
		};
		let beams_out = board.propagate(beam_in);

		assert_eq!(
			beams_out,
			HashSet::from([
				Beam {
					row: 1,
					col: 0,
					direction: Left
				},
				Beam {
					row: 1,
					col: 2,
					direction: Right
				}
			])
		);
		assert!(board.tiles[1][1].is_lit());
	}

	#[test]
	fn test_left_split_vert() {
		let mut board = parse_full("...\n.|.\n...\n");
		let beam_in = Beam {
			row: 1,
			col: 1,
			direction: Left,
		};
		let beams_out = board.propagate(beam_in);

		assert_eq!(
			beams_out,
			HashSet::from([
				Beam {
					row: 0,
					col: 1,
					direction: Up
				},
				Beam {
					row: 2,
					col: 1,
					direction: Down
				}
			])
		);
		assert!(board.tiles[1][1].is_lit());
	}

	#[test]
	fn test_right_split_vert() {
		let mut board = parse_full("...\n.|.\n...\n");
		let beam_in = Beam {
			row: 1,
			col: 1,
			direction: Right,
		};
		let beams_out = board.propagate(beam_in);

		assert_eq!(
			beams_out,
			HashSet::from([
				Beam {
					row: 0,
					col: 1,
					direction: Up
				},
				Beam {
					row: 2,
					col: 1,
					direction: Down
				}
			])
		);
		assert!(board.tiles[1][1].is_lit());
	}

	#[test]
	fn test_split_into_edge() {
		let mut board = parse_full("..\n.-\n..\n");
		let beam_in = Beam {
			row: 1,
			col: 1,
			direction: Up,
		};
		let beams_out = board.propagate(beam_in);

		assert_eq!(
			beams_out,
			HashSet::from([Beam {
				row: 1,
				col: 0,
				direction: Left
			}])
		);
		assert!(board.tiles[1][1].is_lit());
	}

	#[test]
	fn test_duplicate_beams() {
		let mut board = parse_full("...\n./.\n...\n");
		let beam_in = Beam {
			row: 1,
			col: 1,
			direction: Right,
		};

		let first_beams_out = board.propagate(beam_in);
		assert_eq!(
			first_beams_out,
			HashSet::from([Beam {
				row: 0,
				col: 1,
				direction: Up
			}])
		);

		let second_beams_out = board.propagate(beam_in);
		assert_eq!(second_beams_out, HashSet::new());

		assert!(board.tiles[1][1].is_lit());
	}

	#[test]
	fn test_different_beams() {
		let mut board = parse_full("...\n.\\.\n...\n");

		let first_beam_in = Beam {
			row: 1,
			col: 1,
			direction: Right,
		};
		let first_beams_out = board.propagate(first_beam_in);
		assert_eq!(
			first_beams_out,
			HashSet::from([Beam {
				row: 2,
				col: 1,
				direction: Down
			}])
		);

		let second_beam_in = Beam {
			row: 1,
			col: 1,
			direction: Left,
		};
		let second_beams_out = board.propagate(second_beam_in);
		assert_eq!(
			second_beams_out,
			HashSet::from([Beam {
				row: 0,
				col: 1,
				direction: Up
			}])
		);

		assert!(board.tiles[1][1].is_lit());
	}
}

#[cfg(test)]
mod test_trace {
	use super::*;
	use crate::parse_input::parse_full;

	#[test]
	fn test_initial() {
		let board = parse_full("...\n...\n");
		assert_eq!(board.count_lit_tiles(), 0);
	}

	#[test]
	fn test_to_edge() {
		let mut board = parse_full("...\n...\n");
		let beam = Beam {
			row: 0,
			col: 0,
			direction: Direction::Right,
		};
		board.trace(beam);

		assert_eq!(board.count_lit_tiles(), 3);
	}

	#[test]
	fn test_cycle() {
		let mut board = parse_full("-.|\n...\n|.-\n");
		let beam = Beam {
			row: 0,
			col: 0,
			direction: Direction::Right,
		};
		board.trace(beam);

		// Lights all tiles in the cycle
		assert_eq!(board.count_lit_tiles(), 8);

		// But not the central tile
		assert!(!board.tiles[1][1].is_lit());
	}
}
