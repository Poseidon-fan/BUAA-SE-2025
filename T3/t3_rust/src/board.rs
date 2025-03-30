use std::collections::HashSet;

use crate::{point::Point, snake::Snake};

#[derive(Debug)]
pub struct Board {
    pub length: i32,
    enemies: Vec<Snake>,
    foods: Vec<Food>
}

impl Board {
    pub fn new(length: i32, enemies: Vec<Snake>, foods: Vec<Food>) -> Self {
        Self {
            length,
            enemies,
            foods
        }
    }

    pub fn possible_barriers(&self) -> HashSet<Point> {
        self.enemies.iter().flat_map(|s| s.posible_barriers()).collect()
    }
}

#[derive(Debug)]
pub struct Food(Point);

impl From<(i32, i32)> for Food {
    fn from(pos: (i32, i32)) -> Self {
        Self(Point::from(pos))
    }
}
