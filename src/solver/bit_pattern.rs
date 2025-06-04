use super::direction::Direction;
use super::piece::Piece;

/// The size of the bit pattern (number of rows).
const SIZE: usize = 5;

/// A bit pattern representing the state of a board in a puzzle game.
///
/// `BitPattern` represents a 4x5 board or a mask for bitwise operations,
/// where each cell is encoded as a 4-bit value within a 20-cell (4x5) grid.
/// It is used to store the state of the board, the shape of pieces,
/// or bitmasks for various operations in the puzzle solver.
#[derive(Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash, Debug)]
pub struct BitPattern {
    array: [u16; SIZE],
}

// --- Implementation ---

impl BitPattern {
    /// Creates a new `BitPattern` from a 128-bit unsigned integer.
    pub const fn new(image: u128) -> Self {
        Self::from_u16_array([
            (image >> 64) as u16,
            (image >> 48) as u16,
            (image >> 32) as u16,
            (image >> 16) as u16,
            image as u16,
        ])
    }

    const fn from_u16_array(array: [u16; SIZE]) -> Self {
        Self { array }
    }

    /// Returns the 128-bit unsigned integer representation of the bit pattern.
    pub fn get_u128(&self) -> u128 {
        self.array
            .iter()
            .fold(0u128, |acc, &cur| (acc << 16) | (cur as u128))
    }

    /// Checks if the bit pattern is empty.
    pub fn is_empty(&self) -> bool {
        self.array.iter().all(|&v| v == 0)
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
        let mut new_array = [0; SIZE];
        new_array[..(SIZE - 1)].copy_from_slice(&self.array[1..]);
        Self::from_u16_array(new_array)
    }

    fn moved_down(&self) -> Self {
        let mut new_array = [0; SIZE];
        new_array[1..].copy_from_slice(&self.array[..(SIZE - 1)]);
        Self::from_u16_array(new_array)
    }

    fn moved_left(&self) -> Self {
        let mut new_array = self.array;
        for m in new_array.iter_mut() {
            *m <<= 4;
        }
        Self::from_u16_array(new_array)
    }

    fn moved_right(&self) -> Self {
        let mut new_array = self.array;
        for m in new_array.iter_mut() {
            *m >>= 4;
        }
        Self::from_u16_array(new_array)
    }

    /// Mirrors the bit pattern by swapping each piece.
    pub fn mirrored(&self) -> Self {
        let mut new_array = [0; SIZE];
        for (m, v) in new_array.iter_mut().zip(self.array.iter()) {
            *m = Self::mirrored_u16(*v);
        }
        Self::from_u16_array(new_array)
    }

    fn mirrored_u16(data: u16) -> u16 {
        (data << 12) & 0xf000 | (data << 4) & 0x0f00 | (data >> 4) & 0x00f0 | (data >> 12) & 0x000f
    }

    /// Symmetrizes the bit pattern by swapping pairs of pieces.
    pub fn symmetrized(&self, pairs: &Vec<(Piece, Piece)>) -> BitPattern {
        let mut new_images = self.array;
        for m in new_images.iter_mut() {
            *m = Self::symmetrized_u16(*m, pairs);
        }
        BitPattern::from_u16_array(new_images)
    }

    fn symmetrized_u16(data: u16, pairs: &Vec<(Piece, Piece)>) -> u16 {
        let mut result = data;
        for &(piece_a, piece_b) in pairs {
            let swap_pattern = (piece_a.id ^ piece_b.id) as u16 * 0x1111;
            let mask_a = Self::mask_of_piece_u16(data, piece_a);
            let mask_b = Self::mask_of_piece_u16(data, piece_b);
            result ^= (mask_a | mask_b) & swap_pattern;
        }
        result
    }

    /// Returns a bit pattern representing the area occupied by the given piece.
    pub fn mask_of(&self, piece: Piece) -> Self {
        let mut new_array = [0; SIZE];
        for (m, v) in new_array.iter_mut().zip(self.array.iter()) {
            *m = Self::mask_of_piece_u16(*v, piece);
        }
        Self::from_u16_array(new_array)
    }

    fn mask_of_piece_u16(data: u16, piece: Piece) -> u16 {
        let mut mask = data;
        mask ^= (piece.id as u16) * 0x1111;
        mask = ((mask >> 1) | mask) & 0x5555;
        mask = ((mask >> 2) | mask) & 0x1111;
        mask |= mask << 1;
        mask |= mask << 2;
        !mask
    }
}

// --- Operator Implementations ---

impl std::ops::BitAnd for BitPattern {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        let mut new_array = self.array;
        for (m, v) in new_array.iter_mut().zip(rhs.array.iter()) {
            *m &= *v;
        }
        Self::from_u16_array(new_array)
    }
}

impl std::ops::BitOr for BitPattern {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        let mut new_array = self.array;
        for (m, v) in new_array.iter_mut().zip(rhs.array.iter()) {
            *m |= *v;
        }
        Self::from_u16_array(new_array)
    }
}

