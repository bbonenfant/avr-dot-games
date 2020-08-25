/// Enumeration of directions.
#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {

    /// Returns the opposite direction
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use crate::Direction;
    /// 
    /// assert!(Direction::Up.opposite() == Direction::Down);
    /// ```
    pub fn opposite(&self) -> Direction {
        return
            match self {
                Direction::Left => { Direction::Right },
                Direction::Right => { Direction::Left },
                Direction::Up => { Direction::Down },
                Direction::Down => { Direction::Up },
            }
    }
}