use super::BitPattern;
use super::Board;
use super::Piece;
use std::collections::HashMap;

/// Rule struct holds the puzzle's initial state, piece list, symmetry pairs, and goal mask.
#[derive(Debug)]
pub struct Rule {
    pub start: Board,
    pub pieces: Vec<Piece>,
    pub pairs: Vec<(Piece, Piece)>,
    pub goal_mask: BitPattern,
}

#[derive(Debug)]
pub enum RuleError {
    InvalidStartBoardHexLength,
    StartBoardInvalidEmptySpaceCount,
    FirstPieceMissingInStartBoard,
    InvalidPieceShape,
    InvalidGoalMaskHexLength,
    GoalMaskSizeMismatch,
    GoalMaskShapeError,
}

const SHAPE_LARGE: BitPattern = BitPattern::new(0x00ff_00ff);

impl Rule {
    pub fn parse(start_image: &str, goal_mask: &str) -> Result<Self, RuleError> {
        let start_image =
            parse_20_hex_digits(start_image).ok_or(RuleError::InvalidStartBoardHexLength)?;

        if count_empty_spaces(&start_image) != 2 {
            return Err(RuleError::StartBoardInvalidEmptySpaceCount);
        }

        if piece_shape(&start_image, 1) != SHAPE_LARGE {
            return Err(RuleError::FirstPieceMissingInStartBoard);
        }

        for i in 0x2u8..=0xf {
            let shape = piece_shape(&start_image, i);
            if !is_valid_regular_piece_shapes(&shape) {
                return Err(RuleError::InvalidPieceShape);
            }
        }

        let goal_mask =
            parse_20_hex_digits(goal_mask).ok_or(RuleError::InvalidGoalMaskHexLength)?;

        if count_empty_spaces(&goal_mask) != 16 {
            return Err(RuleError::GoalMaskSizeMismatch);
        }

        if piece_shape(&goal_mask, 0xf) != SHAPE_LARGE {
            return Err(RuleError::GoalMaskShapeError);
        }

        let rule = Self::new(&Board::from_bitpattern(start_image), &goal_mask);

        Ok(rule)
    }

    /// Create a new Rule from the start board and goal mask.
    pub fn new(start_board: &Board, goal_mask: &BitPattern) -> Self {
        let pieces = Self::create_pieces(start_board);
        let pairs = Self::create_pairs(start_board, goal_mask, &pieces);
        Self {
            start: start_board.clone(),
            pieces,
            pairs,
            goal_mask: *goal_mask,
        }
    }

    /// Returns true if the board's target piece matches the goal mask.
    pub fn is_finished(&self, board: &Board) -> bool {
        board.pattern.mask_of(Piece::new(1)) == self.goal_mask
    }

    /// Collect all pieces present in the starting board.
    fn create_pieces(start_board: &Board) -> Vec<Piece> {
        // Collect all pieces that are present in the starting board.
        (0x1u8..=0xf)
            .map(Piece::new)
            .filter(|&p| start_board.pattern.mask_of(p).is_not_empty())
            .collect::<Vec<Piece>>()
    }

    /// Creates pairs of pieces based on their mirrored masks.
    fn create_pairs(
        board: &Board,
        goal_mask: &BitPattern,
        pieces: &[Piece],
    ) -> Vec<(Piece, Piece)> {
        if *goal_mask != goal_mask.mirrored() {
            // Asymmetric goal mask: no symmetry pairs.
            return vec![];
        }

        // Map each piece to its mask.
        let piece_to_mask = pieces
            .iter()
            .map(|&p| (p, board.pattern.mask_of(p)))
            .collect::<HashMap<_, _>>();

        // Map mirrored masks to pieces.
        let mirrored_to_piece = piece_to_mask
            .iter()
            .map(|(&p, m)| (m.mirrored(), p))
            .collect::<HashMap<_, _>>();

        // Check if all pieces have a symmetric counterpart.
        let all_pieces_symmetric = piece_to_mask
            .values()
            .all(|m| mirrored_to_piece.contains_key(m));

        if !all_pieces_symmetric {
            // Asymmetric pieces: no symmetry pairs.
            return vec![];
        }

        // Collect unique pairs (p, q) where p < q.
        piece_to_mask
            .iter()
            .map(|(&p, m)| (p, *mirrored_to_piece.get(m).unwrap()))
            .filter(|(p, q)| p < q)
            .collect::<Vec<(_, _)>>()
    }
}

