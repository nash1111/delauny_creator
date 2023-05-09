use crate::model::point_2d::*;

#[derive(Debug, Clone, Copy)]
pub struct Circle {
    pub center: Point2D,
    pub radius: f64,
}

impl Circle {
    pub fn point_in_circle(&self, point: &Point2D) -> bool {
        let squared_distance = {
            let ref this = self.center;
            let dx = this.x - point.x;
            let dy = this.y - point.y;
            dx * dx + dy * dy
        };
        squared_distance <= (self.radius) * (self.radius)
    }
}
