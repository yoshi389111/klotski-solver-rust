use super::bit_pattern::BitPattern;
use super::board::Board;
use super::piece::Piece;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Rule {
    pub start: Board,
    pub pieces: Vec<Piece>,
    pub pairs: Vec<(Piece, Piece)>,
    pub goal_mask: BitPattern,
}

impl Rule {
    /// Creates a new `Rule` instance based on the provided starting board, goal mask, and target piece.
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

    /// Checks if the rule is finished by comparing the board's target piece mask with the goal mask.
    pub fn is_finished(&self, board: &Board) -> bool {
        board.image.mask_of(Piece::new(1)) == self.goal_mask
    }

    /// Creates a list of pieces that are present in the starting board.
    fn create_pieces(start_board: &Board) -> Vec<Piece> {
        // Collect all pieces that are present in the starting board.
        (0x1u8..=0xf)
            .map(Piece::new)
            .filter(|&p| start_board.image.mask_of(p).is_not_empty())
            .collect::<Vec<Piece>>()
    }

    /// Creates pairs of pieces based on their mirrored masks.
    fn create_pairs(
        board: &Board,
        goal_mask: &BitPattern,
        pieces: &[Piece],
    ) -> Vec<(Piece, Piece)> {
        if *goal_mask != goal_mask.mirrored() {
            // The goal mask is asymmetrical.
            return vec![];
        }

        // Mapping of pieces and piece masks.
        let piece_to_mask = pieces
            .iter()
            .map(|&p| (p, board.image.mask_of(p)))
            .collect::<HashMap<_, _>>();

        // Mirrored piece masks and piece mapping.
        let mirrored_to_piece = piece_to_mask
            .iter()
            .map(|(&p, m)| (m.mirrored(), p))
            .collect::<HashMap<_, _>>();

        // True if every piece has a mirror image.
        let all_pieces_symmetric = piece_to_mask
            .values()
            .all(|m| mirrored_to_piece.contains_key(m));

        if !all_pieces_symmetric {
            // It was asymmetrical.
            return vec![];
        }

        // List of symmetrical pieces.
        let pairs = piece_to_mask
            .iter()
            .map(|(&p, m)| (p, *mirrored_to_piece.get(m).unwrap()))
            .filter(|(p, q)| p < q)
            .collect::<Vec<(_, _)>>();
        pairs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rule() {
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

        let mut sorted_paris = rule.pairs.clone();
        sorted_paris.sort();
        let expected_sorted_pairs = vec![
            (Piece::new(2), Piece::new(3)),
            (Piece::new(4), Piece::new(6)),
            (Piece::new(7), Piece::new(8)),
            (Piece::new(9), Piece::new(10)),
        ];
        assert_eq!(sorted_paris, expected_sorted_pairs);

        assert!(rule.is_finished(&Board::new(0x2003_2783_4455_6119_6119)));
    }

    #[test]
    fn test_rule2() {
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
}
