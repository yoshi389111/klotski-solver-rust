use super::bit_pattern::BitPattern;
use super::direction::Direction;
use super::piece::Piece;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Board {
    pub image: BitPattern,
}

static EDGE_TOP: BitPattern = BitPattern::new(0xffff_0000_0000_0000_0000);
static EDGE_BOTTOM: BitPattern = BitPattern::new(0x0000_0000_0000_0000_ffff);
static EDGE_LEFT: BitPattern = BitPattern::new(0xf000_f000_f000_f000_f000);
static EDGE_RIGHT: BitPattern = BitPattern::new(0x000f_000f_000f_000f_000f);

impl Board {
    /// Creates a new `Board` from a 128-bit integer representation.
    pub const fn new(input: u128) -> Self {
        Self::from_bitpattern(BitPattern::new(input))
    }

    /// Creates a new `Board` from a `BitPattern`.
    pub const fn from_bitpattern(image: BitPattern) -> Self {
        Self { image }
    }

    /// Attempts to move the specified piece in the given direction.
    pub fn move_piece(&self, piece: Piece, direction: Direction) -> Option<Board> {
        let piece_mask = self.image.mask_of(piece);
        let edge_mask = match direction {
            Direction::Up => EDGE_TOP,
            Direction::Down => EDGE_BOTTOM,
            Direction::Left => EDGE_LEFT,
            Direction::Right => EDGE_RIGHT,
        };
        if (edge_mask & piece_mask).is_not_empty() {
            // The target piece is on the edge.
            return None;
        }
        let other_pieces = self.image & !piece_mask;
        let moved_piece_mask = piece_mask.moved(direction);
        if (other_pieces & moved_piece_mask).is_not_empty() {
            // There is another piece in the direction of the target piece.
            return None;
        }

        let target_piece = self.image & piece_mask;
        let moved_target_piece = target_piece.moved(direction);
        let next_board = Board::from_bitpattern(other_pieces | moved_target_piece);
        Some(next_board)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_board() {
        let board = Board::new(0x2113_2113_4455_6789_6009);
        let expected_image = BitPattern::new(0x2113_2113_4455_6789_6009);
        assert_eq!(board.image, expected_image);
    }

    #[test]
    fn test_move_piece() {
        let board = Board::new(0x2113_2113_4455_6789_6009);

        let moved_result = board.move_piece(Piece::new(8), Direction::Down);
        let expected_board = Board::new(0x2113_2113_4455_6709_6089);
        assert_eq!(moved_result, Some(expected_board));

        let case_of_on_edge = board.move_piece(Piece::new(9), Direction::Right);
        assert_eq!(case_of_on_edge, None);

        let case_of_overlap = board.move_piece(Piece::new(9), Direction::Left);
        assert_eq!(case_of_overlap, None);

        let board2 = Board::new(0x2113_2113_4455_6709_6809);
        let moved_result2 = board2.move_piece(Piece::new(9), Direction::Left);
        let expected_board2 = Board::new(0x2113_2113_4455_6790_6890);
        assert_eq!(moved_result2, Some(expected_board2));
    }
}
