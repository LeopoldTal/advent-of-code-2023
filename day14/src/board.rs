use std::fmt;

use crate::pretty::{colourise, step_frame};

/// Tile a rock can occupy.
#[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Tile {
	Empty,
	Wall,
	Movable,
}

impl fmt::Display for Tile {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let (symbol, background, foreground) = match self {
			Tile::Empty => (' ', 0, 0),
			Tile::Wall => ('#', 250, 249),
			Tile::Movable => ('●', 0, 214),
		};
		write!(f, "{}", colourise(symbol, background, foreground))
	}
}

/// A 2D array of rocks.
#[derive(Debug, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Board {
	pub nb_rows: usize,
	pub nb_cols: usize,
	pub tiles: Vec<Vec<Tile>>,
}

/// Signed addition on usize
/// # Panics
/// On out-of-bounds.
#[must_use]
fn add(a: usize, b: isize) -> usize {
	let a_signed: isize = a.try_into().expect("Out of bounds: too big");
	(a_signed + b).try_into().expect("Out of bounds: negative")
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

	/// Slides all the movable rocks as far as possible in a given direction.
	pub fn slide_by(
		&mut self,
		row_range: Vec<usize>,
		col_range: Vec<usize>,
		row_step: isize,
		col_step: isize,
		reporting: bool,
	) {
		let mut any_moved = false;
		for &row_here in &row_range {
			for &col_here in &col_range {
				let row_to = add(row_here, row_step);
				let col_to = add(col_here, col_step);
				if self.tiles[row_here][col_here] == Tile::Movable
					&& self.tiles[row_to][col_to] == Tile::Empty
				{
					any_moved = true;
					self.tiles[row_here][col_here] = Tile::Empty;
					self.tiles[row_to][col_to] = Tile::Movable;
				}
			}
		}
		if reporting {
			step_frame();
			println!("{}", &self);
		}
		if any_moved {
			self.slide_by(row_range, col_range, row_step, col_step, reporting);
		}
	}

	/// Slides all the movable rocks as far north as possible.
	pub fn slide_north(&mut self, reporting: bool) {
		let row_range: Vec<usize> = (1..self.nb_rows).collect();
		let col_range: Vec<usize> = (0..self.nb_cols).collect();
		let row_step = -1;
		let col_step = 0;
		self.slide_by(row_range, col_range, row_step, col_step, reporting);
	}

	/// Slides all the movable rocks as far south as possible.
	pub fn slide_south(&mut self, reporting: bool) {
		let row_range: Vec<usize> = (0..self.nb_rows - 1).rev().collect();
		let col_range: Vec<usize> = (0..self.nb_cols).collect();
		let row_step = 1;
		let col_step = 0;
		self.slide_by(row_range, col_range, row_step, col_step, reporting);
	}

	/// Slides all the movable rocks as far west as possible.
	pub fn slide_west(&mut self, reporting: bool) {
		let row_range: Vec<usize> = (0..self.nb_rows).collect();
		let col_range: Vec<usize> = (1..self.nb_cols).collect();
		let row_step = 0;
		let col_step = -1;
		self.slide_by(row_range, col_range, row_step, col_step, reporting);
	}

	/// Slides all the movable rocks as far east as possible.
	pub fn slide_east(&mut self, reporting: bool) {
		let row_range: Vec<usize> = (0..self.nb_rows).collect();
		let col_range: Vec<usize> = (0..self.nb_cols - 1).rev().collect();
		let row_step = 0;
		let col_step = 1;
		self.slide_by(row_range, col_range, row_step, col_step, reporting);
	}

	/// Slides all the movable rocks north, then west, then south, then east.
	pub fn spin_once(&mut self, reporting: bool) {
		self.slide_north(reporting);
		self.slide_west(reporting);
		self.slide_south(reporting);
		self.slide_east(reporting);
	}

	/// Spins the board A BILLION! times.
	pub fn spin_many(&mut self, reporting: bool) {
		let total_spins = 1_000_000_000;
		let mut previous_boards = vec![self.clone()];
		for spin_count in 1..=total_spins {
			self.spin_once(reporting);
			let duplicate = previous_boards.iter().position(|board| board == self);
			if let Some(dup_spin_count) = duplicate {
				let cycle_length = spin_count - dup_spin_count;

				let remaining_spins = (total_spins - spin_count) % cycle_length;
				println!("Stable after {spin_count} spins. Cycle length: {cycle_length}. Need {remaining_spins} to match end state.");

				for _ in 0..remaining_spins {
					// Could just read it from `previous_boards`, but I enjoy watching it go.
					self.spin_once(reporting);
				}

				return;
			}
			previous_boards.push(self.clone());
		}
	}

	/// Computes the load from the position of rocks.
	pub fn get_load(&self) -> usize {
		(0..self.nb_rows)
			.map(|row_index| {
				(self.nb_rows - row_index)
					* self.tiles[row_index]
						.iter()
						.filter(|&&tile| tile == Tile::Movable)
						.count()
			})
			.sum()
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
mod test_slide_north {
	use super::Tile::{Empty, Movable, Wall};
	use super::*;

	#[test]
	fn test_no_movables() {
		let mut board = Board::from(vec![vec![Empty; 3], vec![Empty; 3]]);
		let expected = board.clone();
		board.slide_north(false);
		assert_eq!(board, expected);
	}

	#[test]
	fn test_hit_edge() {
		let mut board = Board::from(vec![
			vec![Empty; 3],
			vec![Movable, Empty, Empty],
			vec![Empty, Movable, Empty],
		]);
		let expected = Board::from(vec![
			vec![Movable, Movable, Empty],
			vec![Empty; 3],
			vec![Empty; 3],
		]);
		board.slide_north(false);
		assert_eq!(board, expected);
	}

	#[test]
	fn test_hit_wall() {
		let mut board = Board::from(vec![
			vec![Wall, Empty],
			vec![Empty, Wall],
			vec![Movable, Movable],
		]);
		let expected = Board::from(vec![
			vec![Wall, Empty],
			vec![Movable, Wall],
			vec![Empty, Movable],
		]);
		board.slide_north(false);
		assert_eq!(board, expected);
	}

	#[test]
	fn test_hit_stopped_movable() {
		let mut board = Board::from(vec![
			vec![Movable, Wall],
			vec![Empty, Movable],
			vec![Empty, Empty],
			vec![Movable, Movable],
		]);
		let expected = Board::from(vec![
			vec![Movable, Wall],
			vec![Movable, Movable],
			vec![Empty, Movable],
			vec![Empty, Empty],
		]);
		board.slide_north(false);
		assert_eq!(board, expected);
	}

	#[test]
	fn test_queueing_movables() {
		let mut board = Board::from(vec![
			vec![Empty],
			vec![Empty],
			vec![Movable],
			vec![Movable],
			vec![Empty],
			vec![Movable],
		]);
		let expected = Board::from(vec![
			vec![Movable],
			vec![Movable],
			vec![Movable],
			vec![Empty],
			vec![Empty],
			vec![Empty],
		]);
		board.slide_north(false);
		assert_eq!(board, expected);
	}
}

#[cfg(test)]
mod test_slide_south {
	use super::Tile::{Empty, Movable, Wall};
	use super::*;

	#[test]
	fn test_no_movables() {
		let mut board = Board::from(vec![vec![Empty; 3], vec![Empty; 3]]);
		let expected = board.clone();
		board.slide_south(false);
		assert_eq!(board, expected);
	}

	#[test]
	fn test_hit_edge() {
		let mut board = Board::from(vec![
			vec![Empty; 3],
			vec![Movable, Empty, Empty],
			vec![Empty, Movable, Empty],
		]);
		let expected = Board::from(vec![
			vec![Empty; 3],
			vec![Empty; 3],
			vec![Movable, Movable, Empty],
		]);
		board.slide_south(false);
		assert_eq!(board, expected);
	}

	#[test]
	fn test_hit_wall() {
		let mut board = Board::from(vec![
			vec![Movable, Movable],
			vec![Wall, Empty],
			vec![Empty, Wall],
		]);
		let expected = Board::from(vec![
			vec![Movable, Empty],
			vec![Wall, Movable],
			vec![Empty, Wall],
		]);
		board.slide_south(false);
		assert_eq!(board, expected);
	}

	#[test]
	fn test_hit_stopped_movable() {
		let mut board = Board::from(vec![
			vec![Movable, Wall],
			vec![Empty, Movable],
			vec![Empty, Empty],
			vec![Movable, Movable],
		]);
		let expected = Board::from(vec![
			vec![Empty, Wall],
			vec![Empty, Empty],
			vec![Movable, Movable],
			vec![Movable, Movable],
		]);
		board.slide_south(false);
		assert_eq!(board, expected);
	}

	#[test]
	fn test_queueing_movables() {
		let mut board = Board::from(vec![
			vec![Movable],
			vec![Movable],
			vec![Empty],
			vec![Movable],
			vec![Empty],
			vec![Empty],
		]);
		let expected = Board::from(vec![
			vec![Empty],
			vec![Empty],
			vec![Empty],
			vec![Movable],
			vec![Movable],
			vec![Movable],
		]);
		board.slide_south(false);
		assert_eq!(board, expected);
	}
}

#[cfg(test)]
mod test_slide_west {
	use super::Tile::{Empty, Movable, Wall};
	use super::*;

	#[test]
	fn test_no_movables() {
		let mut board = Board::from(vec![vec![Empty; 3], vec![Empty; 3]]);
		let expected = board.clone();
		board.slide_west(false);
		assert_eq!(board, expected);
	}

	#[test]
	fn test_hit_edge() {
		let mut board = Board::from(vec![
			vec![Empty; 3],
			vec![Movable, Empty, Empty],
			vec![Empty, Movable, Empty],
		]);
		let expected = Board::from(vec![
			vec![Empty; 3],
			vec![Movable, Empty, Empty],
			vec![Movable, Empty, Empty],
		]);
		board.slide_west(false);
		assert_eq!(board, expected);
	}

	#[test]
	fn test_hit_wall() {
		let mut board = Board::from(vec![vec![Wall, Empty, Movable], vec![Empty, Wall, Movable]]);
		let expected = Board::from(vec![vec![Wall, Movable, Empty], vec![Empty, Wall, Movable]]);
		board.slide_west(false);
		assert_eq!(board, expected);
	}

	#[test]
	fn test_hit_stopped_movable() {
		let mut board = Board::from(vec![
			vec![Wall, Movable, Empty, Movable],
			vec![Movable, Empty, Empty, Movable],
		]);
		let expected = Board::from(vec![
			vec![Wall, Movable, Movable, Empty],
			vec![Movable, Movable, Empty, Empty],
		]);
		board.slide_west(false);
		assert_eq!(board, expected);
	}

	#[test]
	fn test_queueing_movables() {
		let mut board = Board::from(vec![vec![Empty, Empty, Movable, Movable, Empty, Movable]]);
		let expected = Board::from(vec![vec![Movable, Movable, Movable, Empty, Empty, Empty]]);
		board.slide_west(false);
		assert_eq!(board, expected);
	}
}

#[cfg(test)]
mod test_slide_east {
	use super::Tile::{Empty, Movable, Wall};
	use super::*;

	#[test]
	fn test_no_movables() {
		let mut board = Board::from(vec![vec![Empty; 3], vec![Empty; 3]]);
		let expected = board.clone();
		board.slide_east(false);
		assert_eq!(board, expected);
	}

	#[test]
	fn test_hit_edge() {
		let mut board = Board::from(vec![
			vec![Empty; 3],
			vec![Movable, Empty, Empty],
			vec![Empty, Movable, Empty],
		]);
		let expected = Board::from(vec![
			vec![Empty; 3],
			vec![Empty, Empty, Movable],
			vec![Empty, Empty, Movable],
		]);
		board.slide_east(false);
		assert_eq!(board, expected);
	}

	#[test]
	fn test_hit_wall() {
		let mut board = Board::from(vec![vec![Movable, Empty, Wall], vec![Movable, Wall, Empty]]);
		let expected = Board::from(vec![vec![Empty, Movable, Wall], vec![Movable, Wall, Empty]]);
		board.slide_east(false);
		assert_eq!(board, expected);
	}

	#[test]
	fn test_hit_stopped_movable() {
		let mut board = Board::from(vec![
			vec![Movable, Empty, Movable, Wall],
			vec![Movable, Empty, Empty, Movable],
		]);
		let expected = Board::from(vec![
			vec![Empty, Movable, Movable, Wall],
			vec![Empty, Empty, Movable, Movable],
		]);
		board.slide_east(false);
		assert_eq!(board, expected);
	}

	#[test]
	fn test_queueing_movables() {
		let mut board = Board::from(vec![vec![Empty, Empty, Movable, Movable, Empty, Movable]]);
		let expected = Board::from(vec![vec![Empty, Empty, Empty, Movable, Movable, Movable]]);
		board.slide_east(false);
		assert_eq!(board, expected);
	}
}

#[cfg(test)]
mod test_load {
	use super::Tile::{Empty, Movable, Wall};
	use super::*;

	#[test]
	fn test_no_movables() {
		let board = Board::from(vec![vec![Wall], vec![Empty]]);
		assert_eq!(board.get_load(), 0);
	}

	#[test]
	fn test_at_top() {
		let board = Board::from(vec![vec![Movable], vec![Empty], vec![Empty]]);
		assert_eq!(board.get_load(), 3);
	}

	#[test]
	fn test_at_bottom() {
		let board = Board::from(vec![vec![Empty], vec![Empty], vec![Movable]]);
		assert_eq!(board.get_load(), 1);
	}

	#[test]
	fn test_full_column() {
		let board = Board::from(vec![vec![Movable]; 4]);
		assert_eq!(board.get_load(), 1 + 2 + 3 + 4);
	}

	#[test]
	fn test_multi_columns() {
		let board = Board::from(vec![vec![Movable; 3]]);
		assert_eq!(board.get_load(), 3);
	}
}
