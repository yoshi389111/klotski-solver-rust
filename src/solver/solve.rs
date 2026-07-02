use super::{BitPattern, Board, Direction, Piece, Rule, State, VisitedHistory};
use crate::bfs;

/// All possible directions for moving pieces in the puzzle.
const ALL_DIRECTIONS: &[Direction] = &[
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];

/// Solves the klotski puzzle using a breadth-first search algorithm.
pub fn solve(rule: &Rule) -> Option<Vec<State>> {
    let board = rule.start.clone();
    let initial_state = State::Initial { board };

    let is_goal = |s: &State| rule.is_finished(s.board());
    let neighbors = |s: &State| get_neighbors(rule, s);

    let mut visited = VisitedHistory::new();
    let try_visit =
        |s: &State, depth: usize| visited.try_visit(BoardKey::create(rule, s.board()), depth);

    bfs::find_path(&initial_state, is_goal, neighbors, try_visit)
}

/// Creates the next possible states from the current state based on the given rule.
fn get_neighbors(rule: &Rule, state: &State) -> Vec<State> {
    let mut next_states = vec![];
    let current_board = state.board();
    for &piece in &rule.pieces {
        if let Some(last_moved_piece) = state.piece()
            && last_moved_piece == piece
        {
            // Do not move the same piece as last time.
            continue;
        }
        for &direction in ALL_DIRECTIONS {
            if let Some(next_board) = current_board.move_piece(piece, direction) {
                let first_move = make_first_move(&next_board, piece, direction);
                next_states.push(first_move);

                // There are two blank spaces on the board.
                // In some cases, the player can move the same piece twice.
                if let Some(second_move) = find_second_move(&next_board, piece, direction) {
                    next_states.push(second_move);
                }
            }
        }
    }
    next_states
}

/// Creates a new `State` representing the first move of a piece in a given direction.
fn make_first_move(board: &Board, piece: Piece, direction: Direction) -> State {
    State::SingleStep {
        board: board.clone(),
        piece,
        direction,
    }
}

/// Finds a valid second move for the same piece in a different direction, if possible.
fn find_second_move(board: &Board, piece: Piece, first_direction: Direction) -> Option<State> {
    for &second_direction in ALL_DIRECTIONS {
        if first_direction.reversed() == second_direction {
            // Do not move in the opposite direction immediately.
            continue;
        }
        if let Some(next_board) = board.move_piece(piece, second_direction) {
            return Some(State::DoubleStep {
                board: next_board,
                piece,
                first_direction,
                second_direction,
            });
        }
    }
    None
}

/// Represents a unique key for a board state, which is used to identify and compare different board configurations.
#[derive(PartialEq, Eq, Hash, Debug)]
struct BoardKey {
    key: BitPattern,
}

impl BoardKey {
    /// Creates a new `BoardKey` based on the provided rule and board.
    pub fn create(rule: &Rule, board: &Board) -> BoardKey {
        let mut key = board.pattern.min(board.pattern.mirrored());

        if !rule.pairs.is_empty() {
            // In the case of a symmetrical board at the start.
            let symmetrized = board.pattern.symmetrized(&rule.pairs);
            key = key.min(symmetrized).min(symmetrized.mirrored());
        }

        BoardKey { key }
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::*;

    #[test]
    fn test_solve_returns_none_for_unsolvable() {
        // Arrange: Test solve returns None for unsolvable puzzle
        let rule = Rule::new(
            Board::new(0x2112_2112_3344_5678_5008),
            BitPattern::new(0x0000_0000_0000_0ff0_0ff0),
        );
        // Act
        let result = solve(&rule);
        // Assert
        assert_eq!(result, None);
    }

    #[test]
    fn test_get_neighbors() {
        // Arrange: Test that get_neighbors does not move the same piece twice in a row
        let rule = Rule::new(
            Board::new(0x2113_2113_4556_4786_900a),
            BitPattern::new(0x0000_0000_0000_0ff0_0ff0),
        );
        let state = State::Initial {
            board: rule.start.clone(),
        };

        // Act
        let neighbors = get_neighbors(&rule, &state);

        // Assert
        assert_eq!(
            neighbors,
            vec![
                State::SingleStep {
                    board: Board::new(0x2113_2113_4556_4086_970a),
                    piece: Piece::new(7),
                    direction: Direction::Down,
                },
                State::DoubleStep {
                    board: Board::new(0x2113_2113_4556_4086_907a),
                    piece: Piece::new(7),
                    first_direction: Direction::Down,
                    second_direction: Direction::Right,
                },
                State::SingleStep {
                    board: Board::new(0x2113_2113_4556_4706_908a),
                    piece: Piece::new(8),
                    direction: Direction::Down,
                },
                State::DoubleStep {
                    board: Board::new(0x2113_2113_4556_4706_980a),
                    piece: Piece::new(8),
                    first_direction: Direction::Down,
                    second_direction: Direction::Left,
                },
                State::SingleStep {
                    board: Board::new(0x2113_2113_4556_4786_090a),
                    piece: Piece::new(9),
                    direction: Direction::Right,
                },
                State::DoubleStep {
                    board: Board::new(0x2113_2113_4556_4786_009a),
                    piece: Piece::new(9),
                    first_direction: Direction::Right,
                    second_direction: Direction::Right,
                },
                State::SingleStep {
                    board: Board::new(0x2113_2113_4556_4786_90a0),
                    piece: Piece::new(0xa),
                    direction: Direction::Left,
                },
                State::DoubleStep {
                    board: Board::new(0x2113_2113_4556_4786_9a00),
                    piece: Piece::new(0xa),
                    first_direction: Direction::Left,
                    second_direction: Direction::Left,
                },
            ]
        );
    }

    #[test]
    fn test_create_key() {
        // Arrange: Test BoardKey::create produces expected key
        let rule = Rule::new(
            Board::new(0x3112_3112_5544_9876_9006),
            BitPattern::new(0x0000_0000_0000_0ff0_0ff0),
        );
        // Act
        let actual_key = BoardKey::create(&rule, &rule.start);
        // Assert
        let expected_key = BoardKey {
            key: BitPattern::new(0x2113_2113_4455_6789_6009),
        };
        assert_eq!(actual_key, expected_key);
    }
}
