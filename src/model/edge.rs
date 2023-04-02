use crate::model::point_2d::*;

#[derive(Debug, Clone, Copy)]
pub struct Edge {
    pub start: Point2D,
    pub end: Point2D,
}

impl Edge {
    pub fn reverse(&self) -> Edge {
        Edge {
            start: self.end,
            end: self.start,
        }
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        (self.start == other.start && self.end == other.end)
            || (self.start == other.end && self.end == other.start)
    }
}
