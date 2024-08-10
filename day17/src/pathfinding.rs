use std::{cmp::Reverse, collections::HashSet};

use crate::{
	board::{
		Board,
		Direction::{Down, Left, Right, Up},
		State,
	},
	constraints::Constraints,
};
use priority_queue::PriorityQueue;

/// A step on a path through the board.
#[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct PathStep {
	pub state: State,
	pub cost_to: u32,
	// Could save previous step if we want to draw the path
}

struct PathQueue {
	queue: PriorityQueue<PathStep, Reverse<u32>>,
}

impl PathQueue {
	/// Creates an empty queue.
	#[must_use]
	pub fn new() -> Self {
		Self {
			queue: PriorityQueue::new(),
		}
	}

	/// Adds a step to the queue.
	pub fn push(&mut self, step: PathStep) {
		self.queue.push(step, Reverse(step.cost_to));
	}

	/// Pops the least-costly step from the queue.
	#[must_use]
	pub fn pop(&mut self) -> PathStep {
		let (step, _) = self.queue.pop().expect("No path exists!");
		step
	}
}

/// Finds the least costly path from start to goal on a board, and returns its cost.
pub fn find_path(board: &Board, start: (usize, usize), goal: (usize, usize)) -> u32 {
	let mut open = PathQueue::new();
	let mut closed = HashSet::<State>::new();

	// Start in any direction
	for step in get_start_steps(start, &board.constraints) {
		open.push(step);
	}

	// Dijkstra
	loop {
		let step = open.pop();
		// If you can't turn, you also can't stop.
		if (step.state.row, step.state.col) == goal && step.state.can_turn_in == 0 {
			return step.cost_to;
		}
		closed.insert(step.state);

		let neighbours = board.get_neighbours(&step.state);
		for (neighbour, new_cost) in neighbours {
			if !closed.contains(&neighbour) {
				let cost_to = step.cost_to + new_cost;
				open.push(PathStep {
					state: neighbour,
					cost_to,
				});
			}
		}
	}
}

fn get_start_steps(
	(row, col): (usize, usize),
	constraints: &Constraints,
) -> impl Iterator<Item = PathStep> {
	[Up, Down, Left, Right]
		.map(|facing| PathStep {
			state: State {
				row,
				col,
				facing,
				must_turn_in: constraints.max_straight_line,
				can_turn_in: constraints.min_straight_line,
			},
			cost_to: 0,
		})
		.into_iter()
}

#[cfg(test)]
mod test_find_path {
	use crate::constraints::CONSTRAINTS_PART_1;

	use super::*;

	#[test]
	fn test_tiny() {
		let board = Board::from(vec![vec![1, 1], vec![1, 1]], CONSTRAINTS_PART_1);
		let distance = find_path(&board, (0, 0), (1, 1));
		assert_eq!(distance, 2);
	}

	#[test]
	fn test_twisty() {
		let board = Board::from(
			vec![vec![1, 2, 2], vec![1, 1, 2], vec![2, 1, 1]],
			CONSTRAINTS_PART_1,
		);
		let distance = find_path(&board, (0, 0), (2, 2));
		assert_eq!(distance, 4);
	}

	#[test]
	fn test_force_turn() {
		let board = Board::from(vec![vec![1; 5], vec![6; 5]], CONSTRAINTS_PART_1);
		let distance = find_path(&board, (0, 0), (0, 4));
		assert_eq!(distance, 4 + 2 * 6);
	}
}
