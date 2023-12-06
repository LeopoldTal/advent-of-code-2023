use std::collections::{HashMap, HashSet};

/// A range of values: (start inclusive, end exclusive).
pub type ValueRange = (u64, u64);

/// All ranges of seeds values with all their conversions.
pub type Almanac = HashMap<String, HashSet<ValueRange>>;

/// A range to convert a value.
#[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ConversionRange {
	pub from_start: u64,
	pub length: u64,
	pub to_start: u64,
}

impl ConversionRange {
	/// Converts a value that falls within the range or on its end.
	fn convert(&self, x: u64) -> u64 {
		assert!(x >= self.from_start && x <= self.from_start + self.length);
		x + self.to_start - self.from_start
	}
}

#[cfg(test)]
mod test_range {
	use super::*;

	#[test]
	fn test_convert() {
		let range = ConversionRange {
			from_start: 10,
			to_start: 100,
			length: 2,
		};
		assert_eq!(range.convert(10), 100);
		assert_eq!(range.convert(11), 101);
	}
}

/// A group of ranges to convert one kind of value to another.
/// Ranges are sorted and assumed not to overlap.
#[derive(Debug, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ConversionMap {
	pub name_from: String,
	pub name_to: String,
	pub ranges: Vec<ConversionRange>,
}

impl ConversionMap {
	/// Converts a range of values through the map. Unmatched values are unchanged.
	#[must_use]
	fn convert_range(&self, (start, end): ValueRange) -> HashSet<ValueRange> {
		let mut converted = HashSet::new();
		let mut lowest = start;
		for range in &self.ranges {
			if end <= range.from_start {
				// We're past the range to convert, so add the leave and stop now.
				if lowest < end {
					converted.insert((lowest, end));
				}
				return converted;
			}
			if lowest >= range.from_start + range.length {
				// No overlap, so try the next range.
				continue;
			}

			// Add the leave before the range.
			if lowest < range.from_start {
				converted.insert((lowest, range.from_start));
				lowest = range.from_start;
			}

			// Add the converted overlap.
			let overlap_end = end.min(range.from_start + range.length);
			let converted_start = range.convert(lowest);
			let converted_end = range.convert(overlap_end);
			converted.insert((converted_start, converted_end));

			lowest = overlap_end;
		}

		// Add leave after the last range.
		if lowest < end {
			converted.insert((lowest, end));
		}
		converted
	}

	/// Converts all ranges.
	#[must_use]
	fn convert(&self, value_ranges: &HashSet<ValueRange>) -> HashSet<ValueRange> {
		value_ranges
			.iter()
			.flat_map(|value_range| self.convert_range(*value_range))
			.collect()
	}
}

#[cfg(test)]
mod test_map {
	use super::*;

	fn single_range_map() -> ConversionMap {
		let range: ConversionRange = ConversionRange {
			from_start: 10,
			to_start: 1000,
			length: 10,
		};
		ConversionMap {
			name_from: String::from("in"),
			name_to: String::from("out"),
			ranges: vec![range],
		}
	}

	fn spaced_ranges_map() -> ConversionMap {
		let low_range: ConversionRange = ConversionRange {
			from_start: 10,
			to_start: 1000,
			length: 10,
		};
		let high_range: ConversionRange = ConversionRange {
			from_start: 30,
			to_start: 100,
			length: 10,
		};
		ConversionMap {
			name_from: String::from("in"),
			name_to: String::from("out"),
			ranges: vec![low_range, high_range],
		}
	}

	fn contiguous_ranges_map() -> ConversionMap {
		let low_range: ConversionRange = ConversionRange {
			from_start: 10,
			to_start: 1000,
			length: 10,
		};
		let high_range: ConversionRange = ConversionRange {
			from_start: 20,
			to_start: 100,
			length: 10,
		};
		ConversionMap {
			name_from: String::from("in"),
			name_to: String::from("out"),
			ranges: vec![low_range, high_range],
		}
	}

	#[test]
	fn test_miss_low() {
		let map = single_range_map();
		assert_eq!(map.convert_range((1, 2)), HashSet::from([(1, 2)]));
		assert_eq!(map.convert_range((0, 10)), HashSet::from([(0, 10)]));
	}

	#[test]
	fn test_miss_high() {
		let map = single_range_map();
		assert_eq!(map.convert_range((90, 91)), HashSet::from([(90, 91)]));
		assert_eq!(map.convert_range((20, 30)), HashSet::from([(20, 30)]));
	}

