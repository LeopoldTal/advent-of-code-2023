use std::collections::HashMap;

use nom::{
	bytes::complete::tag,
	character::complete::{alpha1, multispace0, one_of},
	multi::many1,
	IResult,
};

use crate::graph::{Game, Instruction, Node};

/// Consumes one instruction.
fn instruction(input: &str) -> IResult<&str, Instruction> {
	let (input, instruction) = one_of("LR")(input)?;
	let instruction = if instruction == 'L' {
		Instruction::Left
	} else {
		Instruction::Right
	};
	Ok((input, instruction))
}

/// Consumes all instructions.
fn instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
	let (input, instructions) = many1(instruction)(input)?;
	let (input, _) = multispace0(input)?;
	Ok((input, instructions))
}

/// Consumes a labelled node.
fn node(input: &str) -> IResult<&str, Node> {
	let (input, label) = alpha1(input)?;
	let (input, _) = tag(" = (")(input)?;
	let (input, left) = alpha1(input)?;
	let (input, _) = tag(", ")(input)?;
	let (input, right) = alpha1(input)?;
	let (input, _) = tag(")")(input)?;
	let (input, _) = multispace0(input)?;
	let node = Node::from(label, left, right);
	Ok((input, node))
}

/// Consumes all nodes.
fn nodes(input: &str) -> IResult<&str, HashMap<&str, Node>> {
	let (input, nodes) = many1(node)(input)?;
	let nodes = nodes.into_iter().map(|node| (node.label, node)).collect();
	Ok((input, nodes))
}

/// Parses the whole input.
/// # Panics
/// On any parse error.
#[must_use]
pub fn parse_full(input: &str) -> Game {
	let (input, instructions) = instructions(input).expect("Invalid instructions");
	let (_, nodes) = nodes(input).expect("Invalid graph nodes");
	Game {
		instructions,
		nodes,
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::graph::Instruction::*;

	#[test]
	fn test_instructions() {
		let input = "RL\n";
		let (_, parsed) = instructions(input).unwrap();
		assert_eq!(parsed, vec![Right, Left])
	}

	#[test]
	fn test_nodes() {
		let input = "foo = (bar, baz)\nqux = (fum, zot)\n";
		let (_, parsed) = nodes(input).unwrap();
		let node1 = Node::from("foo", "bar", "baz");
		let node2 = Node::from("qux", "fum", "zot");
		let expected = HashMap::from([("foo", node1), ("qux", node2)]);
		assert_eq!(parsed, expected);
	}

	#[test]
	fn test_parse_full() {
		let input_instructions = "RRLLRL\n";
		let input_nodes = "ABC = (DEF, GHI)\nJKL = (MNO, PQR)\nSTU = (VWX, YZA)\n";
		let (_, instructions) = instructions(input_instructions).unwrap();
		let (_, nodes) = nodes(input_nodes).unwrap();
		let expected = Game {
			instructions,
			nodes,
		};

		let input = [input_instructions, input_nodes].join("\n");
		let parsed = parse_full(&input);
		assert_eq!(parsed, expected);
	}
}
