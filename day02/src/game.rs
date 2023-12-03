/// The cube-reveal game

/// A revealed hand of cubes
#[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Hand {
	pub red: u32,
	pub green: u32,
	pub blue: u32,
}

impl Hand {
	/// "The power of a set of cubes is equal to the numbers of red, green, and blue cubes multiplied together."
	#[must_use]
	pub fn power(&self) -> u32 {
		self.red * self.blue * self.green
	}
}

/// A full game
#[derive(Debug, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Game {
	pub id: u32,
	pub hands: Vec<Hand>,
}

impl Game {
	/// Counts the smallest number of cubes of each colour possible for the game.
	#[must_use]
	fn get_min_cubes(&self) -> Hand {
		let mut min_hand = Hand {
			red: 0,
			green: 0,
			blue: 0,
		};
		for hand in &self.hands {
			min_hand.red = min_hand.red.max(hand.red);
			min_hand.green = min_hand.green.max(hand.green);
			min_hand.blue = min_hand.blue.max(hand.blue);
		}
		min_hand
	}

	/// Tests if the game is possible given a composition of cubes.
	#[must_use]
	pub fn is_possible(&self, full_hand: &Hand) -> bool {
		let min_hand = self.get_min_cubes();
		min_hand.red <= full_hand.red
			&& min_hand.green <= full_hand.green
			&& min_hand.blue <= full_hand.blue
	}

	/// "The power of a set of cubes is equal to the numbers of red, green, and blue cubes multiplied together."
	#[must_use]
	pub fn power(&self) -> u32 {
		self.get_min_cubes().power()
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_min_single_hand() {
		let hand = Hand {
			red: 3,
			green: 42,
			blue: 5,
		};
		let game = Game {
			id: 42,
			hands: vec![hand],
		};
		assert_eq!(game.get_min_cubes(), hand);
	}

	#[test]
	fn test_min_two_hands() {
		let hand1 = Hand {
			red: 3,
			green: 42,
			blue: 5,
		};
		let hand2 = Hand {
			red: 1,
			green: 0,
			blue: 23,
		};
		let expected = Hand {
			red: 3,
			green: 42,
			blue: 23,
		};
		let game = Game {
			id: 42,
			hands: vec![hand1, hand2],
		};
		assert_eq!(game.get_min_cubes(), expected);
	}

	#[test]
	fn test_possible() {
		let hand1 = Hand {
			red: 3,
			green: 42,
			blue: 5,
		};
		let hand2 = Hand {
			red: 1,
			green: 0,
			blue: 23,
		};
		let huge_hand = Hand {
			red: 100,
			green: 100,
			blue: 100,
		};
		let tiny_hand = Hand {
			red: 3,
			green: 3,
			blue: 3,
		};
		let game = Game {
			id: 42,
			hands: vec![hand1, hand2],
		};
		assert!(game.is_possible(&game.get_min_cubes()));
		assert!(game.is_possible(&huge_hand));
		assert!(!game.is_possible(&tiny_hand));
	}
}
