use std::io::{self, Read};

type DigitMatch = (&'static str, &'static str, u32);

const MATCHES: &[DigitMatch] = &[
	("1", "one", 1),
	("2", "two", 2),
	("3", "three", 3),
	("4", "four", 4),
	("5", "five", 5),
	("6", "six", 6),
	("7", "seven", 7),
	("8", "eight", 8),
	("9", "nine", 9),
];

#[must_use]
fn find_digit(s: &str, include_words: bool, backwards: bool) -> u32 {
	let pred = if backwards {
		str::ends_with
	} else {
		str::starts_with
	};

	assert!(!s.is_empty(), "No digits");

	for (digit_match, word_match, value) in MATCHES {
		if pred(s, digit_match) || include_words && pred(s, word_match) {
			return *value;
		}
	}

	let rest = if backwards {
		&s[..s.len() - 1]
	} else {
		&s[1..]
	};
	find_digit(rest, include_words, backwards)
}

#[must_use]
fn get_digits(s: &str, include_words: bool) -> u32 {
	let first_digit = find_digit(s, include_words, false);
	let last_digit = find_digit(s, include_words, true);
	10 * first_digit + last_digit
}

#[must_use]
fn get_total(s: &str, include_words: bool) -> u32 {
	s.lines().map(|line| get_digits(line, include_words)).sum()
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_get_digits_only_numeric() {
		assert_eq!(get_digits("1abc2", false), 12);
		assert_eq!(get_digits("pqr3stu8vwx", false), 38);
		assert_eq!(get_digits("a1b2c3d4e5f", false), 15);
		assert_eq!(get_digits("treb7uchet", false), 77);
	}

	#[test]
	fn test_get_digits_ignoring_letters() {
		assert_eq!(get_digits("two1nine", false), 11);
		assert_eq!(get_digits("abcone2threexyz", false), 22);
		assert_eq!(get_digits("xtwone3four", false), 33);
		assert_eq!(get_digits("4nineeightseven2", false), 42);
		assert_eq!(get_digits("zoneight234", false), 24);
		assert_eq!(get_digits("7pqrstsixteen", false), 77);
	}

	#[test]
	fn test_get_digits_with_letters() {
		assert_eq!(get_digits("two1nine", true), 29);
		assert_eq!(get_digits("eightwothree", true), 83);
		assert_eq!(get_digits("abcone2threexyz", true), 13);
		assert_eq!(get_digits("xtwone3four", true), 24);
		assert_eq!(get_digits("4nineeightseven2", true), 42);
		assert_eq!(get_digits("zoneight234", true), 14);
		assert_eq!(get_digits("zoneight", true), 18);
		assert_eq!(get_digits("7pqrstsixteen", true), 76);
		assert_eq!(get_digits("thebeautyofourweapons", true), 44);
	}

	#[test]
	fn test_single_line() {
		assert_eq!(get_total("1", false), 11);
	}

	#[test]
	fn test_sample_a() {
		const SAMPLE_INPUT: &str = include_str!("../input_sample_a.txt");
		assert_eq!(get_total(SAMPLE_INPUT, false), 142);
	}

	#[test]
	fn test_sample_b() {
		const SAMPLE_INPUT: &str = include_str!("../input_sample_b.txt");
		assert_eq!(get_total(SAMPLE_INPUT, true), 281);
	}
}

fn main() {
	let mut input = String::new();
	io::stdin()
		.read_to_string(&mut input)
		.expect("Failed to read input");

	println!("Numeric only: {}", get_total(&input, false));
	println!("With letters: {}", get_total(&input, true));
}
