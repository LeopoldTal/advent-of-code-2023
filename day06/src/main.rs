use std::io::{self, Read};

use crate::parse_input::{parse_multi_races, parse_single_race};
use crate::race::Race;

mod parse_input;
mod race;

/// Gets the total number of ways to win all races.
#[must_use]
fn get_nb_permutations(s: &str) -> i64 {
	let races = parse_multi_races(s);
	races.iter().map(Race::nb_winning_holds).product()
}

/// Gets the total number of ways to win a single race.
#[must_use]
fn get_nb_single(s: &str) -> i64 {
	let race = parse_single_race(s);
	race.nb_winning_holds()
}

#[cfg(test)]
mod test {
	use super::*;
	const SAMPLE_INPUT: &str = include_str!("../input_sample.txt");

	#[test]
	fn test_sample() {
		assert_eq!(get_nb_permutations(SAMPLE_INPUT), 288);
		assert_eq!(get_nb_single(SAMPLE_INPUT), 71503);
	}
}

fn main() {
	let mut input = String::new();
	io::stdin()
		.read_to_string(&mut input)
		.expect("Failed to read input");

	println!(
		"Total winning holds, multis: {}",
		get_nb_permutations(&input)
	);
	println!("Total winning holds, single: {}", get_nb_single(&input));
}