impl std::ops::BitXor for BitPattern {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        let mut new_array = self.array;
        for (m, v) in new_array.iter_mut().zip(rhs.array.iter()) {
            *m ^= *v;
        }
        Self::from_u16_array(new_array)
    }
}

impl std::ops::Not for BitPattern {
    type Output = Self;

    fn not(self) -> Self::Output {
        let mut new_array = self.array;
        for m in new_array.iter_mut() {
            *m = !*m;
        }
        Self::from_u16_array(new_array)
    }
}

impl std::fmt::Display for BitPattern {
    /// Formats the `BitPattern` as a hexadecimal string with underscores between rows.
    /// Uses `try_fold` to iterate over the array and build the formatted string.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.array.iter().try_fold("[", |sep, arg| {
            write!(f, "{}{:04x}", sep, arg)?;
            Ok("_")
        })?;
        write!(f, "]")?;
        Ok(())
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
    fn from_u16_array_should_create_correct_pattern() {
        let bit_pattern = BitPattern::from_u16_array([0x2113, 0x2113, 0x4455, 0x6789, 0x6009]);
        let expected_array = [0x2113, 0x2113, 0x4455, 0x6789, 0x6009];
        assert_eq_hex!(&bit_pattern.array, &expected_array);
    }

    #[test]
    fn from_u128_should_create_correct_pattern() {
        let bit_pattern = BitPattern::new(0x2113_2113_4455_6789_6009);
        let expected_array = [0x2113, 0x2113, 0x4455, 0x6789, 0x6009];
        assert_eq_hex!(&bit_pattern.array, &expected_array);
    }

    #[test]
    fn get_u128_should_return_original_value() {
        let bit_pattern = BitPattern::new(0x2113_2113_4455_6789_6009);
        let expected_u128 = 0x2113_2113_4455_6789_6009u128;
        assert_eq_hex!(bit_pattern.get_u128(), expected_u128);
    }

    #[test]
    fn is_empty_and_is_not_empty_should_work() {
        let bit_pattern = BitPattern::new(0x00000000000000000000);
        let expected_array = [0, 0, 0, 0, 0];
        assert_eq_hex!(&bit_pattern.array, &expected_array);
        assert!(bit_pattern.is_empty());
        assert!(!bit_pattern.is_not_empty());

        let non_empty = BitPattern::new(0x0001_0000_0000_0000_0000);
        assert!(!non_empty.is_empty());
        assert!(non_empty.is_not_empty());
    }

    #[test]
    fn bitwise_operations_should_work() {
        let bit_pattern = BitPattern::new(0x2113_2113_4455_6789_6009);
        let bit_mask = BitPattern::new(0xffff_0000_ffff_0000_ffff);
        let and_result = bit_pattern & bit_mask;
        assert_eq_hex!(&and_result.array, &[0x2113, 0x0000, 0x4455, 0x0000, 0x6009]);

        let or_result = bit_pattern | bit_mask;
        assert_eq_hex!(&or_result.array, &[0xffff, 0x2113, 0xffff, 0x6789, 0xffff]);

        let xor_result = bit_pattern ^ bit_mask;
        assert_eq_hex!(
            &xor_result.array,
            &[!0x2113, 0x2113, !0x4455, 0x6789, !0x6009]
        );

        let not_result = !bit_pattern;
        assert_eq_hex!(
            &not_result.array,
            &[!0x2113, !0x2113, !0x4455, !0x6789, !0x6009]
        );
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
    fn mirrored_u16_should_reverse_nibbles() {
        assert_eq_hex!(BitPattern::mirrored_u16(0x1234), 0x4321);
        assert_eq_hex!(BitPattern::mirrored_u16(0x5678), 0x8765);
        assert_eq_hex!(BitPattern::mirrored_u16(0x9abc), 0xcba9);
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
    fn symmetrized_u16_should_swap_pairs() {
        let pairs: Vec<(Piece, Piece)> = vec![
            (Piece::new(2), Piece::new(3)),
            (Piece::new(4), Piece::new(5)),
        ];
        // swap 2 <-> 3, 4 <-> 5
        let swapped_data = BitPattern::symmetrized_u16(0x1234, &pairs);
        assert_eq_hex!(swapped_data, 0x1325);
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
    fn mask_of_piece_u16_should_return_piece_mask() {
        assert_eq_hex!(BitPattern::mask_of_piece_u16(0x1221, Piece::new(1)), 0xf00f);
        assert_eq_hex!(BitPattern::mask_of_piece_u16(0x1221, Piece::new(2)), 0x0ff0);
        assert_eq_hex!(BitPattern::mask_of_piece_u16(0x1234, Piece::new(4)), 0x000f);
        assert_eq_hex!(BitPattern::mask_of_piece_u16(0x5678, Piece::new(6)), 0x0f00);
        assert_eq_hex!(BitPattern::mask_of_piece_u16(0x5678, Piece::new(7)), 0x00f0);
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
