use super::bit_pattern::BitPattern;
use super::board::Board;
use super::rule::Rule;

/// Represents a unique key for a board state, which is used to identify and compare different board configurations.
#[derive(PartialEq, Eq, Hash, Debug)]
pub struct BoardKey {
    key: BitPattern,
}

impl BoardKey {
    /// Creates a new `BoardKey` based on the provided rule and board.
    pub fn create(rule: &Rule, board: &Board) -> BoardKey {
        let min_image = Self::min(board.image, board.image.mirrored());

        if rule.pairs.is_empty() {
            return BoardKey { key: min_image };
        }

        let symmetrized_image = board.image.symmetrized(&rule.pairs);
        let min_image = Self::min(min_image, symmetrized_image);

        let symmetrized_mirrored = symmetrized_image.mirrored();
        let min_image = Self::min(min_image, symmetrized_mirrored);

        BoardKey { key: min_image }
    }

    fn min(a: BitPattern, b: BitPattern) -> BitPattern {
        if b < a { b } else { a }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_key() {
        let rule = Rule::new(
            &Board::new(0x3112_3112_5544_9876_9006),
            &BitPattern::new(0x0000_0000_0000_0ff0_0ff0),
        );
        let actual_key = BoardKey::create(&rule, &rule.start);
        let expected_key = BoardKey {
            key: BitPattern::new(0x2113_2113_4455_6789_6009),
        };
        assert_eq!(actual_key, expected_key);
    }
}
