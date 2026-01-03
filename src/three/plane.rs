use std::fmt;

use super::Point;

// a line, in point-normal form
#[derive(Clone, Copy, Debug)]
pub struct Plane {
    normal: Point,
    origin: Point,
}

impl Plane {
    pub fn from_points(a: Point, b: Point, c: Point) -> Self {
        let normal = (b - a).cross(c - a).to_unit();
        let origin = a;
        Plane { normal, origin }
    }

    pub fn dist_signed(self, a: Point) -> f64 {
        (a - self.origin).dot(self.normal)
    }

    pub fn dist(self, a: Point) -> f64 {
        self.dist_signed(a).abs()
    }

    pub fn project(self, a: Point) -> Point {
        let dist = self.dist_signed(a);

        a - self.normal * dist
    }

    pub fn normal(self) -> Point {
        self.normal
    }

    pub fn origin(self) -> Point {
        self.origin
    }
}

impl fmt::Display for Plane {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Plane (normal {}, origin {})", self.normal, self.origin)
    }
}
