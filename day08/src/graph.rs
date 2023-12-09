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

	/// Gets all node labels matching a predicate.
	pub fn filter_labels(&self, pred: &LabelProp<'a>) -> Vec<&'a str> {
		self.nodes
			.keys()
			.copied()
			.filter(|&label| pred(label))
			.collect()
	}
}

/// Predicate applied to a node label.
pub type LabelProp<'a> = Box<dyn Fn(&'a str) -> bool>;

/// Return a predicate that tests for one exact label name.
pub fn exact<'a>(needle: &'static str) -> LabelProp<'a> {
	Box::new(move |label| label == needle)
}
/// Return a predicate that tests for a suffix.
pub fn ends_with<'a>(needle: &'static str) -> LabelProp<'a> {
	Box::new(move |label| label.ends_with(needle))
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
}
