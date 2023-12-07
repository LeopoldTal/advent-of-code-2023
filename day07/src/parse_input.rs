use nom::{
	character::complete::{multispace1, one_of, space1, u32},
	multi::many1,
	IResult,
};

use crate::{
	card::Card::{
		self, Ace, Deuce, Eight, Five, Four, Jack, Joker, King, Nine, Queen, Seven, Six, Ten, Three,
	},
	hand::Hand,
	Bid,
};

struct Parser {
	pub jacks_are_jokers: bool,
}

impl Default for Parser {
	fn default() -> Self {
		Self::new(false)
	}
}

impl Parser {
	fn new(jacks_are_jokers: bool) -> Self {
		Parser { jacks_are_jokers }
	}

	/// Consumes one card.
	fn card<'a>(&'a self, input: &'a str) -> IResult<&str, Card> {
		let (input, card_rank) = one_of("AKQJT98765432")(input)?;
		let card = match card_rank {
			'A' => Ace,
			'K' => King,
			'Q' => Queen,
			'J' => {
				if self.jacks_are_jokers {
					Joker
				} else {
					Jack
				}
			}
			'T' => Ten,
			'9' => Nine,
			'8' => Eight,
			'7' => Seven,
			'6' => Six,
			'5' => Five,
			'4' => Four,
			'3' => Three,
			'2' => Deuce,
			_ => unreachable!(),
		};
		Ok((input, card))
	}

	/// Consumes a hand of cards.
	fn hand<'a>(&'a self, input: &'a str) -> IResult<&str, Hand> {
		let (input, cards) = many1(|s| self.card(s))(input)?;
		Ok((input, Hand::from(cards)))
	}

	/// Consumes a hand and bid amount.
	fn bid<'a>(&'a self, input: &'a str) -> IResult<&str, Bid> {
		let (input, hand) = self.hand(input)?;
		let (input, _) = space1(input)?;
		let (input, amount) = u32(input)?;
		let (input, _) = multispace1(input)?;
		let bid = Bid {
			hand,
			amount: amount as usize,
		};
		Ok((input, bid))
	}
	/// Parses the whole input.
	/// # Panics
	/// On any parse error.
	#[must_use]
	pub fn full(&self, input: &str) -> Vec<Bid> {
		let (_, bids) = many1(|s| self.bid(s))(input).expect("Parse error");
		bids
	}
}

pub fn parse_full(input: &str, jacks_are_jokers: bool) -> Vec<Bid> {
	Parser::new(jacks_are_jokers).full(input)
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_card() {
		let input = "T";
		let (_, card) = Parser::default().card(input).unwrap();
		assert_eq!(card, Ten);
	}

	#[test]
	fn test_card_jack() {
		let input = "J";
		let (_, card) = Parser::new(false).card(input).unwrap();
		assert_eq!(card, Jack);
	}

	#[test]
	fn test_card_joker() {
		let input = "J";
		let (_, card) = Parser::new(true).card(input).unwrap();
		assert_eq!(card, Joker);
	}

	#[test]
	fn test_hand() {
		let input = "23456";
		let (_, hand) = Parser::default().hand(input).unwrap();
		assert_eq!(hand, Hand::from(vec![Deuce, Three, Four, Five, Six]));
	}

	#[test]
	fn test_bid() {
		let input_bid = "23456 23457\n";

		let input_hand = "23456";
		let (_, hand) = Parser::default().hand(input_hand).unwrap();
		let expected = Bid {
			hand,
			amount: 23457,
		};

		let (_, bid) = Parser::default().bid(input_bid).unwrap();

		assert_eq!(bid, expected);
	}

	#[test]
	fn test_full() {
		let input_bid1 = "AA2KK 23\n";
		let input_bid2 = "4242Q 42\n";
		let input_list = vec![input_bid1, input_bid2].join("");

		let (_, bid1) = Parser::default().bid(input_bid1).unwrap();
		let (_, bid2) = Parser::default().bid(input_bid2).unwrap();

		let bids = Parser::default().full(&input_list);

		assert_eq!(bids, vec![bid1, bid2]);
	}
}
