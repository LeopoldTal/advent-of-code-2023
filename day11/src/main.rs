use std::io::{self, Read};

use parse_input::parse_full;
use starfield::Coords;

mod parse_input;
mod starfield;

/// 2D taxicab distance.
#[must_use]
fn d2((x1, y1): Coords, (x2, y2): Coords) -> usize {
	#[allow(clippy::cast_possible_truncation)]
	let distance = x1.abs_diff(x2) + y1.abs_diff(y2);
	distance
}

/// Lists distances between unique pairs of galaxies in the expanded space.
#[must_use]
fn get_all_distances(points: &[Coords]) -> Vec<usize> {
	let mut distances = Vec::new();
	for (start_index, &start) in points.iter().enumerate() {
		for &end in points.iter().skip(start_index + 1) {
			distances.push(d2(start, end));
		}
	}
	distances
}

#[must_use]
fn get_sum_distances(input: &str, expand_factor: usize) -> usize {
	let starfield = parse_full(input);
	let expanded = starfield.expand(expand_factor);
	let expanded: Vec<Coords> = expanded.into_iter().collect();
	get_all_distances(&expanded).into_iter().sum()
}

#[cfg(test)]
mod test {
	use super::*;
	const SAMPLE_INPUT: &str = include_str!("../input_sample.txt");

	#[test]
	fn test_sample_expand_2() {
		assert_eq!(get_sum_distances(SAMPLE_INPUT, 2), 374);
	}

	#[test]
	fn test_sample_expand_10() {
		assert_eq!(get_sum_distances(SAMPLE_INPUT, 10), 1030);
	}

	#[test]
	fn test_sample_expand_100() {
		assert_eq!(get_sum_distances(SAMPLE_INPUT, 100), 8410);
	}
}

fn main() {
	let mut input = String::new();
	io::stdin()
		.read_to_string(&mut input)
		.expect("Failed to read input");

	println!("Expand by 2: {}", get_sum_distances(&input, 2));
	println!(
		"Expand by a million: {}",
		get_sum_distances(&input, 1_000_000)
	);
}
