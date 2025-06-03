#[derive(Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash, Debug)]
pub struct Piece {
    pub id: u8,
}

impl Piece {
    /// Creates a new `Piece` with the given ID.
    pub const fn new(id: u8) -> Self {
        Self { id }
    }
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:x}", self.id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_piece() {
        let piece = Piece::new(5);
        assert_eq!(piece.id, 5);
        assert_eq!(format!("{}", piece), "5");

        assert_eq!(format!("{}", Piece::new(0xa)), "a");
    }
}
