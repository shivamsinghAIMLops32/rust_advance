pub enum Direction {
    North,
    South,
    East,
    West,
}

// Implementing Debug trait for Direction enum to enable printing
impl std::fmt::Debug for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Direction::North => write!(f, "North"),
            Direction::South => write!(f, "South"),
            Direction::East => write!(f, "East"),
            Direction::West => write!(f, "West"),
        }
    }
}
