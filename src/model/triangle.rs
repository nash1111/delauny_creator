use crate::model::circle::*;
use crate::model::edge::*;
use crate::model::point_2d::*;

#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    pub a: Point2D,
    pub b: Point2D,
    pub c: Point2D,
}

impl Triangle {
    pub fn circumcenter(&self) -> Point2D {
        let d = 2.0
            * ((self.a.x - self.c.x) * (self.b.y - self.c.y)
                - (self.b.x - self.c.x) * (self.a.y - self.c.y));

        let ux = (((self.a.x * self.a.x) + (self.a.y * self.a.y)
            - (self.c.x * self.c.x)
            - (self.c.y * self.c.y))
            * (self.b.y - self.c.y)
            - ((self.b.x * self.b.x) + (self.b.y * self.b.y)
                - (self.c.x * self.c.x)
                - (self.c.y * self.c.y))
                * (self.a.y - self.c.y))
            / d;

        let uy = (((self.b.x * self.b.x) + (self.b.y * self.b.y)
            - (self.c.x * self.c.x)
            - (self.c.y * self.c.y))
            * (self.a.x - self.c.x)
            - ((self.a.x * self.a.x) + (self.a.y * self.a.y)
                - (self.c.x * self.c.x)
                - (self.c.y * self.c.y))
                * (self.b.x - self.c.x))
            / d;

        Point2D { index: i64::MAX, x: ux, y: uy }
    }

    pub fn generate_circumcircle(&self) -> Circle {
        let center = self.circumcenter();
        let radius = center.distance(&self.a);
        Circle { center, radius }
    }

    pub fn edges(&self) -> [Edge; 3] {
        [
            Edge {
                start: self.a,
                end: self.b,
            },
            Edge {
                start: self.b,
                end: self.c,
            },
            Edge {
                start: self.c,
                end: self.a,
            },
        ]
    }

    pub fn contains_edge(&self, edge: (Point2D, Point2D)) -> bool {
        let points = [self.a, self.b, self.c];
        points.contains(&edge.0) && points.contains(&edge.1)
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
