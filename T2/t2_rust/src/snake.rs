use crate::point::Point;

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
}