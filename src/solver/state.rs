use super::board::Board;
use super::move_path::MovePath;
use super::piece::Piece;

#[derive(Clone, Debug)]
pub struct State {
    pub board: Board,
    pub piece: Option<Piece>,
    pub path: MovePath,
}
