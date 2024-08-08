use crate::buckets::BucketList;

/// Single write instruction for the hashmap.
#[derive(Debug, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Instruction {
	Set(String, usize),
	Remove(String),
}

pub fn execute(list: &mut BucketList, instruction: &Instruction) {
	match instruction {
		Instruction::Set(key, value) => list.set(key, *value),
		Instruction::Remove(key) => list.remove(key),
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_execute() {
		let mut list = BucketList::new();
		let mut expected = BucketList::new();

		execute(&mut list, &Instruction::Set(String::from("foobar"), 1337));
		expected.set("foobar", 1337);
		assert_eq!(list, expected);

		execute(&mut list, &Instruction::Remove(String::from("foobar")));
		expected.remove("foobar");
		assert_eq!(list, expected);
	}
}
