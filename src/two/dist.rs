use super::intersect::Intersect;
use super::{Arc, Circle, Line, Point, Ray, Segment};

pub trait Dist {
    fn dist(&self, r: Point) -> f64;
}

impl Dist for Line {
    fn dist(&self, r: Point) -> f64 {
        let perp = self.perp_through(r);
        perp.intersects_at(self).get_one().unwrap().dist(r)
    }
}

impl Dist for Segment {
    fn dist(&self, r: Point) -> f64 {
        // if out of bounds, choose the closest node
        let projected = self.to_line().projected(r);

        if self.bounds_contain(projected) {
            projected.dist(r)
        } else {
            r.dist(self.p()).min(r.dist(self.q()))
        }
    }
}

impl Dist for Ray {
    fn dist(&self, r: Point) -> f64 {
        if self.bounds_contain(r) {
            self.to_line().dist(r)
        } else {
            r.dist(self.init)
        }
    }
}

impl Dist for Circle {
    fn dist(&self, r: Point) -> f64 {
        let center_dist = self.center.dist(r);
        (center_dist - self.radius).abs()
    }
}

impl Dist for Arc {
    fn dist(&self, r: Point) -> f64 {
        if self.bounds_contain(r) {
            self.to_circle().dist(r)
        } else {
            r.dist(self.p()).min(r.dist(self.q()))
        }
    }
}
