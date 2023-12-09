/// Parses one list of numbers on a line.
#[must_use]
pub fn parse_line(input: &str) -> Vec<i64> {
	input.split_whitespace().flat_map(str::parse).collect()
}

/// Parses the whole input.
/// # Panics
/// On any parse error.
#[must_use]
pub fn parse_full(input: &str) -> Vec<Vec<i64>> {
	input.lines().map(parse_line).collect()
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_parse() {
		let input = "1 2 3\n1337\n";
		let expected = vec![vec![1, 2, 3], vec![1337]];
		assert_eq!(parse_full(input), expected);
	}
}
