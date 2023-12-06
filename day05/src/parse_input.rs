use nom::{
	bytes::complete::{tag, take_until},
	character::complete::{multispace0, multispace1, space0, space1, u64},
	multi::{many1, separated_list1},
	IResult,
};

use crate::almanac::{Almanac, ConversionMap, ConversionRange, Converter};

/// Consumes the name of a value type.
fn almanac_title(input: &str) -> IResult<&str, &str> {
	let (input, name) = take_until("s:")(input)?;
	let (input, _) = tag("s:")(input)?;
	let (input, _) = space0(input)?;
	Ok((input, name))
}

/// Consumes a list of integers.
fn num_list(input: &str) -> IResult<&str, Vec<u64>> {
	let (input, _) = space0(input)?;
	let (input, numbers) = separated_list1(space1, u64)(input)?;
	let (input, _) = space0(input)?;
	Ok((input, numbers))
}

/// Consumes a named list of values, into 1-long ranges.
fn almanac_points(input: &str) -> IResult<&str, Almanac> {
	let (input, name) = almanac_title(input)?;
	let (input, numbers) = num_list(input)?;
	let ranges = numbers.into_iter().map(|x| (x, x + 1)).collect();
	let almanac = Almanac::from([(name.to_string(), ranges)]);
	Ok((input, almanac))
}

/// Consumes a single range of values.
fn value_range(input: &str) -> IResult<&str, (u64, u64)> {
	let (input, _) = space0(input)?;
	let (input, start) = u64(input)?;
	let (input, _) = space1(input)?;
	let (input, length) = u64(input)?;
	let (input, _) = space0(input)?;
	Ok((input, (start, start + length)))
}

/// Consumes a named list of values pairs, into (start, length) ranges.
fn almanac_ranges(input: &str) -> IResult<&str, Almanac> {
	let (input, name) = almanac_title(input)?;
	let (input, ranges) = many1(value_range)(input)?;
	let ranges = ranges.into_iter().collect();
	let almanac = Almanac::from([(name.to_string(), ranges)]);
	Ok((input, almanac))
}

/// Consumes a single range for conversion.
fn conversion_range(input: &str) -> IResult<&str, ConversionRange> {
	let (input, _) = space0(input)?;
	let (input, to_start) = u64(input)?;
	let (input, _) = space1(input)?;
	let (input, from_start) = u64(input)?;
	let (input, _) = space1(input)?;
	let (input, length) = u64(input)?;
	let (input, _) = multispace1(input)?;
	Ok((
		input,
		ConversionRange {
			from_start,
			length,
			to_start,
		},
	))
}

/// Consumes the header of a single conversion map.
fn conversion_map_title(input: &str) -> IResult<&str, (String, String)> {
	let (input, _) = multispace0(input)?;
	let (input, name_from) = take_until("-to-")(input)?;
	let (input, _) = tag("-to-")(input)?;
	let (input, name_to) = take_until(" map:")(input)?;
	let (input, _) = take_until("\n")(input)?;
	let (input, _) = multispace0(input)?;
	Ok((input, (name_from.to_string(), name_to.to_string())))
}

/// Consumes one full conversion map.
fn conversion_map(input: &str) -> IResult<&str, ConversionMap> {
	let (input, (name_from, name_to)) = conversion_map_title(input)?;
	let (input, mut ranges) = many1(conversion_range)(input)?;
	ranges.sort_by_key(|range| range.from_start);
	Ok((
		input,
		ConversionMap {
			name_from,
			name_to,
			ranges,
		},
	))
}

/// Consumes the whole input.
fn full(input: &str, as_ranges: bool) -> IResult<&str, (Almanac, Converter)> {
	let parser = if as_ranges {
		almanac_ranges
	} else {
		almanac_points
	};
	let (input, almanac) = parser(input)?;
	let (input, maps) = many1(conversion_map)(input)?;
	Ok((input, (almanac, Converter { maps })))
}

