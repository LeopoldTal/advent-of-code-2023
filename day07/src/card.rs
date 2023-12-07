/// Value of a playing card.
#[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Card {
	Deuce = 2,
	Three = 3,
	Four = 4,
	Five = 5,
	Six = 6,
	Seven = 7,
	Eight = 8,
	Nine = 9,
	Ten = 10,
	Jack = 11,
	Queen = 12,
	King = 13,
	Ace = 14,
	Joker = 0,
}

#[cfg(test)]
mod test_card {
	use super::Card::*;

	#[test]
	fn test_ordering() {
		let mut cards = vec![
			Ace, Deuce, Eight, Five, Four, Jack, Joker, King, Nine, Queen, Seven, Six, Ten, Three,
		];
		let sorted_cards = vec![
			Joker, Deuce, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King, Ace,
		];
		cards.sort();
		assert_eq!(cards, sorted_cards);
	}
}
