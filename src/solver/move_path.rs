use super::direction::Direction;

#[derive(Clone, Debug)]
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
