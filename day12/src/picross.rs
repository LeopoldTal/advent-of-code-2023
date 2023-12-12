/// State of a point along a row.
#[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PointState {
	Working,
	Broken,
	Unknown,
}

/// Row of points with partial information
#[derive(Debug, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PointRow {
	pub points: Vec<PointState>,
	pub groups: Vec<usize>,
}

impl PointRow {
	/// Grows into five copies of itself.
	#[must_use]
	pub fn unfold(&self) -> PointRow {
		let mut points = self.points.clone();
		points.push(PointState::Unknown); // FIXME: oops extra at end
		let mut points = points.repeat(5);
		points.pop();

		let groups = self.groups.repeat(5);

		PointRow { points, groups }
	}

	/// Counts possible arrangements of unknown values to match groups.
	#[must_use]
	pub fn get_arrangements_count(&self) -> usize {
		// Brute force for now, use brain later.
		let first_unknown = self
			.points
			.iter()
			.position(|&point| point == PointState::Unknown);
		if let Some(index) = first_unknown {
			let mut fixed = self.clone();
			fixed.points[index] = PointState::Working;
			let count_working = fixed.get_arrangements_count();
			fixed.points[index] = PointState::Broken;
			let count_broken = fixed.get_arrangements_count();
			count_working + count_broken
		} else {
			self.check_groups().into()
		}
	}

	/// Once all unknowns are filled in, check whether the resulting groups match the spec.
	#[must_use]
	fn check_groups(&self) -> bool {
		let mut groups: Vec<usize> = vec![];
		let mut cur_group_len: usize = 0;
		for point in &self.points {
			match point {
				PointState::Working => {
					if cur_group_len > 0 {
						groups.push(cur_group_len);
					}
					cur_group_len = 0;
				}
				PointState::Broken => {
					cur_group_len += 1;
				}
				PointState::Unknown => panic!("Only call this on already filled-in rows"),
			}
		}
		if cur_group_len > 0 {
			groups.push(cur_group_len);
		}
		groups == self.groups
	}
}

#[cfg(test)]
mod test {

	use super::{
		PointState::{Broken, Unknown, Working},
		*,
	};

	#[test]
	fn test_minimal() {
		let row = PointRow {
			points: vec![Unknown],
			groups: vec![1],
		};
		assert_eq!(row.get_arrangements_count(), 1);
	}

	#[test]
	fn test_all_unknown() {
		let row = PointRow {
			points: vec![Unknown, Unknown],
			groups: vec![1],
		};
		assert_eq!(row.get_arrangements_count(), 2);
	}

	#[test]
	fn test_must_have_gap() {
		let row = PointRow {
			points: vec![Broken, Unknown, Unknown],
			groups: vec![1],
		};
		assert_eq!(row.get_arrangements_count(), 1);
	}

	#[test]
	fn test_cant_have_gap() {
		let row = PointRow {
			points: vec![Broken, Unknown, Unknown],
			groups: vec![2],
		};
		assert_eq!(row.get_arrangements_count(), 1);
	}

	#[test]
	fn test_either_side() {
		let row = PointRow {
			points: vec![Unknown, Working, Unknown],
			groups: vec![1],
		};
		assert_eq!(row.get_arrangements_count(), 2);
	}

	#[test]
	fn test_unfold() {
		let folded = PointRow {
			points: vec![Working],
			groups: vec![1],
		};
		let expected = PointRow {
			points: vec![
				Working, Unknown, Working, Unknown, Working, Unknown, Working, Unknown, Working,
			],
			groups: vec![1; 5],
		};
		assert_eq!(folded.unfold(), expected);
	}
}
