use std::io::{self, Read};

use almanac::Almanac;
use parse_input::parse_full;

mod almanac;
mod parse_input;

#[must_use]
fn get_all(s: &str) -> Vec<Almanac> {
	let (mut almanacs, converter) = parse_full(s);
	converter.map_all(&mut almanacs);
	almanacs
}

#[must_use]
fn min_location(almanacs: &[Almanac]) -> u64 {
	almanacs
		.iter()
		.map(|almanac| almanac["location"])
		.min()
		.expect("No almanacs found")
}

#[cfg(test)]
mod test {
	use super::*;
	const SAMPLE_INPUT: &str = include_str!("../input_sample.txt");

	#[test]
	fn test_sample() {
		let almanacs = get_all(SAMPLE_INPUT);
		assert_eq!(almanacs.len(), 4);
		assert_eq!(almanacs[0]["seed"], 79);
		assert_eq!(almanacs[0]["soil"], 81);
		assert_eq!(almanacs[0]["location"], 82);
		assert_eq!(min_location(&almanacs), 35);
	}
}

fn main() {
	let mut input = String::new();
	io::stdin()
		.read_to_string(&mut input)
		.expect("Failed to read input");

	let almanacs = get_all(&input);
	println!("Min location: {}", min_location(&almanacs));
}