/// Parses the whole input.
/// # Panics
/// On any parse error.
#[must_use]
pub fn parse_full(input: &str, as_ranges: bool) -> (Almanac, Converter) {
	let (_, parsed) = full(input, as_ranges).expect("Parse error");
	parsed
}

#[cfg(test)]
mod test {
	use std::collections::HashSet;

	use super::*;

	#[test]
	fn test_num_list() {
		let input = "1 2 42 etc.";
		let expected = Ok(("etc.", vec![1, 2, 42]));
		assert_eq!(num_list(input), expected);
	}

	#[test]
	fn test_almanac_points() {
		let input = "seeds: 23 1729";
		let (_, parsed) = almanac_points(input).expect("Parse error");
		assert_eq!(parsed["seed"], HashSet::from([(23, 24), (1729, 1730)]));
	}

	#[test]
	fn test_almanac_ranges() {
		let input = "seeds: 23 1729 10 2";
		let (_, parsed) = almanac_ranges(input).expect("Parse error");
		assert_eq!(parsed["seed"], HashSet::from([(23, 23 + 1729), (10, 12)]));
	}

	#[test]
	fn test_conversion_range() {
		let input = "50 98 2\n";
		let (_, parsed) = conversion_range(input).expect("Parse error");
		assert_eq!(
			parsed,
			ConversionRange {
				to_start: 50,
				from_start: 98,
				length: 2
			}
		);
	}

	#[test]
	fn test_conversion_map_title() {
		let input = "water-to-light map:\n";
		let (_, parsed) = conversion_map_title(input).expect("Parse error");
		assert_eq!(parsed, (String::from("water"), String::from("light")));
	}

	#[test]
	fn test_conversion_map() {
		let input = "foo-to-bar map:\n20 0 5\n10 20 3\n";
		let (_, parsed) = conversion_map(input).expect("Parse error");
		let expected = ConversionMap {
			name_from: String::from("foo"),
			name_to: String::from("bar"),
			ranges: vec![
				ConversionRange {
					to_start: 20,
					from_start: 0,
					length: 5,
				},
				ConversionRange {
					to_start: 10,
					from_start: 20,
					length: 3,
				},
			],
		};
		assert_eq!(parsed, expected);
	}

	#[test]
	fn test_conversion_map_sorted() {
		let swapped_ranges = "foo-to-bar map:\n10 20 3\n20 0 5\n";
		let (_, parsed) = conversion_map(swapped_ranges).expect("Parse error");
		let expected = ConversionMap {
			name_from: String::from("foo"),
			name_to: String::from("bar"),
			ranges: vec![
				ConversionRange {
					to_start: 20,
					from_start: 0,
					length: 5,
				},
				ConversionRange {
					to_start: 10,
					from_start: 20,
					length: 3,
				},
			],
		};
		assert_eq!(parsed, expected);
	}

	#[test]
	fn test_full_as_points() {
		let foos = "foos: 1 2\n";
		let foo_to_bar = "foo-to-bar map:\n3 10 1\n";
		let bar_to_baz: &str = "bar-to-baz map:\n10 100 10\n4 20 2\n";

		let (_, almanac) = almanac_points(foos).unwrap();
		let (_, map1) = conversion_map(foo_to_bar).unwrap();
		let (_, map2) = conversion_map(bar_to_baz).unwrap();
		let converter = Converter {
			maps: vec![map1, map2],
		};

		let input = [foos, foo_to_bar, bar_to_baz].join("\n");
		let (_, parsed) = full(&input, false).expect("Parse error");

		assert_eq!(parsed, (almanac, converter));
	}

	#[test]
	fn test_full_as_ranges() {
		let foos = "foos: 1 2\n";
		let foo_to_bar = "foo-to-bar map:\n3 10 1\n";

		let (_, almanac) = almanac_ranges(foos).unwrap();
		let (_, map) = conversion_map(foo_to_bar).unwrap();
		let converter = Converter { maps: vec![map] };

		let input = [foos, foo_to_bar].join("\n");
		let (_, parsed) = full(&input, true).expect("Parse error");

		assert_eq!(parsed, (almanac, converter));
	}
}
