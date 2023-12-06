use std::io::{self, Read};

use almanac::Almanac;
use parse_input::parse_full;

mod almanac;
mod parse_input;

#[must_use]
fn get_all(s: &str, as_ranges: bool) -> Almanac {
	let (mut almanac, converter) = parse_full(s, as_ranges);
	converter.convert_all(&mut almanac);
	almanac
}

#[must_use]
fn min_location(almanac: &Almanac) -> u64 {
	almanac["location"]
		.iter()
		.map(|&range| range.0)
		.min()
		.expect("No almanacs found")
}

#[cfg(test)]
mod test {
	use std::collections::HashSet;

	use super::*;
	const SAMPLE_INPUT: &str = include_str!("../input_sample.txt");

	#[test]
	fn test_sample_part1() {
		let almanac = get_all(SAMPLE_INPUT, false);
		assert_eq!(
			almanac["seed"],
			HashSet::from([(79, 80), (14, 15), (55, 56), (13, 14)])
		);
		assert_eq!(
			almanac["soil"],
			HashSet::from([(81, 82), (14, 15), (57, 58), (13, 14)])
		);
		assert_eq!(
			almanac["location"],
			HashSet::from([(82, 83), (43, 44), (86, 87), (35, 36)])
		);

		assert_eq!(min_location(&almanac), 35);
	}

	#[test]
	fn test_sample_part2() {
		let almanac: std::collections::HashMap<String, HashSet<(u64, u64)>> =
			get_all(SAMPLE_INPUT, true);
		assert_eq!(almanac["seed"], HashSet::from([(79, 93), (55, 68)]));

		assert_eq!(min_location(&almanac), 46);
	}
}

fn main() {
	let mut input = String::new();
	io::stdin()
		.read_to_string(&mut input)
		.expect("Failed to read input");

	let almanac_points = get_all(&input, false);
	println!("Part 1: {}", min_location(&almanac_points));

	let almanac_ranges = get_all(&input, true);
	println!("Part 2: {}", min_location(&almanac_ranges));
}
