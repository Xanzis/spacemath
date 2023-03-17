use super::{Arc, Circle, Line, Point, Ray, Segment};

pub trait Shift {
    fn shift(&self, r: Point) -> Self;

    fn shift_subtract(&self, r: Point) -> Self
    where
        Self: Sized,
    {
        self.shift(r * -1.0)
    }
}

impl Shift for Point {
    fn shift(&self, r: Point) -> Point {
        self.clone() + r
    }

    fn shift_subtract(&self, r: Point) -> Point {
        self.clone() - r
    }
}

impl Shift for Line {
    fn shift(&self, r: Point) -> Line {
        let mut l = self.clone();
        l.offset_x(r.x);
        l.offset_y(r.y);
        l
    }
}

impl Shift for Segment {
    fn shift(&self, r: Point) -> Segment {
        let s = self.clone();
        let (p, q) = s.into_points();
        Segment::new(p.shift(r), q.shift(r))
    }
}

impl Shift for Ray {
    fn shift(&self, r: Point) -> Ray {
        let mut x = self.clone();
        x.offset_x(r.x);
        x.offset_y(r.y);
        x
    }
}

impl Shift for Circle {
    fn shift(&self, r: Point) -> Circle {
        let mut c = self.clone();
        c.offset_x(r.x);
        c.offset_y(r.y);
        c
    }
}

impl Shift for Arc {
    fn shift(&self, r: Point) -> Arc {
        let mut a = self.clone();
        a.offset_x(r.x);
        a.offset_y(r.y);
        a
    }
}
