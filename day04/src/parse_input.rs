use crate::scratchcard::Scratchcard;
use nom::bytes::complete::tag;
use nom::character::complete::{char, space0, space1, u32};
use nom::multi::separated_list1;
use nom::IResult;
use std::collections::HashSet;

/// Consumes a card's numeric ID.
fn card_id(input: &str) -> IResult<&str, u32> {
	let (input, _) = tag("Card")(input)?;
	let (input, _) = space0(input)?;
	let (input, id) = u32(input)?;
	let (input, _) = char(':')(input)?;
	let (input, _) = space0(input)?;
	Ok((input, id))
}

/// Consumes a set of integers.
/// # Panics
/// On any parse error.
fn num_list(input: &str) -> IResult<&str, HashSet<u32>> {
	let (input, numbers) = separated_list1(space1, u32)(input)?;
	Ok((input, HashSet::from_iter(numbers)))
}

/// Consumes a full card.
/// # Panics
/// On any parse error.
fn card(input: &str) -> IResult<&str, Scratchcard> {
	let (input, id) = card_id(input)?;
	let (input, winning_numbers) = num_list(input)?;
	let (input, _) = space0(input)?;
	let (input, _) = tag("|")(input)?;
	let (input, _) = space0(input)?;
	let (input, pulled_numbers) = num_list(input)?;
	Ok((
		input,
		Scratchcard {
			id,
			winning_numbers,
			pulled_numbers,
		},
	))
}

/// Parses a card.
/// # Panics
/// On any parse error.
#[must_use]
fn parse_card(input: &str) -> Scratchcard {
	let (_, card) = card(input).unwrap_or_else(|error| panic!("Parse error: {error}"));
	card
}

/// Parses a list of cards.
/// # Panics
/// On any parse error.
#[must_use]
pub fn parse_cards(input: &str) -> Vec<Scratchcard> {
	input.lines().map(parse_card).collect()
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_parse_card() {
		let input = "Card 42: 1 23 128 | 23 1729 17 2\n";
		let expected = Scratchcard {
			id: 42,
			winning_numbers: HashSet::from([1, 23, 128]),
			pulled_numbers: HashSet::from([2, 17, 23, 1729]),
		};
		assert_eq!(parse_card(input), expected);
	}

	#[test]
	fn test_parse_cards() {
		let input = "Card 1: 1 | 2 3\nCard 2: 4 5 | 6\n";
		let card1 = Scratchcard {
			id: 1,
			winning_numbers: HashSet::from([1]),
			pulled_numbers: HashSet::from([2, 3]),
		};
		let card2 = Scratchcard {
			id: 2,
			winning_numbers: HashSet::from([4, 5]),
			pulled_numbers: HashSet::from([6]),
		};
		assert_eq!(parse_cards(input), vec![card1, card2]);
	}
}
