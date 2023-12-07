use parse_input::parse_full;

use crate::hand::Bid;
use std::io::{self, Read};

mod card;
mod hand;
mod parse_input;

/// Sums total winnings of a list of hands.
#[must_use]
fn get_winnings(bids: &[Bid]) -> usize {
	let mut bids: Vec<&Bid> = bids.iter().collect();
	bids.sort();
	bids.iter()
		.enumerate()
		.map(|(rank, bid)| (rank + 1) * bid.amount)
		.sum()
}

#[must_use]
fn part1(input: &str) -> usize {
	let bids = parse_full(input, false);
	get_winnings(&bids)
}

#[must_use]
fn part2(input: &str) -> usize {
	let bids = parse_full(input, true);
	get_winnings(&bids)
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::{card::Card::*, hand::Hand};
	const SAMPLE_INPUT: &str = include_str!("../input_sample.txt");

	#[test]
	fn test_winnings() {
		let winner = Bid {
			hand: Hand::from(vec![Deuce; 5]),
			amount: 100,
		};
		let middle = Bid {
			hand: Hand::from(vec![Ace, Ace, Ace, Six, Six]),
			amount: 20,
		};
		let loser = Bid {
			hand: Hand::from(vec![Three, Four, Five, Six, Seven]),
			amount: 5,
		};
		let bids = vec![middle, winner, loser];
		assert_eq!(get_winnings(&bids), 3 * 100 + 2 * 20 + 1 * 5);
	}

	#[test]
	fn test_sample() {
		assert_eq!(part1(SAMPLE_INPUT), 6440);
		assert_eq!(part2(SAMPLE_INPUT), 5905);
	}
}

fn main() {
	let mut input = String::new();
	io::stdin()
		.read_to_string(&mut input)
		.expect("Failed to read input");

	println!("Part 1 — no jokers: {}", part1(&input));
	println!("Part 2 — jokers: {}", part2(&input));
}
