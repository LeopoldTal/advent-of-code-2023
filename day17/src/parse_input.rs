/// Parses the whole input.
/// # Panics
/// On any parse error.
#[must_use]
pub fn parse_full(input: &str) -> Vec<Vec<u32>> {
	input
		.lines()
		.map(|line| {
			line.chars()
				.map(|ch| ch.to_digit(10).expect("Not a digit"))
				.collect()
		})
		.collect()
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_parse() {
		let input = "123\n456\n";
		let expected = vec![vec![1, 2, 3], vec![4, 5, 6]];
		assert_eq!(parse_full(input), expected);
	}
}
