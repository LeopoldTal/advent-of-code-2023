use std::{collections::HashSet, usize};

use crate::{
	arithmetic::{all_congruences, lcm},
	cycle_detector::CycleAnalysis,
};

/// Combines multiple paths into the overall path-and-cycle hitting all goals at once.
pub fn merge(cycles: &[CycleAnalysis]) -> CycleAnalysis {
	let mut merged = cycles.first().expect("Nothing to merge").clone();
	for cycle in cycles.iter().skip(1) {
		merged = merged.merge(cycle);
	}
	merged
}

impl CycleAnalysis {
	/// Gets the first goal hit along the path.
	pub fn first_goal(&self) -> usize {
		self.goals_before(usize::MAX)
			.next()
			.expect("No goals exist")
	}

	/// Gets the trajectory that hits the goals of both cycles at once.
	pub fn merge(&self, other: &Self) -> Self {
		let cycle_length = lcm(self.cycle_length, other.cycle_length);
		let time_to_cycle = self.time_to_cycle.max(other.time_to_cycle);

		// Goals hit before both paths have entered their cycles.
		let goals_before_cycle_self = self.goals_before(time_to_cycle);
		let goals_before_cycle_other: HashSet<usize> = other.goals_before(time_to_cycle).collect();
		let goals_before_cycle: Vec<usize> = goals_before_cycle_self
			.filter(|index| goals_before_cycle_other.contains(index))
			.collect();

		// Goals hit when cycles align.
		let goals_remainders = all_congruences(
			&self.goals_remainders,
			self.cycle_length,
			&other.goals_remainders,
			other.cycle_length,
		);

		CycleAnalysis {
			cycle_length,
			time_to_cycle,
			goals_remainders,
			goals_before_cycle,
		}
	}

	/// Lists all goals until a given point is hit.
	fn goals_before(&self, end: usize) -> impl Iterator<Item = usize> + '_ {
		let before_cycle = self
			.goals_before_cycle
			.iter()
			.filter(move |&&index| index < end)
			.copied();
		let on_cycle = (0usize..)
			.map(|cycle_iter| cycle_iter * self.cycle_length)
			.flat_map(|cycle_start| {
				self.goals_remainders
					.iter()
					.map(move |remainder| cycle_start + remainder)
			})
			.skip_while(|&index| index < self.time_to_cycle)
			.take_while(move |&index| index < end);
		before_cycle.chain(on_cycle)
	}
}

#[cfg(test)]
mod test_first_goal {
	use super::*;

	#[test]
	fn test_before_cycle() {
		let cycle = CycleAnalysis {
			cycle_length: 42,
			time_to_cycle: 23,
			goals_remainders: vec![2, 3],
			goals_before_cycle: vec![7, 19],
		};
		assert_eq!(cycle.first_goal(), 7);
	}

	#[test]
	fn test_on_cycle() {
		let cycle = CycleAnalysis {
			cycle_length: 42,
			time_to_cycle: 23,
			goals_remainders: vec![2, 3],
			goals_before_cycle: vec![],
		};
		assert_eq!(cycle.first_goal(), 23 + 21);
	}
}

#[cfg(test)]
mod test_merge {
	use super::*;

	#[test]
	fn test_merge_with_self() {
		let cycle = CycleAnalysis {
			cycle_length: 42,
			time_to_cycle: 1001,
			goals_remainders: vec![0, 1, 12],
			goals_before_cycle: vec![847],
		};
		assert_eq!(cycle.merge(&cycle), cycle);
	}

	#[test]
	fn test_incompatible() {
		let cycle1 = CycleAnalysis {
			cycle_length: 12,
			time_to_cycle: 50,
			goals_remainders: vec![0, 1, 3],
			goals_before_cycle: vec![42],
		};
		let cycle2 = CycleAnalysis {
			cycle_length: 12,
			time_to_cycle: 49,
			goals_remainders: vec![2, 6],
			goals_before_cycle: vec![0, 1, 2],
		};
		let expected = CycleAnalysis {
			cycle_length: 12,
			time_to_cycle: 50,
			goals_remainders: vec![],
			goals_before_cycle: vec![],
		};
		assert_eq!(cycle1.merge(&cycle2), expected);
	}

	#[test]
	fn test_same_cycle_length() {
		let cycle1 = CycleAnalysis {
			cycle_length: 42,
			time_to_cycle: 50,
			goals_remainders: vec![0, 1, 3, 7],
			goals_before_cycle: vec![13, 42],
		};
		let cycle2 = CycleAnalysis {
			cycle_length: 42,
			time_to_cycle: 23,
			goals_remainders: vec![2, 3, 6, 7],
			goals_before_cycle: vec![0, 1, 2, 13, 17],
		};
		let expected = CycleAnalysis {
			cycle_length: 42,
			time_to_cycle: 50,
			goals_remainders: vec![3, 7],
			goals_before_cycle: vec![13],
		};
		assert_eq!(cycle1.merge(&cycle2), expected);
	}

	#[test]
	fn test_coprime_cycle_length_aligned() {
		let cycle1 = CycleAnalysis {
			cycle_length: 3,
			time_to_cycle: 0,
			goals_remainders: vec![0],
			goals_before_cycle: vec![],
		};
		let cycle2 = CycleAnalysis {
			cycle_length: 4,
			time_to_cycle: 0,
			goals_remainders: vec![0],
			goals_before_cycle: vec![],
		};
		let expected = CycleAnalysis {
			cycle_length: 3 * 4,
			time_to_cycle: 0,
			goals_remainders: vec![0],
			goals_before_cycle: vec![],
		};
		assert_eq!(cycle1.merge(&cycle2), expected);
	}

