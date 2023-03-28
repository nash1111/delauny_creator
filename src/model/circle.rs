use crate::model::point_2d::*;

#[derive(Debug, Clone, Copy)]
pub struct Circle {
    pub center: Point2D,
    pub radius: f64,
}

impl Circle {
    pub fn point_in_circle(&self, point: &Point2D) -> bool {
        let squared_distance = self.center.distance_squared(point);
        squared_distance <= (self.radius) * (self.radius)
    }
}