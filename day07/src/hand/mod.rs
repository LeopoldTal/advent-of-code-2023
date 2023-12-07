use crate::card::Card;
use hand_type::HandType;
use std::{cmp::Ordering, collections::HashMap};

mod dejokerify;
mod hand_type;

/// A hand of 5 cards.
#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct Hand {
	pub cards: Vec<Card>,
}

/// A hand with an amount bid on it.
#[derive(Debug, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Bid {
	pub hand: Hand,
	pub amount: usize,
}

impl Hand {
	/// Builds a hand from a vec of exactly 5 cards.
	/// # Panics
	/// On any other hand size.
	#[must_use]
	pub fn from(cards: Vec<Card>) -> Hand {
		assert_eq!(cards.len(), 5, "A hand of poker must have 5 cards.");
		Hand { cards }
	}

	/// Computes what type of hand this is, e.g. two pair, full house, etc.
	#[must_use]
	pub fn get_hand_type(&self) -> HandType {
		self.dejokerify()
			.into_iter()
			.map(|hand| hand.get_base_hand_type())
			.max()
			.expect("Failed to generate any hands. This can never happen")
	}

	/// Computes hand type assuming there are no jokers in the hand.
	#[must_use]
	fn get_base_hand_type(&self) -> HandType {
		let counts = get_nb_cards_by_count(&self.cards);
		if get(&counts, 5) > 0 {
			return HandType::FiveOfKind;
		}
		if get(&counts, 4) > 0 {
			return HandType::FourOfKind;
		}
		if get(&counts, 3) > 0 {
			if get(&counts, 2) > 0 {
				return HandType::FullHouse;
			}
			return HandType::ThreeOfKind;
		}
		match get(&counts, 2) {
			2 => HandType::TwoPair,
			1 => HandType::Pair,
			_ => HandType::HighCard,
		}
	}
}

/// Counts how many cards appear each possible number of times.
#[must_use]
fn get_nb_cards_by_count(cards: &[Card]) -> HashMap<u16, u16> {
	let card_counts = get_counts_by_card(cards);
	card_counts
		.into_values()
		.fold(HashMap::new(), |mut grouped, card_count| {
			*grouped.entry(card_count).or_insert(0) += 1;
			grouped
		})
}

/// Counts occurrences of each card.
#[must_use]
fn get_counts_by_card(cards: &[Card]) -> HashMap<Card, u16> {
	cards.iter().fold(HashMap::new(), |mut counts, &card| {
		*counts.entry(card).or_insert(0) += 1;
		counts
	})
}

/// Sugar for get-or-default.
#[must_use]
fn get(counts: &HashMap<u16, u16>, card_count: u16) -> u16 {
	*counts.get(&card_count).unwrap_or(&0)
}

#[cfg(test)]
mod test_from {
	use super::*;
	use crate::card::Card::*;

	#[test]
	fn test_preserves_input() {
		let input_cards = vec![Queen, Queen, Three, King, Ace];
		let hand = Hand::from(input_cards.clone());
		assert_eq!(hand.cards, input_cards);
	}

	#[test]
	#[should_panic]
	fn test_too_many_cards() {
		let input_cards = vec![Ace; 6];
		let _ = Hand::from(input_cards);
	}

	#[test]
	#[should_panic]
	fn test_too_few_cards() {
		let input_cards = vec![Ace; 4];
		let _ = Hand::from(input_cards);
	}
}

#[cfg(test)]
mod test_type {
	use super::*;
	use crate::card::Card::*;
	use hand_type::HandType::*;

	#[test]
	fn test_high_card() {
		let hand = Hand::from(vec![Deuce, Three, Six, Five, Four]);
		assert_eq!(hand.get_hand_type(), HighCard);
	}

	#[test]
	fn test_one_pair() {
		let hand = Hand::from(vec![Deuce, Three, Six, Five, Three]);
		assert_eq!(hand.get_hand_type(), Pair);
	}

	#[test]
	fn test_two_pair() {
		let hand = Hand::from(vec![Deuce, Three, Six, Deuce, Three]);
		assert_eq!(hand.get_hand_type(), TwoPair);
	}

	#[test]
	fn test_three_of_kind() {
		let hand = Hand::from(vec![Deuce, Three, Six, Three, Three]);
		assert_eq!(hand.get_hand_type(), ThreeOfKind);
	}