/// Parses a string representing a 20 hex digit number, allowing for underscores as separators.
fn parse_20_hex_digits(value: &str) -> Option<BitPattern> {
    let value = value.trim_start_matches("0x").replace('_', "");
    match u128::from_str_radix(&value, 16) {
        Ok(n) if n <= 0xffff_ffff_ffff_ffff_ffff => Some(BitPattern::new(n)),
        _ => None,
    }
}

/// Returns the shape of the specified piece in the bit pattern.
fn piece_shape(bit_pattern: &BitPattern, piece_id: u8) -> BitPattern {
    let piece_mask: u128 = bit_pattern.mask_of(Piece::new(piece_id)).get_u128();
    let piece_shape = match piece_mask {
        0 => 0,
        _ => piece_mask >> piece_mask.trailing_zeros(),
    };
    BitPattern::new(piece_shape)
}

const SHAPE_UNUSED: BitPattern = BitPattern::new(0x0000_0000);
const SHAPE_SMALL: BitPattern = BitPattern::new(0x0000_000f);
const SHAPE_HORIZONTAL: BitPattern = BitPattern::new(0x0000_00ff);
const SHAPE_VERTICAL: BitPattern = BitPattern::new(0x000f_000f);

/// Checks if the given shape is a valid regular piece shape.
fn is_valid_regular_piece_shapes(shape: &BitPattern) -> bool {
    matches!(
        *shape,
        SHAPE_UNUSED | SHAPE_SMALL | SHAPE_HORIZONTAL | SHAPE_VERTICAL
    )
}

