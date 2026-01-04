use std::fmt;

use super::Point;

#[derive(Clone, Copy, Debug)]
pub struct Line {
    origin: Point,
    tangent: Point,
}

impl Line {
    pub fn from_points(a: Point, b: Point) -> Self {
        let origin = a;
        let tangent = (b - a).to_unit();

        Line { origin, tangent }
    }

    pub fn project(self, a: Point) -> Point {
        // project point onto line
        self.origin + self.tangent * (a - self.origin).dot(self.tangent)
    }

    pub fn dist(self, a: Point) -> f64 {
        a.dist(self.project(a))
    }

    pub fn away(self, a: Point) -> Point {
        // return a unit vector perpendicular to self.tangent, pointing away from a wrt the line
        let projected = self.project(a);
        (projected - a).to_unit()
    }
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Line3 (origin {}, tangent {})",
            self.origin, self.tangent
        )
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Segment(pub Point, pub Point);

impl Segment {
    pub fn to_line(self) -> Line {
        Line::from_points(self.0, self.1)
    }

    pub fn clamp(self, p: Point) -> Point {
        // project p onto the segment's line and clamp it to the segment bounds
        let ab = self.1 - self.0;

        let ap = p - self.0;

        let t = ab.dot(ap) / ab.dot(ab);
        let t = t.clamp(0.0, 1.0);

        self.0 + (ab * t)
    }
}

impl From<(Point, Point)> for Segment {
    fn from(x: (Point, Point)) -> Self {
        Self(x.0, x.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clamp() {
        let a: Point = (0.0, 1.0, 0.0).into();
        let b: Point = (1.0, 0.0, 0.0).into();

        let x = Point::origin();
        let y: Point = (-0.5, 1.0, 0.0).into();
        let z: Point = (1.0, -0.5, 0.0).into();

        let ab: Segment = (a, b).into();

        let eps = 1e-6;

        let mid = a.mid(b);

        assert!(ab.clamp(x).dist(mid) < eps);
        assert!(ab.clamp(y).dist(a) < eps);
        assert!(ab.clamp(z).dist(b) < eps);
    }
}
