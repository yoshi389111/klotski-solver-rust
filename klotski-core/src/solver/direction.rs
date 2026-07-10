/// The four possible directions in which a piece can be moved.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    /// Returns the opposite direction.
    pub fn reversed(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_direction_reversed() {
        // Act & Assert
        assert_eq!(Direction::Up.reversed(), Direction::Down);
        assert_eq!(Direction::Down.reversed(), Direction::Up);
        assert_eq!(Direction::Left.reversed(), Direction::Right);
        assert_eq!(Direction::Right.reversed(), Direction::Left);
    }
}
