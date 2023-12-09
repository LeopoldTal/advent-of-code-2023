/// Difference between each element and the next.
pub fn diff(l: &[i64]) -> Vec<i64> {
	(0..l.len() - 1)
		.map(|index| l[index + 1] - l[index])
		.collect()
}

/// Successive differences.
pub fn all_diffs(l: &[i64]) -> Vec<Vec<i64>> {
	let mut latest = l.to_vec();
	let mut diffs = vec![];
	while !latest.iter().all(|&diff| diff == 0) {
		diffs.push(latest.clone());
		latest = diff(&latest);
	}
	diffs.push(latest);
	diffs
}

#[cfg(test)]
mod test_diff {
	use super::*;

	#[test]
	fn test_constant() {
		let input = vec![42, 42, 42, 42];
		let expected = vec![0, 0, 0];
		assert_eq!(diff(&input), expected);
	}

	#[test]
	fn test_increase() {
		let input = vec![1, 2, 4, 7];
		let expected = vec![1, 2, 3];
		assert_eq!(diff(&input), expected);
	}

	#[test]
	fn test_decrease() {
		let input = vec![10, 4];
		let expected = vec![-6];
		assert_eq!(diff(&input), expected);
	}

	#[test]
	fn test_successive_exhausts() {
		let input = vec![1, 2, 4, 8];
		let expected = vec![vec![1, 2, 4, 8], vec![1, 2, 4], vec![1, 2], vec![1], vec![]];
		assert_eq!(all_diffs(&input), expected);
	}

	#[test]
	fn test_successive_flattens() {
		let input = vec![10, 10, 13, 19, 28];
		let expected = vec![
			vec![10, 10, 13, 19, 28],
			vec![0, 3, 6, 9],
			vec![3, 3, 3],
			vec![0, 0],
		];
		assert_eq!(all_diffs(&input), expected);
	}
}

/// Guesses the next value using the successive differences.
pub fn extrapolate(l: &[i64], backwards: bool) -> i64 {
	let mut diffs = all_diffs(l);
	diffs.reverse();

	let sign = if backwards { -1 } else { 1 };

	let mut extrapolated = 0;
	for l in diffs {
		let last_datum = if backwards { l.first() } else { l.last() };
		let last_datum = last_datum.unwrap_or(&0);
		extrapolated = last_datum + sign * extrapolated;
	}
	extrapolated
}

#[cfg(test)]
mod test_extrapolate {
	use super::*;

	#[test]
	fn test_constant() {
		let input = vec![42, 42, 42, 42];
		assert_eq!(extrapolate(&input, false), 42);
		assert_eq!(extrapolate(&input, true), 42);
	}

	#[test]
	fn test_increase() {
		let input = vec![1, 2, 4, 7];
		assert_eq!(extrapolate(&input, false), 11);
		assert_eq!(extrapolate(&input, true), 1);
	}

	#[test]
	fn test_decrease() {
		let input = vec![10, 4];
		assert_eq!(extrapolate(&input, false), -2);
		assert_eq!(extrapolate(&input, true), 16);
	}

	#[test]
	fn test_exhausts() {
		let input = vec![1, 2, 4, 8];
		assert_eq!(extrapolate(&input, false), 15);
		assert_eq!(extrapolate(&input, true), 0);
	}

	#[test]
	fn test_flattens() {
		let input = vec![10, 10, 13, 19, 28];
		assert_eq!(extrapolate(&input, false), 40);
		assert_eq!(extrapolate(&input, true), 13);
	}
}
