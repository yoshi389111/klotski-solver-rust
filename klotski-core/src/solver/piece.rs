/// A piece in the puzzle.
#[derive(Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash, Debug)]
pub struct Piece {
    /// The unique identifier for the piece (0x0 - 0xf).
    pub id: u8,
}

impl Piece {
    /// Creates a new `Piece` with the given ID.
    pub const fn new(id: u8) -> Self {
        Self { id }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_piece() {
        // Arrange & Act
        let piece = Piece::new(5);

        // Assert
        assert_eq!(piece.id, 5);
    }
}
