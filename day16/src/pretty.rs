type AnsiColour = u8;

#[must_use]
pub fn colourise(ch: char, background: AnsiColour, foreground: AnsiColour) -> String {
	format!("\x1b[38;5;{foreground};48;5;{background}m{ch}\x1b[0m")
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_colours() {
		let input = '╬';
		let colourised = colourise(input, 100, 200);
		assert_eq!(colourised, "\x1b[38;5;200;48;5;100m╬\x1b[0m");
	}
}