	#[test]
	fn test_full_house() {
		let hand = Hand::from(vec![Six, Three, Six, Three, Three]);
		assert_eq!(hand.get_hand_type(), FullHouse);
	}

	#[test]
	fn test_four_of_kind() {
		let hand = Hand::from(vec![Six, Three, Six, Six, Six]);
		assert_eq!(hand.get_hand_type(), FourOfKind);
	}

	#[test]
	fn test_five_of_kind() {
		let hand = Hand::from(vec![Six; 5]);
		assert_eq!(hand.get_hand_type(), FiveOfKind);
	}

	#[test]
	fn test_pair_with_joker() {
		let hand = Hand::from(vec![Ace, King, Queen, Deuce, Joker]);
		assert_eq!(hand.get_hand_type(), Pair);
	}

	#[test]
	fn test_pair_of_jokers() {
		let hand = Hand::from(vec![Ace, King, Queen, Joker, Joker]);
		assert_eq!(hand.get_hand_type(), ThreeOfKind);
	}

	#[test]
	fn test_three_of_kind_with_joker() {
		let hand = Hand::from(vec![Ace, Ace, Joker, Three, Deuce]);
		assert_eq!(hand.get_hand_type(), ThreeOfKind);
	}

	#[test]
	fn test_full_house_with_joker() {
		let hand = Hand::from(vec![Ace, Ace, Joker, Deuce, Deuce]);
		assert_eq!(hand.get_hand_type(), FullHouse);
	}

	#[test]
	fn test_four_of_kind_with_joker() {
		let hand = Hand::from(vec![Ace, Ace, Joker, Three, Ace]);
		assert_eq!(hand.get_hand_type(), FourOfKind);
	}

	#[test]
	fn test_five_of_kind_with_one_joker() {
		let hand = Hand::from(vec![Six, Six, Six, Six, Joker]);
		assert_eq!(hand.get_hand_type(), FiveOfKind);
	}

	#[test]
	fn test_five_of_kind_with_two_jokers() {
		let hand = Hand::from(vec![Joker, Six, Six, Six, Joker]);
		assert_eq!(hand.get_hand_type(), FiveOfKind);
	}

	#[test]
	fn test_five_jokers() {
		let hand = Hand::from(vec![Joker; 5]);
		assert_eq!(hand.get_hand_type(), FiveOfKind);
	}
}

impl PartialOrd for Hand {
	#[must_use]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for Hand {
	#[must_use]
	fn cmp(&self, other: &Self) -> Ordering {
		let my_type = self.get_hand_type();
		let other_type = other.get_hand_type();
		if my_type < other_type {
			return Ordering::Less;
		}
		if my_type > other_type {
			return Ordering::Greater;
		}
		self.cards.cmp(&other.cards)
	}
}

#[cfg(test)]
mod test_compare {
	use super::*;
	use crate::card::Card::*;

	#[test]
	fn test_five_of_kind_beats_four() {
		let winner = Hand::from(vec![King; 5]);
		let loser = Hand::from(vec![Ace, Ace, Ace, Ace, King]);
		assert!(winner > loser);
	}

	#[test]
	fn test_three_of_kind_beats_two_pair() {
		let winner = Hand::from(vec![Jack, Jack, Jack, Three, Deuce]);
		let loser = Hand::from(vec![King, King, Ten, Ten, Ace]);
		assert!(winner > loser);
	}

	#[test]
	fn test_compares_on_first_card() {
		let winner = Hand::from(vec![Six, Six, Six, Six, Three]);
		let loser = Hand::from(vec![Three, Six, Six, Six, Six]);
		assert!(winner > loser);
	}

	#[test]
	fn test_compares_on_second_card() {
		let winner = Hand::from(vec![Six, Seven, Six, Six, Six]);
		let loser = Hand::from(vec![Six, Six, Seven, Six, Six]);
		assert!(winner > loser);
	}

	#[test]
	fn test_bid_compares_like_hand() {
		let winner = Bid {
			hand: Hand::from(vec![Ace; 5]),
			amount: 1,
		};
		let loser = Bid {
			hand: Hand::from(vec![Six, Six, Seven, Six, Six]),
			amount: 1000,
		};
		assert!(winner > loser);
	}
}
