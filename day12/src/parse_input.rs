use nom::{
	bytes::complete::tag,
	character::complete::{multispace1, one_of, space1, u16},
	multi::{many1, separated_list1},
	IResult,
};

use crate::picross::{PointRow, PointState};

/// Consumes one point on the row.
fn point_state(input: &str) -> IResult<&str, PointState> {
	let (input, point) = one_of(".#?")(input)?;
	let point = match point {
		'.' => PointState::Working,
		'#' => PointState::Broken,
		'?' => PointState::Unknown,
		_ => unreachable!(),
	};
	Ok((input, point))
}

/// Consumes group numbers.
fn groups(input: &str) -> IResult<&str, Vec<usize>> {
	let (input, groups) = separated_list1(tag(","), u16)(input)?;
	let groups = groups.into_iter().map(|coord| coord as usize).collect();
	Ok((input, groups))
}

/// Consumes a whole row.
fn point_row(input: &str) -> IResult<&str, PointRow> {
	let (input, points) = many1(point_state)(input)?;
	let (input, _) = space1(input)?;
	let (input, groups) = groups(input)?;
	let (input, _) = multispace1(input)?;
	Ok((input, PointRow { points, groups }))
}

/// Parses the whole input.
/// # Panics
/// On any parse error.
#[must_use]
pub fn parse_full(input: &str) -> Vec<PointRow> {
	let (_, point_rows) = many1(point_row)(input).expect("Parse error");
	point_rows
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::picross::PointState::{Broken, Unknown, Working};

	#[test]
	fn test_parse() {
		let input = "#.? 1,1\n? 1\n";
		let expected = vec![
			PointRow {
				points: vec![Broken, Working, Unknown],
				groups: vec![1, 1],
			},
			PointRow {
				points: vec![Unknown],
				groups: vec![1],
			},
		];
		assert_eq!(parse_full(input), expected);
	}
}
