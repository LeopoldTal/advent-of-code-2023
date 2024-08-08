pub const SIZE: usize = 256;

/// Hash function for this hashmap.
#[must_use]
#[allow(clippy::module_name_repetitions)]
pub fn get_hash(input: &str) -> usize {
	let mut hash = 0;
	for ch in input.bytes() {
		hash += usize::from(ch);
		hash *= 17;
		hash %= SIZE;
	}
	hash
}

#[cfg(test)]
mod test {
	use super::*;
	#[test]
	fn test_empty() {
		assert_eq!(get_hash(""), 0);
	}

	#[test]
	fn test_single_char() {
		assert_eq!(get_hash("H"), 200);
	}

	#[test]
	fn test_hash() {
		assert_eq!(get_hash("HASH"), 52);
		assert_eq!(get_hash("cm"), 0);
		assert_eq!(get_hash("rn"), 0);
		assert_eq!(get_hash("qp"), 1);
		assert_eq!(get_hash("ab"), 3);
		assert_eq!(get_hash("ot"), 3);
		assert_eq!(get_hash("pc"), 3);
		assert_eq!(get_hash("rn=1"), 30);
		assert_eq!(get_hash("cm-"), 253);
		assert_eq!(get_hash("qp=3"), 97);
		assert_eq!(get_hash("cm=2"), 47);
		assert_eq!(get_hash("qp-"), 14);
		assert_eq!(get_hash("pc=4"), 180);
		assert_eq!(get_hash("ot=9"), 9);
		assert_eq!(get_hash("ab=5"), 197);
		assert_eq!(get_hash("pc-"), 48);
		assert_eq!(get_hash("pc=6"), 214);
		assert_eq!(get_hash("ot=7"), 231);
	}
}
