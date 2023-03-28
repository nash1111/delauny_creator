use std::ops::Sub;

use crate::model::point_2d::*;
use crate::model::circle::*;
use crate::model::edge::*;

#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    pub a: Point2D,
    pub b: Point2D,
    pub c: Point2D,
}

impl Triangle {
    // TODO
    // generate circumcircle
    pub fn generate_circumcircle(&self) -> Circle {
        let d = 2.0 * ((self.a.x - self.c.x) * (self.b.y - self.c.y) - (self.b.x - self.c.x) * (self.a.y - self.c.y));

        let ux = ((self.a.x.powi(2) - self.c.x.powi(2) + self.a.y.powi(2) - self.c.y.powi(2)) * (self.b.y - self.c.y)
            - (self.b.x.powi(2) - self.c.x.powi(2) + self.b.y.powi(2) - self.c.y.powi(2)) * (self.a.y - self.c.y))
            / d;

        let uy = ((self.b.x - self.c.x) * (self.b.x.powi(2) - self.c.x.powi(2) + self.b.y.powi(2) - self.c.y.powi(2))
            - (self.a.x - self.c.x) * (self.a.x.powi(2) - self.c.x.powi(2) + self.a.y.powi(2) - self.c.y.powi(2)))
            / d;

        let center = Point2D { x: ux, y: uy };
        let radius = (center.x - self.a.x).hypot(center.y - self.a.y);

        Circle { center, radius }
    }

    pub fn circumcircle_contains(&self, point: &Point2D) -> bool {
        let d = 2.0 * ((self.a.x - self.c.x) * (self.b.y - self.c.y) - (self.b.x - self.c.x) * (self.a.y - self.c.y));

        let ux = ((self.a.x.powi(2) - self.c.x.powi(2) + self.a.y.powi(2) - self.c.y.powi(2)) * (self.b.y - self.c.y)
            - (self.b.x.powi(2) - self.c.x.powi(2) + self.b.y.powi(2) - self.c.y.powi(2)) * (self.a.y - self.c.y))
            / d;

        let uy = ((self.b.x - self.c.x) * (self.b.x.powi(2) - self.c.x.powi(2) + self.b.y.powi(2) - self.c.y.powi(2))
            - (self.a.x - self.c.x) * (self.a.x.powi(2) - self.c.x.powi(2) + self.a.y.powi(2) - self.c.y.powi(2)))
            / d;

        let center = Point2D { x: ux, y: uy };
        let radius = (center.x - self.a.x).hypot(center.y - self.a.y);

        let distance = (center.x - point.x).hypot(center.y - point.y);
        distance <= radius
    }
    
    pub fn edges(&self) -> [Edge; 3] {
        [
            Edge { start: self.a, end: self.b },
            Edge { start: self.b, end: self.c },
            Edge { start: self.c, end: self.a },
        ]
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

    pub fn vertices(&self) -> [Point2D; 3] {
        [self.a, self.b, self.c]
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