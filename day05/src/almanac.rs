use std::collections::HashMap;

/// A seed value with all its conversions.
pub type Almanac = HashMap<String, u64>;

/// A range to convert a value.
#[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ConversionRange {
	pub from_start: u64,
	pub length: u64,
	pub to_start: u64,
}

impl ConversionRange {
	/// Converts a value iff it falls within the range.
	fn convert(&self, x: u64) -> Option<u64> {
		if x >= self.from_start && x < self.from_start + self.length {
			Some(x + self.to_start - self.from_start)
		} else {
			None
		}
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
		assert_eq!(range.convert(9), None);
		assert_eq!(range.convert(10), Some(100));
		assert_eq!(range.convert(11), Some(101));
		assert_eq!(range.convert(12), None);
		assert_eq!(range.convert(100), None);
	}
}

/// A group of ranges to convert one kind of value to another.
#[derive(Debug, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ConversionMap {
	pub name_from: String,
	pub name_to: String,
	pub ranges: Vec<ConversionRange>,
}

impl ConversionMap {
	/// Converts a value through the map. Unmatched values are unchanged.
	fn convert(&self, x_in: u64) -> u64 {
		for range in &self.ranges {
			if let Some(x_out) = range.convert(x_in) {
				return x_out;
			}
		}
		x_in
	}
}

#[cfg(test)]
mod test_map {
	use super::*;

	#[test]
	fn test_map() {
		let range1 = ConversionRange {
			from_start: 10,
			to_start: 100,
			length: 2,
		};
		let range2 = ConversionRange {
			from_start: 14,
			to_start: 1000,
			length: 10,
		};
		let range3 = ConversionRange {
			from_start: 12,
			to_start: 0,
			length: 1,
		};
		let map = ConversionMap {
			name_from: String::from("in"),
			name_to: String::from("out"),
			ranges: vec![range1, range2, range3],
		};
		assert_eq!(map.convert(9), 9);
		assert_eq!(map.convert(10), 100);
		assert_eq!(map.convert(11), 101);
		assert_eq!(map.convert(12), 0);
		assert_eq!(map.convert(13), 13);
		assert_eq!(map.convert(14), 1000);
		assert_eq!(map.convert(15), 1001);
		assert_eq!(map.convert(16), 1002);
		assert_eq!(map.convert(23), 1009);
		assert_eq!(map.convert(24), 24);
	}
}

/// A group of conversion maps.
#[derive(Debug, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Converter {
	pub maps: Vec<ConversionMap>,
}

impl Converter {
	/// Apply all possible conversions to a value.
	/// Performance: This generates conversions that might not be useful for the needed end result.
	fn convert_all(&self, almanac: &mut Almanac) {
		loop {
			let next_map = self.maps.iter().find(|map| {
				almanac.contains_key(&map.name_from) && !almanac.contains_key(&map.name_to)
			});
			if let Some(next_map) = next_map {
				let x = almanac[&next_map.name_from];
				almanac.insert(next_map.name_to.clone(), next_map.convert(x));
			} else {
				return;
			}
		}
	}

	/// Apply all possible conversions to all values.
	pub fn map_all(&self, almanacs: &mut [Almanac]) {
		for almanac in almanacs {
			self.convert_all(almanac);
		}
	}
}

#[cfg(test)]
mod test_converter {
	use crate::parse_input::parse_full;

	#[test]
	fn test_map_all() {
		let input = "foos: 2 1000

foo-to-bar map:
10 0 100

bar-to-baz map:
0 1000 1
";
		let (mut almanacs, converter) = parse_full(input);

		converter.map_all(&mut almanacs);

		assert_eq!(almanacs[0]["foo"], 2);
		assert_eq!(almanacs[0]["bar"], 12);
		assert_eq!(almanacs[0]["baz"], 12);

		assert_eq!(almanacs[1]["foo"], 1000);
		assert_eq!(almanacs[1]["bar"], 1000);
		assert_eq!(almanacs[1]["baz"], 0);
	}
}
