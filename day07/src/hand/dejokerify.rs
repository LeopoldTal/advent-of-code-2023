use std::collections::HashSet;

use crate::{card::Card, hand::Hand};

impl Hand {
	/// Lists all possible hands when replacing the joker with any card.
	#[must_use]
	pub fn dejokerify(&self) -> HashSet<Hand> {
		if !self.cards.contains(&Card::Joker) {
			return HashSet::from([self.clone()]);
		}

		// Other cards to replace the joker with.
		let other_cards: HashSet<&Card> =
			self.cards.iter().filter(|&&x| x != Card::Joker).collect();
		let other_cards = if other_cards.is_empty() {
			// Special case: there's nothing but jokers. Replace them with any card.
			HashSet::from([&Card::Jack])
		} else {
			other_cards
		};

		// Recursively replace each joker with each possible card.
		other_cards
			.into_iter()
			.map(|&other_card| replace_first(&self.cards, Card::Joker, other_card))
			.map(Self::from)
			.flat_map(|hand| hand.dejokerify())
			.collect()
	}
}

#[cfg(test)]
mod test_dejokerify {
	use super::*;
	use crate::card::Card::*;

	#[test]
	fn test_no_jokers() {
		let hand = Hand::from(vec![Ace; 5]);
		let expected = HashSet::from([Hand::from(vec![Ace; 5])]);
		assert_eq!(hand.dejokerify(), expected);
	}

	#[test]
	fn test_one_joker() {
		let hand = Hand::from(vec![Ace, Ace, Joker, Ace, Six]);
		let expected = HashSet::from([
			Hand::from(vec![Ace, Ace, Ace, Ace, Six]),
			Hand::from(vec![Ace, Ace, Six, Ace, Six]),
		]);
		assert_eq!(hand.dejokerify(), expected);
	}

	#[test]
	fn test_two_jokers() {
		let hand = Hand::from(vec![Deuce, Deuce, Three, Joker, Joker]);
		let expected = HashSet::from([
			Hand::from(vec![Deuce, Deuce, Three, Deuce, Deuce]),
			Hand::from(vec![Deuce, Deuce, Three, Deuce, Three]),
			Hand::from(vec![Deuce, Deuce, Three, Three, Deuce]),
			Hand::from(vec![Deuce, Deuce, Three, Three, Three]),
		]);
		assert_eq!(hand.dejokerify(), expected);
	}

	#[test]
	fn test_all_jokers() {
		let hand = Hand::from(vec![Joker; 5]);
		let expected = HashSet::from([Hand::from(vec![Jack; 5])]);
		assert_eq!(hand.dejokerify(), expected);
	}
}

/// Clones a vector, with the first instance of `needle` replaced with `replacement`.
#[must_use]
fn replace_first<T>(haystack: &[T], needle: T, replacement: T) -> Vec<T>
where
	T: Copy,
	T: Eq,
{
	let needle_index = haystack.iter().position(|&x| x == needle);
	if let Some(needle_index) = needle_index {
		let (before, after) = haystack.split_at(needle_index);
		let after = &after[1..];
		[before, &[replacement], after].concat()
	} else {
		haystack.to_vec()
	}
}

#[cfg(test)]
mod test_replace_first {
	use super::*;

	#[test]
	fn test_replaces_nothing() {
		let input = vec![1, 2, 3];
		let output = replace_first(&input, 23, 42);
		assert_eq!(output, vec![1, 2, 3]);
	}

	#[test]
	fn test_replaces_at_beginning() {
		let input = vec![1, 2];
		let output = replace_first(&input, 1, 42);
		assert_eq!(output, vec![42, 2]);
	}

	#[test]
	fn test_replaces_at_end() {
		let input = vec![1, 2];
		let output = replace_first(&input, 2, 42);
		assert_eq!(output, vec![1, 42]);
	}

	#[test]
	fn test_replaces_inside() {
		let input = vec![1, 2, 3];
		let output = replace_first(&input, 2, 42);
		assert_eq!(output, vec![1, 42, 3]);
	}

	#[test]
	fn test_replaces_only_first() {
		let input = vec![1, 2, 2, 2, 3];
		let output = replace_first(&input, 2, 42);
		assert_eq!(output, vec![1, 42, 2, 2, 3]);
	}
}
