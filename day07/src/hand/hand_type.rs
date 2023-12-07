/// Types of hands you can get.
#[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum HandType {
	HighCard,
	Pair,
	TwoPair,
	ThreeOfKind,
	FullHouse,
	FourOfKind,
	FiveOfKind,
}

#[cfg(test)]
mod test_hand_type {
	use super::*;

	#[test]
	fn test_ordering() {
		let mut types = vec![
			HandType::FiveOfKind,
			HandType::FourOfKind,
			HandType::FullHouse,
			HandType::HighCard,
			HandType::Pair,
			HandType::ThreeOfKind,
			HandType::TwoPair,
		];
		let sorted_types = vec![
			HandType::HighCard,
			HandType::Pair,
			HandType::TwoPair,
			HandType::ThreeOfKind,
			HandType::FullHouse,
			HandType::FourOfKind,
			HandType::FiveOfKind,
		];
		types.sort();
		assert_eq!(types, sorted_types);
	}
}
