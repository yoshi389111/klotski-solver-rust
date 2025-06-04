use super::super::direction::Direction;
use super::super::piece::Piece;

/// A bit pattern representing the state of a board in a puzzle game.
///
/// `BitPattern` represents a 4x5 board or a mask for bitwise operations,
/// where each cell is encoded as a 4-bit value within a 20-cell (4x5) grid.
/// It is used to store the state of the board, the shape of pieces,
/// or bitmasks for various operations in the puzzle solver.
#[derive(Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash, Debug)]
pub struct BitPattern {
    pattern: u128,
}

const BIT_PATTERN_MASK: u128 = 0xffff_ffff_ffff_ffff_ffff;

// --- Implementation ---

impl BitPattern {
    /// Creates a new `BitPattern` from a 128-bit unsigned integer.
    pub const fn new(image: u128) -> Self {
        Self { pattern: image }
    }

    /// Returns the 128-bit unsigned integer representation of the bit pattern.
    pub fn get_u128(&self) -> u128 {
        self.pattern
    }

    /// Checks if the bit pattern is empty.
    pub fn is_empty(&self) -> bool {
        self.pattern == 0
    }

    /// Checks if the bit pattern is not empty.
    pub fn is_not_empty(&self) -> bool {
        !self.is_empty()
    }

    /// Moves the bit pattern in the specified direction.
    pub fn moved(&self, direction: Direction) -> Self {
        match direction {
            Direction::Up => self.moved_up(),
            Direction::Down => self.moved_down(),
            Direction::Left => self.moved_left(),
            Direction::Right => self.moved_right(),
        }
    }

    fn moved_up(&self) -> Self {
        Self {
            pattern: (self.pattern << 16) & BIT_PATTERN_MASK,
        }
    }

    fn moved_down(&self) -> Self {
        Self::new((self.pattern >> 16) & BIT_PATTERN_MASK)
    }

    fn moved_left(&self) -> Self {
        Self::new((self.pattern << 4) & 0xfff0_fff0_fff0_fff0_fff0)
    }

    fn moved_right(&self) -> Self {
        Self::new((self.pattern >> 4) & 0x0fff_0fff_0fff_0fff_0fff)
    }

    /// Mirrors the bit pattern by swapping each piece.
    pub fn mirrored(&self) -> Self {
        let pattern = (self.pattern << 12) & 0xf000_f000_f000_f000_f000
            | (self.pattern << 4) & 0x0f00_0f00_0f00_0f00_0f00
            | (self.pattern >> 4) & 0x00f0_00f0_00f0_00f0_00f0
            | (self.pattern >> 12) & 0x000f_000f_000f_000f_000f;
        Self::new(pattern)
    }

    /// Symmetrizes the bit pattern by swapping pairs of pieces.
    pub fn symmetrized(&self, pairs: &Vec<(Piece, Piece)>) -> BitPattern {
        let mut new_pattern = self.pattern;
        for &(piece_a, piece_b) in pairs {
            let swap_pattern = (piece_a.id ^ piece_b.id) as u128 * 0x1111_1111_1111_1111_1111;
            let mask_a = self.mask_of_u128(piece_a);
            let mask_b = self.mask_of_u128(piece_b);
            new_pattern ^= (mask_a | mask_b) & swap_pattern;
        }
        Self::new(new_pattern)
    }

    /// Returns a bit pattern representing the area occupied by the given piece.
    pub fn mask_of(&self, piece: Piece) -> Self {
        Self::new(self.mask_of_u128(piece))
    }

    /// Returns a bit pattern representing the area occupied by the given piece.
    fn mask_of_u128(&self, piece: Piece) -> u128 {
        let mut mask = self.pattern;
        mask ^= (piece.id as u128) * 0x1111_1111_1111_1111_1111;
        mask = ((mask >> 1) | mask) & 0x5555_5555_5555_5555_5555;
        mask = ((mask >> 2) | mask) & 0x1111_1111_1111_1111_1111;
        mask |= mask << 1;
        mask |= mask << 2;
        (!mask) & BIT_PATTERN_MASK
    }
}

// --- Operator Implementations ---

impl std::ops::BitAnd for BitPattern {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self::new(self.pattern & rhs.pattern)
    }
}

impl std::ops::BitOr for BitPattern {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self::new(self.pattern | rhs.pattern)
    }
}

impl std::ops::BitXor for BitPattern {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self::new(self.pattern ^ rhs.pattern)
    }
}

impl std::ops::Not for BitPattern {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self::new((!self.pattern) & BIT_PATTERN_MASK)
    }
}

impl std::fmt::Display for BitPattern {
    /// Formats the `BitPattern` as a hexadecimal string with underscores between rows.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let hex20 = format!("{:0>20x}", self.pattern & BIT_PATTERN_MASK);
        write!(
            f,
            "[{}_{}_{}_{}_{}]",
            &hex20[0..4],
            &hex20[4..8],
            &hex20[8..12],
            &hex20[12..16],
            &hex20[16..20]
        )
    }
}

