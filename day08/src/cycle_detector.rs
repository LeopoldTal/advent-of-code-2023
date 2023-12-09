use std::collections::HashSet;

use crate::{
	cycle_combiner::merge,
	graph::{Game, LabelProp},
};

/// Cycle characteristics of a graph path.
#[derive(Debug, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CycleAnalysis {
	pub cycle_length: usize,
	pub time_to_cycle: usize,
	pub goals_remainders: Vec<usize>,
	pub goals_before_cycle: Vec<usize>,
}

impl<'a> Game<'a> {
	/// Traverse until a cycle is detected. Returns:
	/// - the cycle length
	/// - the length of the path before entering the cycle
	/// - the remainders at which the cycle hits a goal
	/// - the goals hit before entering a cycle
	pub fn analyse(&self, start_label: &'a str, goal: &LabelProp<'a>) -> CycleAnalysis {
		let nb_instructions = self.instructions.len();
		let instructions = self.instructions.iter().cycle().enumerate();

		let mut visited = HashSet::from([(0, start_label)]);
		let mut visited_path = vec![start_label];
		let mut here = start_label;
		for (nb_steps, &instruction) in instructions {
			let current_step = nb_steps + 1;
			here = self.step(here, instruction);

			let instruction_step_here = current_step % nb_instructions;

			// We visited this node before, at the same point in the instructions:
			// that means we just ended the first cycle!
			if visited.contains(&(instruction_step_here, here)) {
				// When did we enter the cycle?
				let time_to_cycle = visited_path
					.iter()
					.enumerate()
					.position(|(step, &label)| {
						step % nb_instructions == instruction_step_here && label == here
					})
					.expect("Node was visited but not found in path history");

				let cycle_length = current_step - time_to_cycle;

				// Find goals on and before the cycle.
				let goals_indices = visited_path
					.iter()
					.enumerate()
					.filter(|(_, label)| goal(label))
					.map(|(step, _)| step);

				let (goals_before_cycle, goals_during_cycle): (Vec<usize>, Vec<usize>) =
					goals_indices.partition(|&step| step < time_to_cycle);
				let mut goals_remainders: Vec<usize> = goals_during_cycle
					.into_iter()
					.map(|step| step % cycle_length)
					.collect();
				goals_remainders.sort_unstable();

				return CycleAnalysis {
					cycle_length,
					time_to_cycle,
					goals_remainders,
					goals_before_cycle,
				};
			}

			visited_path.push(here);
			visited.insert((instruction_step_here, here));
		}
		unreachable!()
	}

	/// Compute the number of steps needed to hit goals from every start simultaneously.
	pub fn get_steps_to_all_goals(&self, start: &LabelProp<'a>, goal: &LabelProp<'a>) -> usize {
		let starts = self.filter_labels(start);
		let cycles: Vec<CycleAnalysis> = starts
			.into_iter()
			.map(|start_label| self.analyse(start_label, goal))
			.collect();
		let total_cycle = merge(&cycles);
		total_cycle.first_goal()
	}
}

#[cfg(test)]
mod test {
	use std::vec;

	use crate::{graph::exact, parse_input::parse_full};

	use super::*;

	const SAMPLE_INPUT_SINGLE_PASS: &str = include_str!("../input_sample_a.txt");
	const SAMPLE_INPUT_REPEATED: &str = include_str!("../input_sample_b.txt");
	const SAMPLE_INPUT_MULTIPLE: &str = include_str!("../input_sample_multi.txt");

	#[test]
	fn test_single_point() {
		let game = parse_full(
			"L

only = (only, only)
",
		);

		let expected = CycleAnalysis {
			cycle_length: 1,
			time_to_cycle: 0,
			goals_remainders: vec![0],
			goals_before_cycle: vec![],
		};

		let goal: LabelProp = Box::new(|_| true);
		assert_eq!(game.analyse("only", &goal), expected);
	}

	#[test]
	fn test_two_cycle() {
		let game = parse_full(
			"L

1 = (2, 2)
2 = (1, 1)
",
		);

		let expected = CycleAnalysis {
			cycle_length: 2,
			time_to_cycle: 0,
			goals_remainders: vec![0, 1],
			goals_before_cycle: vec![],
		};

		let goal: LabelProp = Box::new(|_| true);
		assert_eq!(game.analyse("1", &goal), expected);
	}