	#[test]
	fn test_aligned_low() {
		let map = single_range_map();
		assert_eq!(map.convert_range((10, 12)), HashSet::from([(1000, 1002)]));
	}

	#[test]
	fn test_aligned_high() {
		let map = single_range_map();
		assert_eq!(map.convert_range((15, 20)), HashSet::from([(1005, 1010)]));
	}

	#[test]
	fn test_aligned_exact() {
		let map = single_range_map();
		assert_eq!(map.convert_range((10, 20)), HashSet::from([(1000, 1010)]));
	}

	#[test]
	fn test_overlap_low() {
		let map = single_range_map();
		assert_eq!(
			map.convert_range((5, 15)),
			HashSet::from([(5, 10), (1000, 1005)])
		);
	}

	#[test]
	fn test_overlap_high() {
		let map = single_range_map();
		assert_eq!(
			map.convert_range((15, 25)),
			HashSet::from([(1005, 1010), (20, 25)])
		);
	}

	#[test]
	fn test_subset() {
		let map = single_range_map();
		assert_eq!(map.convert_range((18, 19)), HashSet::from([(1008, 1009)]));
	}

	#[test]
	fn test_superset_single() {
		let map = single_range_map();
		assert_eq!(
			map.convert_range((0, 50)),
			HashSet::from([(0, 10), (1000, 1010), (20, 50)])
		);
	}

	#[test]
	fn test_overlap_contiguous() {
		let map = contiguous_ranges_map();
		assert_eq!(
			map.convert_range((15, 25)),
			HashSet::from([(1005, 1010), (100, 105)])
		);
	}

	#[test]
	fn test_overlap_spaced() {
		let map = spaced_ranges_map();
		assert_eq!(
			map.convert_range((15, 35)),
			HashSet::from([(1005, 1010), (20, 30), (100, 105)])
		);
	}

	#[test]
	fn test_between_spaced() {
		let map = spaced_ranges_map();
		assert_eq!(map.convert_range((25, 26)), HashSet::from([(25, 26)]));
	}

	#[test]
	fn test_superset_contiguous() {
		let map = contiguous_ranges_map();
		assert_eq!(
			map.convert_range((0, 100)),
			HashSet::from([(0, 10), (1000, 1010), (100, 110), (30, 100)])
		);
	}

	#[test]
	fn test_superset_spaced() {
		let map = spaced_ranges_map();
		assert_eq!(
			map.convert_range((0, 100)),
			HashSet::from([(0, 10), (1000, 1010), (20, 30), (100, 110), (40, 100)])
		);
	}

	#[test]
	fn test_convert() {
		let map = single_range_map();
		let value_ranges = HashSet::from([(0, 2), (8, 15), (18, 21), (50, 51)]);
		let expected = HashSet::from([
			(0, 2),
			(8, 10),
			(1000, 1005),
			(1008, 1010),
			(20, 21),
			(50, 51),
		]);
		assert_eq!(map.convert(&value_ranges), expected);
	}
}

/// A group of conversion maps.
#[derive(Debug, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Converter {
	pub maps: Vec<ConversionMap>,
}

impl Converter {
	/// Apply all possible conversions to the value ranges.
	/// Performance: This generates conversions that might not be useful for the needed end result.
	pub fn convert_all(&self, almanac: &mut Almanac) {
		loop {
			let next_map = self.maps.iter().find(|map| {
				almanac.contains_key(&map.name_from) && !almanac.contains_key(&map.name_to)
			});
			if let Some(next_map) = next_map {
				let values = &almanac[&next_map.name_from];
				almanac.insert(next_map.name_to.clone(), next_map.convert(values));
			} else {
				return;
			}
		}
	}
}

#[cfg(test)]
mod test_converter {
	use std::collections::HashSet;

	use crate::parse_input::parse_full;

	#[test]
	fn test_map_all() {
		let input = "foos: 2 1000

foo-to-bar map:
10 0 100

bar-to-baz map:
0 1000 1
";
		let (mut almanac, converter) = parse_full(input, false);

		converter.convert_all(&mut almanac);

		assert_eq!(almanac["foo"], HashSet::from([(2, 3), (1000, 1001)]));
		assert_eq!(almanac["bar"], HashSet::from([(12, 13), (1000, 1001)]));
		assert_eq!(almanac["baz"], HashSet::from([(12, 13), (0, 1)]));
	}
}
