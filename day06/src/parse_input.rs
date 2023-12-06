use std::iter::zip;

use crate::race::Race;

/// Reads a prefixed number containing white splaces.
#[must_use]
fn single_num(input: &str) -> i64 {
	let num_parts: Vec<&str> = input.split_whitespace().skip(1).collect();
	num_parts.join("").parse().expect("Not a number")
}

/// Reads a prefixed, whitespace-separated list of numbers.
#[must_use]
fn num_list(input: &str) -> Vec<i64> {
	input
		.split_whitespace()
		.skip(1)
		.map(|n| n.parse().expect("Not a number"))
		.collect()
}

/// Reads all boat races.
#[must_use]
pub fn parse_multi_races(input: &str) -> Vec<Race> {
	let mut lines = input.lines().take(2);

	let time_line = lines.next().expect("No times");
	let times = num_list(time_line);

	let distance_line = lines.next().expect("No distances");
	let distances = num_list(distance_line);

	zip(times, distances)
		.map(|(time_limit, distance_threshold)| Race {
			time_limit,
			distance_threshold,
		})
		.collect()
}

/// Reads one boat race, ignoring spaces.
#[must_use]
pub fn parse_single_race(input: &str) -> Race {
	let mut lines = input.lines().take(2);

	let time_line = lines.next().expect("No times");
	let time_limit = single_num(time_line);

	let distance_line = lines.next().expect("No distances");
	let distance_threshold = single_num(distance_line);

	Race {
		time_limit,
		distance_threshold,
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_parse_multi_races() {
		let input = "Time: 1 2\nDistance: 3 4\n";
		let parsed = parse_multi_races(input);
		let expected = vec![
			Race {
				time_limit: 1,
				distance_threshold: 3,
			},
			Race {
				time_limit: 2,
				distance_threshold: 4,
			},
		];
		assert_eq!(parsed, expected);
	}

	#[test]
	fn test_parse_single_race() {
		let input = "Time: 1 2\nDistance: 3 4\n";
		let parsed = parse_single_race(input);
		let expected = Race {
			time_limit: 12,
			distance_threshold: 34,
		};
		assert_eq!(parsed, expected);
	}
}
