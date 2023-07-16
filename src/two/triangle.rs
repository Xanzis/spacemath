use super::dist::Dist;
use super::Point;
use crate::Orient;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Triangle(Point, Point, Point);

impl Triangle {
    pub fn dir(self) -> Orient {
        // return whether the triangle (p, q, r) turns counterclockwise
        // postive is natural (ccw), negative is cw
        let Triangle(p, q, r) = self;
        let val = (q.y - p.y) * (r.x - q.x) - (q.x - p.x) * (r.y - q.y);
        if val == 0.0 {
            Orient::Zero
        } else if val > 0.0 {
            Orient::Negative
        } else {
            Orient::Positive
        }
    }

    pub fn into_points(self) -> (Point, Point, Point) {
        (self.0, self.1, self.2)
    }

    pub fn bary_coor(self, p: Point) -> (f64, f64, f64) {
        // returns the barymetric coordinates of a point
        let Triangle(p1, p2, p3) = self;
        let (x1, y1) = p1.into();
        let (x2, y2) = p2.into();
        let (x3, y3) = p3.into();
        let (x, y) = p.into();

        let denom = (y2 - y3) * (x1 - x3) + (x3 - x2) * (y1 - y3);
        let a = ((y2 - y3) * (x - x3) + (x3 - x2) * (y - y3)) / denom;
        let b = ((y3 - y1) * (x - x3) + (x1 - x3) * (y - y3)) / denom;
        let c = 1.0 - a - b;
        (a, b, c)
    }

    pub fn in_triangle(self, p: Point) -> bool {
        // determine whether p is in the triangle
        let (a, b, c) = self.bary_coor(p);
        let rng = 0.0..=1.0;
        rng.contains(&a) && rng.contains(&b) && rng.contains(&c)
    }

    pub fn circumradius(self) -> f64 {
        let a = self.0.dist(self.1);
        let b = self.1.dist(self.2);
        let c = self.2.dist(self.0);

        (a * b * c) / ((a + b + c) * (b + c - a) * (c + a - b) * (a + b - c)).sqrt()
    }
}

impl From<(Point, Point, Point)> for Triangle {
    fn from(tri: (Point, Point, Point)) -> Self {
        Self(tri.0, tri.1, tri.2)
    }
}
