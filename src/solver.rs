pub mod bit_pattern;
pub mod board;
mod board_key;
pub mod direction;
pub mod move_path;
pub mod piece;
pub mod rule;
pub mod state;
mod visited_history;

use super::bfs;
use board_key::BoardKey;
use direction::Direction;
use move_path::MovePath;
use rule::Rule;
use state::State;
use visited_history::VisitedHistory;

/// Solves the klotski puzzle using a breadth-first search algorithm.
pub fn solve(rule: &Rule) -> Option<Vec<State>> {
    let start_state = State {
        board: rule.start.clone(),
        piece: None,
        path: MovePath::None,
    };

    let is_goal = |s: &State| rule.is_finished(&s.board);
    let neighbors = |s: &State| get_neighbors(rule, s);

    let mut visited = VisitedHistory::new();
    let try_visit =
        |s: &State, depth: usize| visited.try_visit(BoardKey::create(rule, &s.board), depth);

    bfs::find_path(&start_state, is_goal, neighbors, try_visit)
}

/// All possible directions for moving pieces in the puzzle.
static ALL_DIRECTIONS: &[Direction] = &[
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];

/// Creates the next possible states from the current state based on the given rule.
fn get_neighbors(rule: &Rule, current_state: &State) -> Vec<State> {
    let mut next_states = vec![];
    let current_board = &current_state.board;
    for &piece in &rule.pieces {
        if let Some(prev_piece) = current_state.piece {
            if prev_piece == piece {
                // Do not move the same piece as last time.
                continue;
            }
        }
        for &direction in ALL_DIRECTIONS {
            if let Some(next_board) = current_board.move_piece(piece, direction) {
                // Move a piece in a certain direction.
                let next_state = State {
                    board: next_board.clone(),
                    piece: Some(piece),
                    path: MovePath::One(direction),
                };
                next_states.push(next_state);

                // There are two blank spaces on the board.
                // In some cases, the player can move the same piece twice.
                for &direction2 in ALL_DIRECTIONS {
                    if direction.reversed() == direction2 {
                        // Do not move in the opposite direction immediately.
                        continue;
                    }
                    if let Some(next2_board) = next_board.move_piece(piece, direction2) {
                        // Move the same piece once more.
                        let next2_state = State {
                            board: next2_board,
                            piece: Some(piece),
                            path: MovePath::Two(direction, direction2),
                        };
                        next_states.push(next2_state);
                    }
                }
            }
        }
    }
    next_states
}
