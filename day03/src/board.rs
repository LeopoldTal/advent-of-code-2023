/// A horizontal span on the 2D board, occupied by a number.
#[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Span {
	pub row: usize,
	pub start_col: usize, // inclusive
	pub end_col: usize,   // exclusive
}

/// A 2D char array.
#[derive(Debug, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Board {
	pub nb_rows: usize,
	pub nb_cols: usize,
	pub tiles: Vec<Vec<char>>,
}

/// Reads a board from a string.
#[must_use]
pub fn parse(s: &str) -> Board {
	let tiles: Vec<Vec<char>> = s.lines().map(|line| line.chars().collect()).collect();
	Board {
		nb_rows: tiles.len(),
		nb_cols: tiles.first().expect("Empty board").len(),
		tiles,
	}
}

/// Tests if a char is a symbol
#[must_use]
fn is_symbol(ch: char) -> bool {
	ch != '.' && ch.is_ascii_punctuation()
}

impl Board {
	/// Finds all numbers horizontally written on the board.
	#[must_use]
	fn find_numbers(&self) -> Vec<Span> {
		let mut spans: Vec<Span> = vec![];
		for row in 0..self.nb_rows {
			let mut start_digit: Option<usize> = None;
			for col in 0..self.nb_cols {
				if self.tiles[row][col].is_ascii_digit() {
					// Start a span if this is the first digit.
					if start_digit.is_none() {
						start_digit = Some(col);
					}
				} else {
					// End a span if any.
					if let Some(start_col) = start_digit {
						let new_span = Span {
							row,
							start_col,
							end_col: col,
						};
						spans.push(new_span);
						start_digit = None;
					}
				}
			}
			// At end of row, end current span if any.
			if let Some(start_col) = start_digit {
				let new_span = Span {
					row,
					start_col,
					end_col: self.nb_cols,
				};
				spans.push(new_span);
			}
		}
		spans
	}

	/// Reads a number horizontally written on the board.
	#[must_use]
	fn read_number(&self, span: &Span) -> u32 {
		let mut n = 0;
		for index in span.start_col..span.end_col {
			let next_char = self.tiles[span.row][index];
			let digit = next_char
				.to_digit(10)
				.unwrap_or_else(|| panic!("Not a digit: {next_char}"));
			n *= 10;
			n += digit;
		}
		n
	}

	/// Checks if there is any punctuation in the ring around a number.
	#[must_use]
	fn scan_halo(&self, span: &Span) -> bool {
		let min_col = if span.start_col > 0 {
			span.start_col - 1
		} else {
			0
		};
		let max_col = if span.end_col < self.nb_cols {
			span.end_col + 1
		} else {
			self.nb_cols
		};

		// Above
		if span.row > 0 {
			for col in min_col..max_col {
				if is_symbol(self.tiles[span.row - 1][col]) {
					return true;
				}
			}
		}

		// Below
		if span.row < self.nb_cols - 1 {
			for col in min_col..max_col {
				if is_symbol(self.tiles[span.row + 1][col]) {
					return true;
				}
			}
		}

		// Left
		if span.start_col > 0 && is_symbol(self.tiles[span.row][span.start_col - 1]) {
			return true;
		}

		// Right
		if span.end_col < self.nb_cols && is_symbol(self.tiles[span.row][span.end_col]) {
			return true;
		}

		false
	}

	/// Finds all the numbers whose halo contains a symbol.
	#[must_use]
	pub fn get_part_numbers(&self) -> Vec<u32> {
		self.find_numbers()
			.into_iter()
			.filter(|span| self.scan_halo(span))
			.map(|span| self.read_number(&span))
			.collect()
	}
}

#[cfg(test)]
mod test {
	use super::*;

	const SAMPLE_INPUT: &str = include_str!("../input_sample.txt");

	#[test]
	fn test_to_board() {
		let input = ".12\n*..\n";
		let expected = Board {
			nb_rows: 2,
			nb_cols: 3,
			tiles: vec![vec!['.', '1', '2'], vec!['*', '.', '.']],
		};
		assert_eq!(parse(&input), expected);
	}

	#[test]
	fn test_find_numbers() {
		let board = parse("1.2\n345\n.67\n");
		let expected = vec![
			Span {
				row: 0,
				start_col: 0,
				end_col: 1,
			},
			Span {
				row: 0,
				start_col: 2,
				end_col: 3,
			},
			Span {
				row: 1,
				start_col: 0,
				end_col: 3,
			},
			Span {
				row: 2,
				start_col: 1,
				end_col: 3,
			},
		];
		assert_eq!(board.find_numbers(), expected);
	}

	#[test]
	fn test_read_number() {
		let board = parse(".12\n*..\n");
		let span = Span {
			row: 0,
			start_col: 1,
			end_col: 3,
		};
		assert_eq!(board.read_number(&span), 12);
	}

	#[test]
	fn test_scan_middle() {
		let board = parse(
			"........
..456...
........
....123.
...&....
",
		);
		let no_hit = Span {
			row: 1,
			start_col: 2,
			end_col: 5,
		};
		let yes_hit = Span {
			row: 3,
			start_col: 4,
			end_col: 7,
		};
		assert!(!board.scan_halo(&no_hit));
		assert!(board.scan_halo(&yes_hit));
	}

	#[test]
	fn test_scan_edge() {
		let board = parse(
			"789....
.....*.
....246
",
		);
		let no_hit = Span {
			row: 0,
			start_col: 0,
			end_col: 3,
		};
		let yes_hit = Span {
			row: 2,
			start_col: 4,
			end_col: 7,
		};
		assert!(!board.scan_halo(&no_hit));
		assert!(board.scan_halo(&yes_hit));
	}

	#[test]
	fn test_get_part_numbers() {
		let board = parse(&SAMPLE_INPUT);
		let expected = vec![467, 35, 633, 617, 592, 755, 664, 598];
		assert_eq!(board.get_part_numbers(), expected);
	}
}
