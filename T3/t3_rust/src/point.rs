#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl From<(i32, i32)> for Point {
    fn from((x, y): (i32, i32)) -> Self {
        Self { x, y }
    }
}

impl Point {
    pub fn move_to(&self, direction: Direction) -> Self {
        match direction {
            Direction::Up => Self { x: self.x, y: self.y + 1 },
            Direction::Left => Self { x: self.x - 1, y: self.y },
            Direction::Down => Self { x: self.x, y: self.y - 1 },
            Direction::Right => Self { x: self.x + 1, y: self.y },
        }
    }
}

#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up = 0,
    Left = 1,
    Down = 2,
    Right = 3,
}

impl From<i32> for Direction {
    fn from(value: i32) -> Self {
        match value {
            0 => Self::Up,
            1 => Self::Left,
            2 => Self::Down,
            3 => Self::Right,
            _ => panic!("Invalid direction"),
        }
    }
}

impl From<(&Point, &Point)> for Direction {
    fn from((from, to): (&Point, &Point)) -> Self {
        match (to.x - from.x, to.y - from.y) {
            (0, 1) => Self::Up,
            (-1, 0) => Self::Left,
            (0, -1) => Self::Down,
            (1, 0) => Self::Right,
            _ => panic!("Invalid direction"),
        }
    }
}
