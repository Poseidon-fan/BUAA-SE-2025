use std::collections::HashSet;

use crate::point::{Direction, Point};

#[derive(Debug)]
pub struct Snake(Vec<Point>);

impl From<&[i32]> for Snake {
    fn from(value: &[i32]) -> Self {
        let mut points = Vec::new();
        for i in (0..value.len()).step_by(2) {
            let point = Point::from((value[i], value[i + 1]));
            points.push(point);
        }
        Self(points)
    }
}

impl Snake {
    pub fn head(&self) -> &Point {
        &self.0[0]
    }

    pub fn body_segment(&self, i: i32) -> Option<&Point> {
        self.0.get(i as usize)
    }

    pub fn posible_barriers(&self) -> Vec<Point> {
        let mut barriers_set: HashSet<Point> = self.0.iter().cloned().collect();
        const ALL_DIRECTIONS: [Direction; 4] = [Direction::Up, Direction::Left, Direction::Down, Direction::Right];
        for d in ALL_DIRECTIONS {
            barriers_set.insert(
                self.head().move_to(d)
            );
        }
        barriers_set.into_iter().collect()
    }

    pub fn absolute_barriers(&self) -> Vec<Point> {
        self.0.clone()
    }
}