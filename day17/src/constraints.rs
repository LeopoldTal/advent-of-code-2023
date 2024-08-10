/// Constraints on the mover on the board
#[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Constraints {
	pub max_straight_line: usize,
	pub min_straight_line: usize,
}

pub const CONSTRAINTS_PART_1: Constraints = Constraints {
	max_straight_line: 3,
	min_straight_line: 0,
};

pub const CONSTRAINTS_PART_2: Constraints = Constraints {
	max_straight_line: 10,
	min_straight_line: 4,
};