	#[test]
	fn test_coprime_cycle_length_misaligned() {
		/*
			0 [1] 2  0 [1] 2  0 [1] 2  0  [1]  2
			0  1 [2] 3  0  1 [2] 3  0  1  [2]  0
			0  1  2  3  4  5  6  7  8  9 [10] 11
		*/
		let cycle1 = CycleAnalysis {
			cycle_length: 3,
			time_to_cycle: 0,
			goals_remainders: vec![1],
			goals_before_cycle: vec![],
		};
		let cycle2 = CycleAnalysis {
			cycle_length: 4,
			time_to_cycle: 0,
			goals_remainders: vec![2],
			goals_before_cycle: vec![],
		};
		let expected = CycleAnalysis {
			cycle_length: 3 * 4,
			time_to_cycle: 0,
			goals_remainders: vec![10],
			goals_before_cycle: vec![],
		};
		assert_eq!(cycle1.merge(&cycle2), expected);
	}

	#[test]
	fn test_lcm_cycle_length_aligned() {
		let cycle1 = CycleAnalysis {
			cycle_length: 42 * 27,
			time_to_cycle: 0,
			goals_remainders: vec![0],
			goals_before_cycle: vec![],
		};
		let cycle2 = CycleAnalysis {
			cycle_length: 42 * 256 * 11,
			time_to_cycle: 0,
			goals_remainders: vec![0],
			goals_before_cycle: vec![],
		};
		let expected = CycleAnalysis {
			cycle_length: 42 * 27 * 256 * 11,
			time_to_cycle: 0,
			goals_remainders: vec![0],
			goals_before_cycle: vec![],
		};
		assert_eq!(cycle1.merge(&cycle2), expected);
	}

	#[test]
	fn test_lcm_cycle_length_incompatible() {
		/*
			0  1 [2] 3  0  1 [2] 3  0  1  [2]  3
			0 [1] 2  3  4  5  0 [1] 2  3   4   5
			0  1  2  3  4  5  6  7  8  9 [10] 11
		*/
		let cycle1 = CycleAnalysis {
			cycle_length: 4,
			time_to_cycle: 0,
			goals_remainders: vec![2],
			goals_before_cycle: vec![],
		};
		let cycle2 = CycleAnalysis {
			cycle_length: 6,
			time_to_cycle: 0,
			goals_remainders: vec![1],
			goals_before_cycle: vec![],
		};
		let expected = CycleAnalysis {
			cycle_length: 12,
			time_to_cycle: 0,
			goals_remainders: vec![],
			goals_before_cycle: vec![],
		};
		assert_eq!(cycle1.merge(&cycle2), expected);
	}

	#[test]
	fn test_lcm_cycle_length_misaligned() {
		/*
			0  1  2 [3] 0  1  2 [3] 0  1  2  3
			0 [1] 2  3  4  5  0 [1] 2  3  4  5
			0  1  2  3  4  5  6 [7] 8  9 10 11
		*/
		let cycle1 = CycleAnalysis {
			cycle_length: 4,
			time_to_cycle: 0,
			goals_remainders: vec![3],
			goals_before_cycle: vec![],
		};
		let cycle2 = CycleAnalysis {
			cycle_length: 6,
			time_to_cycle: 0,
			goals_remainders: vec![1],
			goals_before_cycle: vec![],
		};
		let expected = CycleAnalysis {
			cycle_length: 12,
			time_to_cycle: 0,
			goals_remainders: vec![7],
			goals_before_cycle: vec![],
		};
		assert_eq!(cycle1.merge(&cycle2), expected);
	}

	#[test]
	fn test_lcm_cycle_length_multiple() {
		/*
			0 [1] [2][3] 0 [1][2][3] 0 [1] [2] [3]
			0 [1]  2  3  4  5  0 [1] 2  3   4   5
			0 [1]  2  3  4  5  6 [7] 8  9 [10] 11
		*/
		let cycle1 = CycleAnalysis {
			cycle_length: 4,
			time_to_cycle: 0,
			goals_remainders: vec![1, 2, 3],
			goals_before_cycle: vec![],
		};
		let cycle2 = CycleAnalysis {
			cycle_length: 6,
			time_to_cycle: 0,
			goals_remainders: vec![1],
			goals_before_cycle: vec![],
		};
		let expected = CycleAnalysis {
			cycle_length: 12,
			time_to_cycle: 0,
			goals_remainders: vec![1, 7],
			goals_before_cycle: vec![],
		};
		assert_eq!(cycle1.merge(&cycle2), expected);
	}

	#[test]
	fn test_long_path_before_cycle() {
		/*
			0 [1] 2  3  0 [1] 2  3  0 [1] 2  0
			0  1 [2] 0  1 [2] 0  1  2  0  1 [2]
			0  1  2  3  4 [5] 6  7  8  9 10 11
		*/
		let cycle1 = CycleAnalysis {
			cycle_length: 4,
			time_to_cycle: 1000,
			goals_remainders: vec![1],
			goals_before_cycle: vec![8, 10, 32, 33, 95, 666],
		};
		let cycle2 = CycleAnalysis {
			cycle_length: 3,
			time_to_cycle: 20,
			goals_remainders: vec![2],
			goals_before_cycle: vec![10, 15],
		};
		let expected = CycleAnalysis {
			cycle_length: 12,
			time_to_cycle: 1000,
			goals_remainders: vec![5],
			goals_before_cycle: vec![10, 32, 95],
		};
		assert_eq!(cycle1.merge(&cycle2), expected);
	}
}