// --- Tests ---

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_eq_hex {
        ($left:expr, $right:expr) => {
            if $left != $right {
                panic!(
                    "Assertion failed at {}:{}:{}\n  Left:  {:X?}\n  Right: {:X?}",
                    file!(),
                    line!(),
                    column!(),
                    $left,
                    $right
                );
            }
        };
    }

    #[test]
    fn from_u128_should_create_correct_pattern() {
        let bit_pattern = BitPattern::new(0x2113_2113_4455_6789_6009);
        assert_eq_hex!(&bit_pattern.pattern, &0x2113_2113_4455_6789_6009u128);
    }

    #[test]
    fn get_u128_should_return_original_value() {
        let bit_pattern = BitPattern::new(0x2113_2113_4455_6789_6009);
        let expected_u128 = 0x2113_2113_4455_6789_6009u128;
        assert_eq_hex!(bit_pattern.get_u128(), expected_u128);
    }

    #[test]
    fn is_empty_and_is_not_empty_should_work() {
        let bit_pattern = BitPattern::new(0x0000_0000_0000_0000_0000);
        assert_eq_hex!(&bit_pattern.pattern, &0x0000_0000_0000_0000_0000u128);
        assert!(bit_pattern.is_empty());
        assert!(!bit_pattern.is_not_empty());

        let non_empty = BitPattern::new(0x0001_0000_0000_0000_0000);
        assert_eq_hex!(&non_empty.pattern, &0x0001_0000_0000_0000_0000u128);
        assert!(!non_empty.is_empty());
        assert!(non_empty.is_not_empty());
    }

    #[test]
    fn bitwise_operations_should_work() {
        let bit_pattern = BitPattern::new(0x2113_2113_4455_6789_6009);
        let bit_mask = BitPattern::new(0xffff_0000_ffff_0000_ffff);
        let and_result = bit_pattern & bit_mask;
        assert_eq_hex!(&and_result.pattern, &0x2113_0000_4455_0000_6009u128);

        let or_result = bit_pattern | bit_mask;
        assert_eq_hex!(&or_result.pattern, &0xffff_2113_ffff_6789_ffffu128);

        let xor_result = bit_pattern ^ bit_mask;
        assert_eq_hex!(&xor_result.pattern, &0xdeec_2113_bbaa_6789_9ff6u128);

        let not_result = !bit_pattern;
        assert_eq_hex!(&not_result.pattern, &0xdeec_deec_bbaa_9876_9ff6u128);
    }

    #[test]
    fn moved_should_shift_pattern_correctly() {
        let bit_pattern = BitPattern::new(0x2113_2113_4455_6789_6009);

        assert_eq!(
            bit_pattern.moved(Direction::Up),
            BitPattern::new(0x2113_4455_6789_6009_0000)
        );
        assert_eq!(
            bit_pattern.moved(Direction::Down),
            BitPattern::new(0x0000_2113_2113_4455_6789)
        );
        assert_eq!(
            bit_pattern.moved(Direction::Left),
            BitPattern::new(0x1130_1130_4550_7890_0090)
        );
        assert_eq!(
            bit_pattern.moved(Direction::Right),
            BitPattern::new(0x0211_0211_0445_0678_0600)
        );
    }

    #[test]
    fn mirrored_should_reverse_each_row() {
        let bit_pattern = BitPattern::new(0x2113_2113_4455_6789_6009);

        assert_eq!(
            bit_pattern.mirrored(),
            BitPattern::new(0x3112_3112_5544_9876_9006)
        );
    }

    #[test]
    fn symmetrized_should_swap_pairs() {
        let bit_pattern = BitPattern::new(0x2113_2113_4455_6789_6009);
        let pairs: Vec<(Piece, Piece)> = vec![
            (Piece::new(2), Piece::new(3)),
            (Piece::new(4), Piece::new(5)),
            (Piece::new(6), Piece::new(9)),
            (Piece::new(7), Piece::new(8)),
        ];

        let swapped_bitpattern = bit_pattern.symmetrized(&pairs);
        let expected_bitpattern = BitPattern::new(0x3112_3112_5544_9876_9006);

        assert_eq!(swapped_bitpattern, expected_bitpattern);
    }

    #[test]
    fn mask_of_should_return_piece_mask() {
        let bit_pattern = BitPattern::new(0x2113_2113_4455_6789_6009);

        assert_eq!(
            bit_pattern.mask_of(Piece::new(1)),
            BitPattern::new(0x0ff0_0ff0_0000_0000_0000)
        );
        assert_eq!(
            bit_pattern.mask_of(Piece::new(2)),
            BitPattern::new(0xf000_f000_0000_0000_0000)
        );
        assert_eq!(
            bit_pattern.mask_of(Piece::new(3)),
            BitPattern::new(0x000f_000f_0000_0000_0000)
        );
        assert_eq!(
            bit_pattern.mask_of(Piece::new(4)),
            BitPattern::new(0x0000_0000_ff00_0000_0000)
        );
    }

    #[test]
    fn display_should_format_as_hex_string() {
        let bit_pattern = BitPattern::new(0x2113_2113_4455_6789_6009);
        let displayed = format!("{}", bit_pattern);
        assert_eq!(displayed, "[2113_2113_4455_6789_6009]");
    }

    #[test]
    fn moved_should_return_empty_when_all_zero() {
        let bit_pattern = BitPattern::new(0x0000_0000_0000_0000_0000);
        assert_eq!(
            bit_pattern.moved(Direction::Up),
            BitPattern::new(0x0000_0000_0000_0000_0000)
        );
        assert_eq!(
            bit_pattern.moved(Direction::Down),
            BitPattern::new(0x0000_0000_0000_0000_0000)
        );
        assert_eq!(
            bit_pattern.moved(Direction::Left),
            BitPattern::new(0x0000_0000_0000_0000_0000)
        );
        assert_eq!(
            bit_pattern.moved(Direction::Right),
            BitPattern::new(0x0000_0000_0000_0000_0000)
        );
    }

    #[test]
    fn mask_of_should_return_empty_for_nonexistent_piece() {
        let bit_pattern = BitPattern::new(0x2113_2113_4455_6789_6009);
        // Piece 0 does not exist in this pattern
        assert_eq!(
            bit_pattern.mask_of(Piece::new(0xa)),
            BitPattern::new(0x0000_0000_0000_0000_0000)
        );
    }
}
