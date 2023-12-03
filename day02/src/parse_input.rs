use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, space0, u32};
use nom::multi::{many1, separated_list1};
use nom::IResult;

use crate::game::{Game, Hand};

enum Colour {
	Red,
	Green,
	Blue,
}

fn red(input: &str) -> IResult<&str, Colour> {
	let (input, _) = tag("red")(input)?;
	Ok((input, Colour::Red))
}
fn green(input: &str) -> IResult<&str, Colour> {
	let (input, _) = tag("green")(input)?;
	Ok((input, Colour::Green))
}
fn blue(input: &str) -> IResult<&str, Colour> {
	let (input, _) = tag("blue")(input)?;
	Ok((input, Colour::Blue))
}

/// Consumes a colour name
/// FIXME: there's *got* to be a better way!
fn colour(input: &str) -> IResult<&str, Colour> {
	alt((red, green, blue))(input)
}

/// Consumes a single revealed colour.
fn cube_count(input: &str) -> IResult<&str, (u32, Colour)> {
	let (input, amount) = u32(input)?;
	let (input, _) = space0(input)?;
	let (input, colour) = colour(input)?;
	Ok((input, (amount, colour)))
}

/// Consumes a revealed hand.
fn hand(input: &str) -> IResult<&str, Hand> {
	let mut hand = Hand {
		red: 0,
		green: 0,
		blue: 0,
	};
	let (input, reveals) = separated_list1(tag(", "), cube_count)(input)?;
	for (amount, colour) in reveals {
		match colour {
			Colour::Red => {
				hand.red += amount;
			}
			Colour::Green => {
				hand.green += amount;
			}
			Colour::Blue => {
				hand.blue += amount;
			}
		}
	}
	Ok((input, hand))
}

/// Consumes a game's numeric ID.
fn game_id(input: &str) -> IResult<&str, u32> {
	let (input, _) = tag("Game")(input)?;
	let (input, _) = space0(input)?;
	let (input, id) = u32(input)?;
	let (input, _) = char(':')(input)?;
	let (input, _) = space0(input)?;
	Ok((input, id))
}

/// Consumes the hands making up a game.
fn hand_list(input: &str) -> IResult<&str, Vec<Hand>> {
	separated_list1(tag("; "), hand)(input)
}

/// Consumes a complete game, including terminating newline.
fn game(input: &str) -> IResult<&str, Game> {
	let (input, id) = game_id(input)?;
	let (input, hands) = hand_list(input)?;
	let (input, _) = char('\n')(input)?;
	let game = Game { id, hands };
	Ok((input, game))
}

/// Consumes a list of games.
fn game_list(input: &str) -> IResult<&str, Vec<Game>> {
	many1(game)(input)
}

/// Parses the full list of games.
/// # Panics
/// On any parse error.
#[must_use]
pub fn parse_all(input: &str) -> Vec<Game> {
	let (remaining, games) =
		game_list(input).unwrap_or_else(|error| panic!("Parse error: {error}"));
	assert_eq!(
		remaining.trim(),
		"",
		"Unexpected trailing after input: {remaining}"
	);
	games
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_hand() {
		let input = "3 red, 5 blue, 42 green";
		let (_, hand) = hand(input).unwrap();
		let expected = Hand {
			red: 3,
			green: 42,
			blue: 5,
		};
		assert_eq!(hand, expected);
	}

	#[test]
	fn test_game() {
		let input = "Game 23: 1 red; 1 green, 2 blue\n";
		let (_, game) = game(input).unwrap();
		let hand1 = Hand {
			red: 1,
			green: 0,
			blue: 0,
		};
		let hand2 = Hand {
			red: 0,
			green: 1,
			blue: 2,
		};
		let expected = Game {
			id: 23,
			hands: vec![hand1, hand2],
		};
		assert_eq!(game, expected);
	}

	#[test]
	fn test_full() {
		let input = "Game 1: 1 blue\nGame 2: 3 green\n";
		let parsed = parse_all(input);
		let expected = vec![
			Game {
				id: 1,
				hands: vec![Hand {
					red: 0,
					green: 0,
					blue: 1,
				}],
			},
			Game {
				id: 2,
				hands: vec![Hand {
					red: 0,
					green: 3,
					blue: 0,
				}],
			},
		];
		assert_eq!(parsed, expected);
	}
}
