use super::bit_pattern::BitPattern;
use super::board::Board;
use super::piece::Piece;
use std::collections::HashMap;

/// Rule struct holds the puzzle's initial state, piece list, symmetry pairs, and goal mask.
#[derive(Debug)]
pub struct Rule {
    pub start: Board,
    pub pieces: Vec<Piece>,
    pub pairs: Vec<(Piece, Piece)>,
    pub goal_mask: BitPattern,
}

impl Rule {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rule_new_should_initialize_fields() {
        let rule = Rule::new(
            &Board::new(0x2113_2113_4556_4786_900a),
            &BitPattern::new(0x0000_0000_0000_0ff0_0ff0),
        );
        let expected_start_board = Board::new(0x2113_2113_4556_4786_900a);
        assert_eq!(rule.start, expected_start_board);

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

        let mut sorted_pairs = rule.pairs.clone();
        sorted_pairs.sort();
        let expected_sorted_pairs = vec![
            (Piece::new(2), Piece::new(3)),
            (Piece::new(4), Piece::new(6)),
            (Piece::new(7), Piece::new(8)),
            (Piece::new(9), Piece::new(10)),
        ];
        assert_eq!(sorted_pairs, expected_sorted_pairs);

        assert!(rule.is_finished(&Board::new(0x2003_2783_4455_6119_6119)));
    }

    #[test]
    fn rule_new_should_handle_asymmetric_goal() {
        let rule = Rule::new(
            &Board::new(0x2113_2113_4556_7896_700a),
            &BitPattern::new(0x0000_0000_0000_0ff0_0ff0),
        );
        let expected_start_board = Board::new(0x2113_2113_4556_7896_700a);
        assert_eq!(rule.start, expected_start_board);

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

        let expected_pairs = vec![];
        assert_eq!(rule.pairs, expected_pairs);
    }

    #[test]
    fn is_finished_should_return_true_for_goal() {
        let rule = Rule::new(
            &Board::new(0x2113_2113_4556_4786_900a),
            &BitPattern::new(0x0ff0_0ff0_0000_0000_0000),
        );
        let goal_board = Board::new(0x2113_2113_4556_4786_900a);
        assert!(rule.is_finished(&goal_board));
    }

    #[test]
    fn create_pieces_should_return_empty_for_empty_board() {
        let empty_board = Board::new(0x0000_0000_0000_0000_0000);
        let pieces = Rule::create_pieces(&empty_board);
        assert_eq!(pieces, vec![]);
    }

    #[test]
    fn create_pairs_should_return_empty_for_asymmetric_goal() {
        let board = Board::new(0x2113_2113_4556_4786_900a);
        let asymmetric_goal_mask = BitPattern::new(0x0000_0000_0000_00ff_00ff);
        let pieces = Rule::create_pieces(&board);
        let pairs = Rule::create_pairs(&board, &asymmetric_goal_mask, &pieces);
        assert_eq!(pairs, vec![]);
    }
}
