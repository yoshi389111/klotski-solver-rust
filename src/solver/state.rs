use super::Board;
use super::Direction;
use super::MovePath;
use super::Piece;
use super::Rule;

/// All possible directions for moving pieces in the puzzle.
static ALL_DIRECTIONS: &[Direction] = &[
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct State {
    pub board: Board,
    pub piece: Option<Piece>,
    pub path: MovePath,
}

impl State {
    /// Creates the next possible states from the current state based on the given rule.
    pub fn get_neighbors(&self, rule: &Rule) -> Vec<Self> {
        let mut next_states = vec![];
        let current_board = &self.board;
        for &piece in &rule.pieces {
            if let Some(prev_piece) = self.piece
                && prev_piece == piece
            {
                // Do not move the same piece as last time.
                continue;
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
}

#[cfg(test)]
mod tests {
    use super::super::BitPattern;
    use super::*;

    #[test]
    fn test_get_neighbors() {
        // Arrange: Test that get_neighbors does not move the same piece twice in a row
        let rule = Rule::new(
            &Board::new(0x2113_2113_4556_4786_900a),
            &BitPattern::new(0x0000_0000_0000_0ff0_0ff0),
        );
        let state = State {
            board: rule.start.clone(),
            piece: Some(rule.pieces[0]),
            path: MovePath::None,
        };

        // Act
        let neighbors = state.get_neighbors(&rule);

        // Assert
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
}
