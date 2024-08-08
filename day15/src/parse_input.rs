use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, u16};
use nom::combinator::all_consuming;
use nom::IResult;

use crate::instruction::Instruction;

/// Splits the whole input into steps.
#[must_use]
pub fn to_steps(input: &str) -> Vec<String> {
	input
		.replace('\n', "")
		.split(',')
		.map(String::from)
		.collect()
}

/// Consumes a set instruction.
fn instruction_set(input: &str) -> IResult<&str, Instruction> {
	let (input, key) = alpha1(input)?;
	let (input, _) = tag("=")(input)?;
	let (input, value) = u16(input)?;
	let instruction = Instruction::Set(String::from(key), usize::from(value));
	Ok((input, instruction))
}

/// Consumes a remove instruction.
fn instruction_remove(input: &str) -> IResult<&str, Instruction> {
	let (input, key) = alpha1(input)?;
	let (input, _) = tag("-")(input)?;
	let instruction = Instruction::Remove(String::from(key));
	Ok((input, instruction))
}

/// Parses one instruction for the hashmap.
/// # Panics
/// On any parse error.
#[must_use]
pub fn parse_instruction(input: &str) -> Instruction {
	let mut parser = all_consuming(alt((instruction_set, instruction_remove)));
	let (_, instruction) = parser(input).expect("Invalid instruction");
	instruction
}

#[cfg(test)]
mod test_to_steps {
	use super::*;

	#[test]
	fn test_empty() {
		assert_eq!(to_steps(""), vec![""]);
	}

	#[test]
	fn test_single_entry() {
		assert_eq!(to_steps("foo"), vec!["foo"]);
	}

	#[test]
	fn test_splits_on_commas() {
		assert_eq!(to_steps("foo,bar,bazqux"), vec!["foo", "bar", "bazqux"]);
	}

	#[test]
	fn test_ignores_newlines() {
		assert_eq!(to_steps("\na\nb\n,\n\nc"), vec!["ab", "c"]);
	}
}

#[cfg(test)]
mod test_parse_instruction {
	use super::*;

	#[test]
	fn test_set() {
		let expected = Instruction::Set(String::from("foo"), 6);
		assert_eq!(parse_instruction("foo=6"), expected);
	}

	#[test]
	fn test_remove() {
		let expected = Instruction::Remove(String::from("yo"));
		assert_eq!(parse_instruction("yo-"), expected);
	}
}
