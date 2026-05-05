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

pub enum Shape {
    Circle(f64),         // radius
    Rectangle(f64, f64), // width, height
    Square(f64),         // side length
}

impl Shape {
    pub fn area(&self) -> f64 {
        match self {
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
            Shape::Rectangle(width, height) => width * height,
            Shape::Square(side) => side * side,
        }
    }
}
