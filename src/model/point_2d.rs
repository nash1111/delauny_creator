#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point2D {
    pub index: i64,
    pub x: f64,
    pub y: f64,
}

impl Point2D {
    pub fn distance_squared(&self, other: &Point2D) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        dx * dx + dy * dy
    }

    pub fn distance(&self, p: &Point2D) -> f64 {
        let dx = self.x - p.x;
        let dy = self.y - p.y;
        (dx * dx + dy * dy).sqrt()
    }
}