	#[test]
	fn test_two_cycle_with_path_in() {
		let game = parse_full(
			"L

1 = (2, 2)
2 = (3, 3)
3 = (4, 4)
4 = (5, 5)
5 = (4, 4)
",
		);

		let expected = CycleAnalysis {
			cycle_length: 2,
			time_to_cycle: 3,
			goals_remainders: vec![0, 1],
			goals_before_cycle: vec![0, 1, 2],
		};

		let goal: LabelProp = Box::new(|_| true);
		assert_eq!(game.analyse("1", &goal), expected);
	}

	#[test]
	fn test_three_cycle() {
		let game = parse_full(
			"L

1 = (2, 2)
2 = (3, 3)
3 = (1, 1)
",
		);

		let expected = CycleAnalysis {
			cycle_length: 3,
			time_to_cycle: 0,
			goals_remainders: vec![0, 1],
			goals_before_cycle: vec![],
		};

		let goal: LabelProp = Box::new(|label: &str| label == "1" || label == "2");
		assert_eq!(game.analyse("1", &goal), expected);
	}

	#[test]
	fn test_three_cycle_with_path_in() {
		let game = parse_full(
			"L

1 = (2, 2)
2 = (3, 3)
3 = (4, 4)
4 = (5, 5)
5 = (3, 3)
",
		);

		let expected = CycleAnalysis {
			cycle_length: 3,
			time_to_cycle: 2,
			goals_remainders: vec![2],
			goals_before_cycle: vec![0],
		};

		let goal: LabelProp = Box::new(|label: &str| label == "1" || label == "3");
		assert_eq!(game.analyse("1", &goal), expected);
	}

	#[test]
	fn test_three_cycle_with_instruction_index() {
		let game = parse_full(
			"LR

1 = (2, 2)
2 = (3, 3)
3 = (1, 1)
",
		);

		let expected = CycleAnalysis {
			cycle_length: 6,
			time_to_cycle: 0,
			goals_remainders: vec![0, 1, 3, 4],
			goals_before_cycle: vec![],
		};

		let goal: LabelProp = Box::new(|label: &str| label == "1" || label == "2");
		assert_eq!(game.analyse("1", &goal), expected);
	}

	#[test]
	fn test_three_cycle_with_path_in_and_instruction_index() {
		let game = parse_full(
			"LR

start = (1, 1)
1 = (2, 2)
2 = (3, 3)
3 = (1, 1)
",
		);

		let expected = CycleAnalysis {
			cycle_length: 6,
			time_to_cycle: 1,
			goals_remainders: vec![1, 2, 4, 5],
			goals_before_cycle: vec![],
		};

		let goal: LabelProp = Box::new(|label: &str| label == "1" || label == "2");
		assert_eq!(game.analyse("start", &goal), expected);
	}

	#[test]
	fn test_sample_a() {
		let game = parse_full(SAMPLE_INPUT_SINGLE_PASS);

		let expected = CycleAnalysis {
			cycle_length: 2,
			time_to_cycle: 2,
			goals_remainders: vec![0, 1],
			goals_before_cycle: vec![],
		};

		let goal: LabelProp = exact("ZZZ");
		assert_eq!(game.analyse("AAA", &goal), expected);
	}

	#[test]
	fn test_sample_b() {
		let game = parse_full(SAMPLE_INPUT_REPEATED);

		let expected = CycleAnalysis {
			cycle_length: 3,
			time_to_cycle: 6,
			goals_remainders: vec![0, 1, 2],
			goals_before_cycle: vec![],
		};

		let goal: LabelProp = exact("ZZZ");
		assert_eq!(game.analyse("AAA", &goal), expected);
	}

	#[test]
	fn test_sample_multi_first_cycle() {
		let game = parse_full(SAMPLE_INPUT_MULTIPLE);

		let expected = CycleAnalysis {
			cycle_length: 2,
			time_to_cycle: 1,
			goals_remainders: vec![0],
			goals_before_cycle: vec![],
		};

		let goal: LabelProp = exact("11Z");
		assert_eq!(game.analyse("11A", &goal), expected);
	}

	#[test]
	fn test_sample_multi_second_cycle() {
		let game = parse_full(SAMPLE_INPUT_MULTIPLE);

		let expected = CycleAnalysis {
			cycle_length: 6,
			time_to_cycle: 1,
			goals_remainders: vec![0, 3],
			goals_before_cycle: vec![],
		};

		let goal: LabelProp = exact("22Z");
		assert_eq!(game.analyse("22A", &goal), expected);
	}
}
