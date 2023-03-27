use std::ops::Sub;

use crate::model::point_2d::*;

#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    pub a: Point2D,
    pub b: Point2D,
    pub c: Point2D,
}

impl Triangle {
    pub fn circumcircle_contains(&self, point: &Point2D) -> bool {
        let ab = self.a - self.b;
        let ac = self.a - self.c;
        let det = 2.0 * (ab.x * ac.y - ab.y * ac.x);

        let a_squared = self.a.x * self.a.x + self.a.y * self.a.y;
        let b_squared = self.b.x * self.b.x + self.b.y * self.b.y;
        let c_squared = self.c.x * self.c.x + self.c.y * self.c.y;

        let u_x = (a_squared * (self.c.y - self.b.y)
            + b_squared * (self.a.y - self.c.y)
            + c_squared * (self.b.y - self.a.y))
            / det;
        let u_y = (a_squared * (self.b.x - self.c.x)
            + b_squared * (self.c.x - self.a.x)
            + c_squared * (self.a.x - self.b.x))
            / det;

        let center = Point2D { x: u_x, y: u_y };
        let radius_squared = (self.a.x - center.x).powi(2) + (self.a.y - center.y).powi(2);
        let distance_squared = (point.x - center.x).powi(2) + (point.y - center.y).powi(2);

        distance_squared <= radius_squared
    }

    pub fn contains_edge(&self, edge: (Point2D, Point2D)) -> bool {
        let points = [self.a, self.b, self.c];
        points.contains(&edge.0) && points.contains(&edge.1)
    }

    pub fn contains_vertex(&self, point: Point2D) -> bool {
        self.a == point || self.b == point || self.c == point
    }

    pub fn contains_point(&self, point: &Point2D) -> bool {
        let det = (self.a.x * (self.b.y - self.c.y) + self.b.x * (self.c.y - self.a.y) + self.c.x * (self.a.y - self.b.y)).abs();
        let alpha = ((self.b.x * self.c.y - self.c.x * self.b.y) + (self.c.y - self.b.y) * point.x + (self.b.x - self.c.x) * point.y) / det;
        let beta = ((self.c.x * self.a.y - self.a.x * self.c.y) + (self.a.y - self.c.y) * point.x + (self.c.x - self.a.x) * point.y) / det;
        let gamma = 1.0 - alpha - beta;

        alpha >= 0.0 && alpha <= 1.0 && beta >= 0.0 && beta <= 1.0 && gamma >= 0.0 && gamma <= 1.0
    }
}

impl PartialEq for Triangle {
    fn eq(&self, other: &Self) -> bool {
        self.contains_edge((other.a, other.b))
            && self.contains_edge((other.b, other.c))
            && self.contains_edge((other.c, other.a))
    }
}

impl std::ops::Index<usize> for Triangle {
    type Output = Point2D;

    fn index(&self, i: usize) -> &Self::Output {
        match i {
            0 => &self.a,
            1 => &self.b,
            2 => &self.c,
            _ => panic!("Index out of bounds: {}", i),
        }
    }
}