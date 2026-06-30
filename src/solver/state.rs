use super::Board;
use super::MovePath;
use super::Piece;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct State {
    pub board: Board,
    pub piece: Option<Piece>,
    pub path: MovePath,
}
