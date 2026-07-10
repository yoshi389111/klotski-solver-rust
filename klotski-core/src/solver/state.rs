use super::{Board, Direction, Piece};

/// State enum represents the different states of the puzzle during the solving process.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum State {
    /// The initial state of the puzzle.
    Initial { board: Board },
    /// A step in which a single piece is moved.
    SingleStep {
        board: Board,
        piece: Piece,
        direction: Direction,
    },
    /// A step in which a single piece is moved twice in succession.
    DoubleStep {
        board: Board,
        piece: Piece,
        first_direction: Direction,
        second_direction: Direction,
    },
}

impl State {
    /// Returns a reference to the board associated with the state.
    pub fn board(&self) -> &Board {
        match self {
            State::Initial { board } => board,
            State::SingleStep { board, .. } => board,
            State::DoubleStep { board, .. } => board,
        }
    }

    /// Returns the piece that was moved in the state, if applicable.
    pub fn piece(&self) -> Option<Piece> {
        match self {
            State::Initial { .. } => None,
            State::SingleStep { piece, .. } => Some(*piece),
            State::DoubleStep { piece, .. } => Some(*piece),
        }
    }
}
