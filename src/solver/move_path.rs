use super::Direction;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum MovePath {
    None,
    One(Direction),
    Two(Direction, Direction),
}

impl std::fmt::Display for MovePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MovePath::None => write!(f, "None"),
            MovePath::One(d) => write!(f, "{d}"),
            MovePath::Two(d1, d2) => write!(f, "{d1} and {d2}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_path_display() {
        // Arrange: Test MovePath Display implementation
        let up = Direction::Up;
        let down = Direction::Down;
        let path_none = MovePath::None;
        let path_one = MovePath::One(up);
        let path_two = MovePath::Two(up, down);
        // Act & Assert
        assert_eq!(format!("{path_none}"), "None");
        assert_eq!(format!("{path_one}"), format!("{up}"));
        assert_eq!(format!("{path_two}"), format!("{up} and {down}"));
    }
}
