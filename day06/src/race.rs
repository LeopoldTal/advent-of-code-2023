/// A boat race.
#[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Race {
	pub time_limit: i64,
	pub distance_threshold: i64,
}

impl Race {
	/// Gets the min and max *inclusive* holding times that beat the threshold distance.
	#[must_use]
	fn winning_holds(&self) -> Option<(i64, i64)> {
		let min_distance = self.distance_threshold + 1; // must beat record, not just equal
		let discr_sq = self.time_limit.pow(2) - 4 * min_distance;
		if discr_sq < 0 {
			None
		} else {
			#[allow(clippy::cast_precision_loss)]
			let time = self.time_limit as f64;
			#[allow(clippy::cast_precision_loss)]
			let discr = (discr_sq as f64).sqrt();

			let min_winning = (time - discr) / 2.;
			#[allow(clippy::cast_possible_truncation)]
			let min_winning = min_winning.ceil() as i64;

			let max_winning = (time + discr) / 2.;
			#[allow(clippy::cast_possible_truncation)]
			let max_winning = max_winning.floor() as i64;

			Some((min_winning, max_winning))
		}
	}

	/// Gets the number of winning holding times.
	#[must_use]
	pub fn nb_winning_holds(&self) -> i64 {
		if let Some((min_winning, max_winning)) = self.winning_holds() {
			max_winning - min_winning + 1
		} else {
			0
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_sample_race_1() {
		let race = Race {
			time_limit: 7,
			distance_threshold: 9,
		};
		assert_eq!(race.winning_holds(), Some((2, 5)));
		assert_eq!(race.nb_winning_holds(), 4);
	}

	#[test]
	fn test_sample_race_2() {
		let race = Race {
			time_limit: 15,
			distance_threshold: 40,
		};
		assert_eq!(race.winning_holds(), Some((4, 11)));
		assert_eq!(race.nb_winning_holds(), 8);
	}

	#[test]
	fn test_sample_race_3() {
		let race = Race {
			time_limit: 30,
			distance_threshold: 200,
		};
		assert_eq!(race.winning_holds(), Some((11, 19)));
		assert_eq!(race.nb_winning_holds(), 9);
	}

	#[test]
	fn test_impossible() {
		let race = Race {
			time_limit: 7,
			distance_threshold: 900,
		};
		assert_eq!(race.winning_holds(), None);
		assert_eq!(race.nb_winning_holds(), 0);
	}
}
