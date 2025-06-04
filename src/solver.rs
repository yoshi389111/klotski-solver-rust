pub mod bit_pattern;
pub mod board;
pub mod direction;
pub mod piece;
pub mod rule;
mod visited_history;

use super::bfs;
use bit_pattern::BitPattern;
use board::Board;
use direction::Direction;
use piece::Piece;
use rule::Rule;
use visited_history::VisitedHistory;

// --- Structs and Enums ---

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct State {
    pub board: Board,
    pub piece: Option<Piece>,
    pub path: MovePath,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum MovePath {
    None,
    One(Direction),
    Two(Direction, Direction),
}

impl std::fmt::Display for MovePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MovePath::None => write!(f, "None"),
            MovePath::One(d) => write!(f, "{d}"),
            MovePath::Two(d1, d2) => write!(f, "{d1} and {d2}"),
        }
    }
}

/// Represents a unique key for a board state, which is used to identify and compare different board configurations.
#[derive(PartialEq, Eq, Hash, Debug)]
pub struct BoardKey {
    key: BitPattern,
}

impl BoardKey {
    /// Creates a new `BoardKey` based on the provided rule and board.
    pub fn create(rule: &Rule, board: &Board) -> BoardKey {
        let min_image = Self::min(board.pattern, board.pattern.mirrored());

        if rule.pairs.is_empty() {
            return BoardKey { key: min_image };
        }

        let symmetrized_image = board.pattern.symmetrized(&rule.pairs);
        let min_image = Self::min(min_image, symmetrized_image);

        let symmetrized_mirrored = symmetrized_image.mirrored();
        let min_image = Self::min(min_image, symmetrized_mirrored);

        BoardKey { key: min_image }
    }

    fn min(a: BitPattern, b: BitPattern) -> BitPattern {
        if b < a { b } else { a }
    }
}

// --- Constants ---

/// All possible directions for moving pieces in the puzzle.
static ALL_DIRECTIONS: &[Direction] = &[
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];

// --- Functions ---

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

// --- Tests ---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_key() {
        // Test BoardKey::create produces expected key
        let rule = Rule::new(
            &Board::new(0x3112_3112_5544_9876_9006),
            &BitPattern::new(0x0000_0000_0000_0ff0_0ff0),
        );
        let actual_key = BoardKey::create(&rule, &rule.start);
        let expected_key = BoardKey {
            key: BitPattern::new(0x2113_2113_4455_6789_6009),
        };
        assert_eq!(actual_key, expected_key);
    }

    #[test]
    fn test_move_path_display() {
        // Test MovePath Display implementation
        let up = Direction::Up;
        let down = Direction::Down;
        let path_none = MovePath::None;
        let path_one = MovePath::One(up);
        let path_two = MovePath::Two(up, down);
        assert_eq!(format!("{path_none}"), "None");
        assert_eq!(format!("{path_one}"), format!("{up}"));
        assert_eq!(format!("{path_two}"), format!("{up} and {down}"));
    }

    #[test]
    fn test_get_neighbors() {
        // Test that get_neighbors does not move the same piece twice in a row
        let rule = Rule::new(
            &Board::new(0x2113_2113_4556_4786_900a),
            &BitPattern::new(0x0000_0000_0000_0ff0_0ff0),
        );
        let state = State {
            board: rule.start.clone(),
            piece: Some(rule.pieces[0]),
            path: MovePath::None,
        };
        let neighbors = super::get_neighbors(&rule, &state);
        assert_eq!(
            neighbors,
            vec![
                State {
                    board: Board::new(0x2113_2113_4556_4086_970a),
                    piece: Some(Piece::new(7)),
                    path: MovePath::One(Direction::Down),
                },
                State {
                    board: Board::new(0x2113_2113_4556_4086_907a),
                    piece: Some(Piece::new(7)),
                    path: MovePath::Two(Direction::Down, Direction::Right),
                },
                State {
                    board: Board::new(0x2113_2113_4556_4706_908a),
                    piece: Some(Piece::new(8)),
                    path: MovePath::One(Direction::Down),
                },
                State {
                    board: Board::new(0x2113_2113_4556_4706_980a),
                    piece: Some(Piece::new(8)),
                    path: MovePath::Two(Direction::Down, Direction::Left),
                },
                State {
                    board: Board::new(0x2113_2113_4556_4786_090a),
                    piece: Some(Piece::new(9)),
                    path: MovePath::One(Direction::Right),
                },
                State {
                    board: Board::new(0x2113_2113_4556_4786_009a),
                    piece: Some(Piece::new(9)),
                    path: MovePath::Two(Direction::Right, Direction::Right),
                },
                State {
                    board: Board::new(0x2113_2113_4556_4786_90a0),
                    piece: Some(Piece::new(0xa)),
                    path: MovePath::One(Direction::Left),
                },
                State {
                    board: Board::new(0x2113_2113_4556_4786_9a00),
                    piece: Some(Piece::new(0xa)),
                    path: MovePath::Two(Direction::Left, Direction::Left),
                },
            ]
        );
    }

    #[test]
    fn test_solve_returns_none_for_unsolvable() {
        // Test solve returns None for unsolvable puzzle
        let rule = Rule::new(
            &Board::new(0x2112_2112_3344_5678_5008),
            &BitPattern::new(0x0000_0000_0000_0ff0_0ff0),
        );
        let result = super::solve(&rule);
        assert_eq!(result, None);
    }
}
