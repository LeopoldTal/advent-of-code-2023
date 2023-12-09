/// Greatest common divisor.
pub fn gcd(a: usize, b: usize) -> usize {
	let r = a % b;
	if r == 0 {
		b
	} else {
		gcd(b, r)
	}
}

#[cfg(test)]
mod test_gcd {
	use super::*;

	#[test]
	fn test_self() {
		assert_eq!(gcd(1337, 1337), 1337);
	}

	#[test]
	fn test_coprime() {
		assert_eq!(gcd(23, 256), 1);
	}

	#[test]
	fn test_shared_factors() {
		let shared = 17 * 13 * 4;
		assert_eq!(gcd(23 * shared, 256 * shared), shared);
	}
}

/// Least common multiple.
pub fn lcm(a: usize, b: usize) -> usize {
	a / gcd(a, b) * b
}

#[cfg(test)]
mod test_lcm {
	use super::*;

	#[test]
	fn test_self() {
		assert_eq!(lcm(42, 42), 42);
	}

	#[test]
	fn test_coprime() {
		assert_eq!(lcm(42, 125), 42 * 125);
	}

	#[test]
	fn test_shared_factors() {
		let shared = 17 * 13 * 4;
		assert_eq!(lcm(42 * shared, 125 * shared), 42 * 125 * shared);
	}
}

/// Gets all numbers congruent to any pair from the given lists.
pub fn all_congruences(
	remainders_a: &[usize],
	base_a: usize,
	remainders_b: &[usize],
	base_b: usize,
) -> Vec<usize> {
	let mut indices: Vec<usize> = remainders_a
		.iter()
		.flat_map(|remainder_a| {
			remainders_b
				.iter()
				.map(|remainder_b| congruence((*remainder_a, base_a), (*remainder_b, base_b)))
		})
		.flatten()
		.collect();
	indices.sort_unstable();
	indices
}

/// Solves a 2-congruence equation:
/// `congruence((r_a, a), (r_b, b))` ≡ `r_a` mod a, and ≡ `r_b` mod b.
pub fn congruence(
	(remainder_a, base_a): (usize, usize),
	(remainder_b, base_b): (usize, usize),
) -> Option<usize> {
	let shared_factor = gcd(base_a, base_b);
	if remainder_a % shared_factor == remainder_b % shared_factor {
		let solution = congruence_euclid(
			(signed(remainder_a), signed(base_a)),
			(signed(remainder_b), signed(base_b)),
			signed(shared_factor),
		);

		let base = signed(lcm(base_a, base_b));
		Some(solution.rem_euclid(base) as usize)
	} else {
		None
	}
}

/// Solves a 2-congruence equation assuming a solution exists
fn congruence_euclid(
	(remainder_a, base_a): (isize, isize),
	(remainder_b, base_b): (isize, isize),
	shared_factor: isize,
) -> isize {
	let scaled_remainder = (remainder_a - remainder_b) / shared_factor;
	let difference = inverse(base_a, base_b) * base_a * scaled_remainder;

	remainder_a - difference
}

#[cfg(test)]
mod test_congruence {
	use super::*;

	#[test]
	fn test_incompatible() {
		assert_eq!(congruence((0, 8), (1, 4)), None);
	}

	#[test]
	fn test_same_base() {
		assert_eq!(congruence((23, 42), (23, 42)), Some(23));
	}

	#[test]
	fn test_zero() {
		assert_eq!(congruence((0, 8), (0, 5)), Some(0));
	}

	#[test]
	fn test_same_offset() {
		assert_eq!(congruence((2, 5), (2, 3)), Some(2));
	}

	#[test]
	fn test_coprime() {
		assert_eq!(congruence((3, 8), (2, 5)), Some(27));
		assert_eq!(congruence((3, 8), (2, 3)), Some(11));
	}

	#[test]
	fn test_not_coprime() {
		assert_eq!(congruence((5, 6), (3, 8)), Some(11));
		assert_eq!(
			congruence((3 * 17, 8 * 17), (2 * 17, 5 * 17)),
			Some(27 * 17)
		);
	}

	#[test]
	fn test_multiple() {
		let indices = all_congruences(&[0, 3], 8, &[0, 2], 5);
		assert_eq!(indices, vec![0, 27, 32, 35]);
	}

	#[test]
	fn test_multiple_with_incompatible() {
		let indices = all_congruences(&[3], 4, &[0, 1], 2);
		assert_eq!(indices, vec![3]);
	}
}

/// Gets `u` such that `n` divides `m * u - gcd(m, n)`.
fn inverse(a: isize, b: isize) -> isize {
	let mut r0 = b;
	let mut r1 = a;

	let mut t0 = 0;
	let mut t1 = 1;

	while r1 > 0 {
		let q: isize = r0 / r1;
		let new_remainder = r0.rem_euclid(r1);
		r0 = r1;
		r1 = new_remainder;

		let new_t = t0 - q * t1;
		t0 = t1;
		t1 = new_t;
	}

	t0
}

#[cfg(test)]
mod test_inverse {
	use super::*;

	#[test]
	fn test_prime_base() {
		let inv = inverse(8, 19);
		assert_eq!((inv * 8).rem_euclid(19), 1);
	}

	#[test]
	fn test_non_prime_base() {
		let inv = inverse(23, 42);
		assert_eq!((inv * 23).rem_euclid(42), 1);
	}

	#[test]
	fn test_non_coprime() {
		let inv = inverse(6, 8);
		assert_eq!((inv * 6).rem_euclid(8), 2);
	}
}

/// Casts usize to isize with an explicit panic on overflow.
fn signed(x: usize) -> isize {
	isize::try_from(x).expect("Signed integer overflow")
}
