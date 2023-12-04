use crate::scratchcard::Scratchcard;

type CardCount = (Scratchcard, u32);

// A list of scratchies that make more scratchies.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CardList {
	pub card_counts: Vec<CardCount>,
}

impl CardList {
	/// Get number of matches and number of copies of card at given index.
	fn get_counts_at(&self, index: usize) -> (usize, u32) {
		let (card, repeats) = &self.card_counts[index];
		let nb_wins = card.get_matches().len();
		(nb_wins, *repeats)
	}

	/// Create more cards from given card.
	fn scratch_at(&mut self, index: usize) {
		let (nb_wins, repeats) = self.get_counts_at(index);
		for win_index in index + 1..=index + nb_wins {
			self.card_counts[win_index].1 += repeats;
		}
	}

	/// Create all cards in order.
	pub fn scratch(&mut self) {
		for index in 0..self.card_counts.len() {
			self.scratch_at(index);
		}
	}

	/// Counts the total number of cards, with repeats.
	pub fn count_cards(&self) -> u32 {
		self.card_counts.iter().map(|(_, repeats)| repeats).sum()
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_no_wins() {
		let no_wins = Scratchcard::from(1, [], []);
		let end_card = Scratchcard::from(2, [], []);
		let mut list = CardList {
			card_counts: vec![(no_wins, 1), (end_card, 1)],
		};

		list.scratch_at(0);

		assert_eq!(list.card_counts[0].1, 1);
		assert_eq!(list.card_counts[1].1, 1);
	}

	#[test]
	fn test_one_win() {
		let one_win = Scratchcard::from(1, [100], [100]);
		let no_wins = Scratchcard::from(2, [], []);
		let end_card = Scratchcard::from(3, [], []);
		let mut list = CardList {
			card_counts: vec![(one_win, 1), (no_wins, 1), (end_card, 1)],
		};

		list.scratch_at(0);

		assert_eq!(list.card_counts[0].1, 1);
		assert_eq!(list.card_counts[1].1, 2);
		assert_eq!(list.card_counts[2].1, 1);
	}

	#[test]
	fn test_multi_wins() {
		let start_card = Scratchcard::from(0, [], []);
		let two_wins = Scratchcard::from(1, [1, 2], [1, 2]);
		let no_wins = Scratchcard::from(3, [], []);
		let end_card = Scratchcard::from(4, [], []);
		let mut list = CardList {
			card_counts: vec![
				(start_card, 1),
				(two_wins, 42),
				(no_wins, 1),
				(end_card, 10),
			],
		};

		list.scratch_at(1);

		assert_eq!(list.card_counts[0].1, 1);
		assert_eq!(list.card_counts[1].1, 42);
		assert_eq!(list.card_counts[2].1, 43);
		assert_eq!(list.card_counts[3].1, 52);
	}

	#[test]
	fn test_cascade() {
		let two_wins = Scratchcard::from(1, [1, 2], [1, 2]);
		let one_win = Scratchcard::from(2, [3], [3]);
		let end_card = Scratchcard::from(3, [], []);
		let mut list = CardList {
			card_counts: vec![(two_wins, 1), (one_win, 1), (end_card, 1)],
		};

		assert_eq!(list.count_cards(), 3);

		list.scratch();

		assert_eq!(list.card_counts[0].1, 1);
		assert_eq!(list.card_counts[1].1, 2);
		assert_eq!(list.card_counts[2].1, 4);
		assert_eq!(list.count_cards(), 1 + 2 + 4);
	}
}
