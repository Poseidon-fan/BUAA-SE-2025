use crate::point::Point;

pub struct Food(Point);

pub struct Barrier(Point);

impl From<(i32, i32)> for Food {
    fn from(pos: (i32, i32)) -> Self {
        Self(Point::from(pos))
    }
}

impl From<(i32, i32)> for Barrier {
    fn from(pos: (i32, i32)) -> Self {
        Self(Point::from(pos))
    }
}

impl Barrier {
    pub fn point(&self) -> Point {
        // Bug fix: Use `Self(Point)` to construct the tuple struct
        self.0
    }
}

impl Food {
    pub fn point(&self) -> Point {
        self.0
    }
}

pub struct Board {
    pub length: i32,
    pub foods: Vec<Food>,
    pub barriers: Vec<Barrier>,
}

impl Board {
    pub fn new(length: i32, foods_pos: &[i32], barriers_pos: &[i32]) -> Self {
        let mut foods = Vec::new();
        let mut barriers = Vec::new();
        for i in (0..foods_pos.len()).step_by(2) {
            let food = Food::from((foods_pos[i], foods_pos[i + 1]));
            foods.push(food);
        }
        for i in (0..barriers_pos.len()).step_by(2) {
            let barrier = Barrier::from((barriers_pos[i], barriers_pos[i + 1]));
            barriers.push(barrier);
        }

        Self { 
            length,
            foods,
            barriers,
        }
    }
}