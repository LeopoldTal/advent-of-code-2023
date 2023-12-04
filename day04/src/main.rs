use card_list::CardList;
use scratchcard::Scratchcard;
use std::{
	io::{self, Read},
	time::Instant,
};

use crate::parse_input::parse_cards;

mod card_list;
mod parse_input;
mod scratchcard;

#[must_use]
fn get_total_score(cards: &[Scratchcard]) -> u32 {
	cards.iter().map(Scratchcard::get_score).sum()
}

#[must_use]
fn get_total_cards(cards: &[Scratchcard]) -> u32 {
	let card_counts = cards.iter().cloned().map(|card| (card, 1)).collect();
	let mut list = CardList { card_counts };
	list.scratch();
	list.count_cards()
}

#[cfg(test)]
mod test {
	use super::*;

	const SAMPLE_INPUT: &str = include_str!("../input_sample.txt");

	#[test]
	fn test_sample() {
		let cards = parse_cards(SAMPLE_INPUT);
		assert_eq!(get_total_score(&cards), 13);
		assert_eq!(get_total_cards(&cards), 30);
	}
}

fn main() {
	let mut input = String::new();
	io::stdin()
		.read_to_string(&mut input)
		.expect("Failed to read input");

	let now = Instant::now();

	let cards = parse_cards(&input);
	let part1 = get_total_score(&cards);
	let part2 = get_total_cards(&cards);

	let elapsed = now.elapsed();

	println!("Score: {part1}");
	println!("Cards: {part2}");
	println!("Time: {} µs", elapsed.as_micros());
}
