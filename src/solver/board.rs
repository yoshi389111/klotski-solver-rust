use super::{BitPattern, Direction, Piece};

/// The board of the Klotski puzzle.
/// It has a 4x5 grid of cells, where a single hexadecimal digit represents the piece ID.
/// There are the following four types of pieces, and there are two cells where no piece is placed.
///
/// - 2x2 piece (ID: 0x1)
/// - 2x1 piece (ID: 0x2 - 0xf)
/// - 1x2 piece (ID: 0x2 - 0xf)
/// - 1x1 piece (ID: 0x2 - 0xf)
///
/// The goal of this puzzle is to move the large piece (2x2) to the position indicated by the goal mask.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Board {
    pub pattern: BitPattern,
}

impl Board {
    /// Creates a new `Board` from a 128-bit integer representation.
    pub const fn new(image: u128) -> Self {
        Self::from_bitpattern(BitPattern::new(image))
    }

    /// Creates a new `Board` from a `BitPattern`.
    pub const fn from_bitpattern(pattern: BitPattern) -> Self {
        Self { pattern }
    }

    /// Attempts to move the specified piece in the given direction.
    pub fn move_piece(&self, piece: Piece, direction: Direction) -> Option<Board> {
        let piece_mask = self.pattern.mask_of(piece);
        let edge_mask = match direction {
            Direction::Up => BitPattern::new(0xffff_0000_0000_0000_0000),
            Direction::Down => BitPattern::new(0x0000_0000_0000_0000_ffff),
            Direction::Left => BitPattern::new(0xf000_f000_f000_f000_f000),
            Direction::Right => BitPattern::new(0x000f_000f_000f_000f_000f),
        };
        if (edge_mask & piece_mask).is_not_empty() {
            // The target piece is on the edge.
            return None;
        }
        let other_pieces = self.pattern & !piece_mask;
        let moved_piece_mask = piece_mask.moved(direction);
        if (other_pieces & moved_piece_mask).is_not_empty() {
            // There is another piece in the direction of the target piece.
            return None;
        }

        let target_piece = self.pattern & piece_mask;
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
        // Arrange & Act
        let board = Board::new(0x2113_2113_4455_6789_6009);
        // Assert
        let expected_image = BitPattern::new(0x2113_2113_4455_6789_6009);
        assert_eq!(board.pattern, expected_image);
    }

    #[test]
    fn test_move_piece() {
        // Arrange#1-#3
        let board = Board::new(0x2113_2113_4455_6789_6009);

        // Act#1
        let moved_result = board.move_piece(Piece::new(8), Direction::Down);
        // Assert#1
        let expected_board = Board::new(0x2113_2113_4455_6709_6089);
        assert_eq!(moved_result, Some(expected_board));

        // Act#2
        let case_of_on_edge = board.move_piece(Piece::new(9), Direction::Right);
        // Assert#2
        assert_eq!(case_of_on_edge, None);

        // Act#3
        let case_of_overlap = board.move_piece(Piece::new(9), Direction::Left);
        // Assert#3
        assert_eq!(case_of_overlap, None);

        // Arrange#4
        let board2 = Board::new(0x2113_2113_4455_6709_6809);
        // Act#4
        let moved_result2 = board2.move_piece(Piece::new(9), Direction::Left);
        // Assert#4
        let expected_board2 = Board::new(0x2113_2113_4455_6790_6890);
        assert_eq!(moved_result2, Some(expected_board2));
    }
}