/// Counts the number of empty spaces in the given bit pattern.
fn count_empty_spaces(bit_pattern: &BitPattern) -> usize {
    let mut value = bit_pattern.get_u128();
    let mut count = 0;
    for _ in 0..20 {
        if (value & 0xf) == 0 {
            count += 1;
        }
        value >>= 4;
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_and_invalid() {
        // Arrange
        let start = "0x2113_2113_4556_4786_900a";
        let goal = "0x0000_0000_0000_0ff0_0ff0";
        // Act
        let result = Rule::parse(start, goal);
        // Assert
        assert!(result.is_ok());

        // Arrange: Invalid start
        let bad_start = "0x0000_0000_0000_0000_0000";
        // Act
        let result = Rule::parse(bad_start, goal);
        // Assert
        assert!(result.is_err());

        // Arrange: Invalid goal
        let bad_goal = "0x0000_0000_0000_0000_0001";
        // Act
        let result = Rule::parse(start, bad_goal);
        // Assert
        assert!(result.is_err());
    }

    #[test]
    fn rule_new_should_initialize_fields() {
        // Arrange & Act
        let rule = Rule::new(
            &Board::new(0x2113_2113_4556_4786_900a),
            &BitPattern::new(0x0000_0000_0000_0ff0_0ff0),
        );

        // Assert#1
        let expected_start_board = Board::new(0x2113_2113_4556_4786_900a);
        assert_eq!(rule.start, expected_start_board);

        // Assert#2
        let expected_pieces = vec![
            Piece::new(1),
            Piece::new(2),
            Piece::new(3),
            Piece::new(4),
            Piece::new(5),
            Piece::new(6),
            Piece::new(7),
            Piece::new(8),
            Piece::new(9),
            Piece::new(10),
        ];
        assert_eq!(rule.pieces, expected_pieces);

        // Assert#3
        let mut sorted_pairs = rule.pairs.clone();
        sorted_pairs.sort();
        let expected_sorted_pairs = vec![
            (Piece::new(2), Piece::new(3)),
            (Piece::new(4), Piece::new(6)),
            (Piece::new(7), Piece::new(8)),
            (Piece::new(9), Piece::new(10)),
        ];
        assert_eq!(sorted_pairs, expected_sorted_pairs);

        // Assert#4
        assert!(rule.is_finished(&Board::new(0x2003_2783_4455_6119_6119)));
    }

    #[test]
    fn rule_new_should_handle_asymmetric_goal() {
        // Arrange & Act
        let rule = Rule::new(
            &Board::new(0x2113_2113_4556_7896_700a),
            &BitPattern::new(0x0000_0000_0000_0ff0_0ff0),
        );

        // Assert#1
        let expected_start_board = Board::new(0x2113_2113_4556_7896_700a);
        assert_eq!(rule.start, expected_start_board);

        // Assert#2
        let expected_pieces = vec![
            Piece::new(1),
            Piece::new(2),
            Piece::new(3),
            Piece::new(4),
            Piece::new(5),
            Piece::new(6),
            Piece::new(7),
            Piece::new(8),
            Piece::new(9),
            Piece::new(10),
        ];
        assert_eq!(rule.pieces, expected_pieces);

        // Assert#3
        let expected_pairs = vec![];
        assert_eq!(rule.pairs, expected_pairs);
    }

    #[test]
    fn is_finished_should_return_true_for_goal() {
        // Arrange
        let rule = Rule::new(
            &Board::new(0x2113_2113_4556_4786_900a),
            &BitPattern::new(0x0ff0_0ff0_0000_0000_0000),
        );
        let goal_board = Board::new(0x2113_2113_4556_4786_900a);

        // Act & Assert
        assert!(rule.is_finished(&goal_board));
    }

    #[test]
    fn create_pieces_should_return_empty_for_empty_board() {
        // Arrange
        let empty_board = Board::new(0x0000_0000_0000_0000_0000);
        // Act
        let pieces = Rule::create_pieces(&empty_board);
        // Assert
        assert_eq!(pieces, vec![]);
    }

    #[test]
    fn create_pairs_should_return_empty_for_asymmetric_goal() {
        // Arrange
        let board = Board::new(0x2113_2113_4556_4786_900a);
        let asymmetric_goal_mask = BitPattern::new(0x0000_0000_0000_00ff_00ff);
        let pieces = Rule::create_pieces(&board);
        // Act
        let pairs = Rule::create_pairs(&board, &asymmetric_goal_mask, &pieces);
        // Assert
        assert_eq!(pairs, vec![]);
    }

    #[test]
    fn test_parse_20_hex_digits_valid() {
        // Arrange: Valid 20-digit hex string with underscores
        let s = "0x1234_5678_9abc_def0_1234";
        // Act
        let pat = parse_20_hex_digits(s);
        // Assert
        assert_eq!(pat, Some(BitPattern::new(0x1234_5678_9abc_def0_1234)));
    }

    #[test]
    fn test_parse_20_hex_digits_invalid() {
        // Arrange: Invalid: 21 digits
        let s = "0x1234_5678_9abc_def0_12345";
        // Act & Assert
        assert_eq!(parse_20_hex_digits(s), None);

        // Arrange: Invalid: Non-hex characters
        let s = "0x1234_5678_9abc_defg_1234";
        // Act & Assert
        assert_eq!(parse_20_hex_digits(s), None);
    }

    #[test]
    fn test_count_empty_spaces() {
        // Arrange: Two empty spaces
        let pat = BitPattern::new(0x2113_2113_4556_4786_900a);
        // Act & Assert
        assert_eq!(count_empty_spaces(&pat), 2);

        // Arrange: All empty
        let pat = BitPattern::new(0);
        // Act & Assert
        assert_eq!(count_empty_spaces(&pat), 20);

        // Arrange: No empty
        let pat = BitPattern::new(0x1111_1111_1111_1111_1111);
        // Act & Assert
        assert_eq!(count_empty_spaces(&pat), 0);
    }

    #[test]
    fn test_piece_shape_and_is_valid_regular_piece_shapes() {
        // Arrange: Small piece
        let pat = BitPattern::new(0x0000_000f);
        // Act & Assert
        assert!(is_valid_regular_piece_shapes(&pat));

        // Arrange: Horizontal piece
        let pat = BitPattern::new(0x0000_00ff);
        // Act & Assert
        assert!(is_valid_regular_piece_shapes(&pat));

        // Arrange: Vertical piece
        let pat = BitPattern::new(0x000f_000f);
        // Act & Assert
        assert!(is_valid_regular_piece_shapes(&pat));

        // Arrange: Unused
        let pat = BitPattern::new(0x0000_0000);
        // Act & Assert
        assert!(is_valid_regular_piece_shapes(&pat));

        // Arrange: Invalid shape
        let pat = BitPattern::new(0x0000_0fff);
        // Act & Assert
        assert!(!is_valid_regular_piece_shapes(&pat));
    }
}
