use std::collections::HashSet;

// A scratchie, with two sets of numbers.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Scratchcard {
	pub id: u32,
	pub winning_numbers: HashSet<u32>,
	pub pulled_numbers: HashSet<u32>,
}

impl Scratchcard {
	/// Gets numbers present in both sets.
	#[must_use]
	pub fn get_matches(&self) -> HashSet<u32> {
		self.winning_numbers
			.intersection(&self.pulled_numbers)
			.copied()
			.collect()
	}

	/// Gets total score of card.
	#[must_use]
	pub fn get_score(&self) -> u32 {
		let nb_matches = u32::try_from(self.get_matches().len()).expect("Too many matches");
		if nb_matches > 0 {
			2_u32.pow(nb_matches - 1)
		} else {
			0
		}
	}
}

#[cfg(test)]
impl Scratchcard {
	/// Build helper.
	#[must_use]
	pub fn from<const N: usize, const M: usize>(
		id: u32,
		winning_numbers: [u32; N],
		pulled_numbers: [u32; M],
	) -> Self {
		Scratchcard {
			id,
			winning_numbers: HashSet::from(winning_numbers),
			pulled_numbers: HashSet::from(pulled_numbers),
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_no_wins() {
		let card = Scratchcard::from(42, [1, 3, 5], [2, 4, 6, 8]);
		assert_eq!(card.get_matches(), HashSet::from([]));
		assert_eq!(card.get_score(), 0);
	}

	#[test]
	fn test_all_wins() {
		let card = Scratchcard::from(42, [1, 2, 3], [1, 2, 3]);
		assert_eq!(card.get_matches(), HashSet::from([1, 2, 3]));
		assert_eq!(card.get_score(), 4);
	}

	#[test]
	fn test_some_wins() {
		let card = Scratchcard::from(42, [1, 3, 5, 6], [1, 2, 4, 5]);
		assert_eq!(card.get_matches(), HashSet::from([1, 5]));
		assert_eq!(card.get_score(), 2);
	}
}
