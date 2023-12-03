use game::{Game, Hand};
use parse_input::parse_all;
use std::io::{self, Read};

mod game;
mod parse_input;

#[must_use]
fn sum_possible_games(games: &[Game]) -> u32 {
	let target = Hand {
		red: 12,
		green: 13,
		blue: 14,
	};
	games
		.iter()
		.filter(|game| game.is_possible(&target))
		.map(|game| game.id)
		.sum()
}

#[must_use]
fn sum_power(games: &[Game]) -> u32 {
	games.iter().map(game::Game::power).sum()
}

#[cfg(test)]
mod test {
	use super::*;
	const SAMPLE_INPUT: &str = include_str!("../input_sample.txt");

	#[test]
	fn test_sample() {
		let games = parse_all(SAMPLE_INPUT);
		assert_eq!(sum_possible_games(&games), 8);
		assert_eq!(sum_power(&games), 2286);
	}
}

fn main() {
	let mut input = String::new();
	io::stdin()
		.read_to_string(&mut input)
		.expect("Failed to read input");
	let games = parse_all(&input);

	println!("Possible games: {}", sum_possible_games(&games));
	println!("Power: {}", sum_power(&games));
}
