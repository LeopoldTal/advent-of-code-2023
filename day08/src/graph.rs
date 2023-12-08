use std::collections::HashMap;

/// Left-or-right instruction.
#[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Instruction {
	Left,
	Right,
}

/// Graph node.
#[derive(Debug, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Node<'a> {
	/// Those might have to become owned strings but it'll slap if I can just use refs.
	pub label: &'a str,
	pub left: &'a str,
	pub right: &'a str,
}

impl<'a> Node<'a> {
	/// Less-verbose constructor.
	pub fn from(label: &'a str, left: &'a str, right: &'a str) -> Self {
		Self { label, left, right }
	}
}

/// Graph and isntructions for traversing it
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Game<'a> {
	pub instructions: Vec<Instruction>,
	pub nodes: HashMap<&'a str, Node<'a>>,
}

impl<'a> Game<'a> {
	/// Move from one node to the next.
	pub fn step(&self, from_label: &'a str, instruction: Instruction) -> &'a str {
		let from_node = &self.nodes[from_label];
		match instruction {
			Instruction::Left => from_node.left,
			Instruction::Right => from_node.right,
		}
	}

	/// Traverse until goal, cycling instructions. Returns the number of steps to reach the goal.
	pub fn run(&self, start_label: &'a str, goal_label: &'a str) -> usize {
		let instructions = self.instructions.iter().cycle().enumerate();
		let mut here = start_label;
		for (nb_steps, &instruction) in instructions {
			here = self.step(here, instruction);
			if here == goal_label {
				return nb_steps + 1;
			}
		}
		unreachable!()
	}
}

#[cfg(test)]
mod test {
	use crate::parse_input::parse_full;

	use super::*;
	const SAMPLE_INPUT: &str = include_str!("../input_sample_a.txt");

	#[test]
	fn test_step_left() {
		let game = parse_full(SAMPLE_INPUT);
		let label = game.step("BBB", Instruction::Left);
		assert_eq!(label, "DDD");
	}

	#[test]
	fn test_step_right() {
		let game = parse_full(SAMPLE_INPUT);
		let label = game.step("CCC", Instruction::Right);
		assert_eq!(label, "GGG");
	}

	#[test]
	fn test_sample_one_step() {
		let game = parse_full(SAMPLE_INPUT);
		let nb_steps = game.run("BBB", "EEE");
		assert_eq!(nb_steps, 1);
	}

	#[test]
	fn test_sample_two_steps() {
		let game = parse_full(
			"LR

one = (two, one)
two = (one, three)
three = (one, one)
",
		);
		let nb_steps = game.run("one", "three");
		assert_eq!(nb_steps, 2);
	}

	#[test]
	fn test_sample_repeated() {
		let game = parse_full(
			"L

one = (two, one)
two = (three, one)
three = (four, one)
four = (one, one)
",
		);
		let nb_steps = game.run("one", "four");
		assert_eq!(nb_steps, 3);
	}
}
