use super::shift::Shift;
use super::{Arc, Circle, Line, Point, Ray, Segment};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Count {
    Zero,
    One,
    Many(u8),
    Inf,
}

impl From<usize> for Count {
    fn from(val: usize) -> Self {
        match val {
            0 => Count::Zero,
            1 => Count::One,
            x => Count::Many(x as u8),
        }
    }
}

pub trait Intersect<T> {
    fn intersects_at(&self, _other: &T) -> Vec<Point> {
        unimplemented!()
    }

    // count of intersections
    fn intersects(&self, other: &T) -> Count {
        self.intersects_at(other).len().into()
    }
}

impl Intersect<Line> for Line {
    fn intersects(&self, other: &Line) -> Count {
        let disc = (self.a * other.b) - (self.b * other.a);

        // future: some kind of tolerance?
        if disc == 0.0 {
            let scale = self.a / other.a;
            if (self.b / other.b == scale) && (self.c / other.c == scale) {
                Count::Inf
            } else {
                Count::Zero
            }
        } else {
            Count::One
        }
    }

    fn intersects_at(&self, other: &Line) -> Vec<Point> {
        match self.intersects(other) {
            Count::One => {
                let x = ((self.b * other.c) - (self.c * other.b))
                    / ((self.b * other.a) - (self.a * other.b));
                let y = ((self.c * other.a) - (self.a * other.c))
                    / ((self.b * other.a) - (self.a * other.b));

                vec![Point::new(x, y)]
            }
            _ => Vec::new(),
        }
    }
}

impl Intersect<Line> for Segment {
    fn intersects_at(&self, other: &Line) -> Vec<Point> {
        self.to_line()
            .intersects_at(other)
            .into_iter()
            .filter(|p| self.bounds_contain(*p))
            .collect()
    }
}

impl Intersect<Segment> for Line {
    fn intersects_at(&self, other: &Segment) -> Vec<Point> {
        other.intersects_at(&self)
    }
}

impl Intersect<Line> for Circle {
    fn intersects_at(&self, other: &Line) -> Vec<Point> {
        // simplify the problem to a line through a circle at the origin
        let r = self.radius;
        let shifted_line = other.shift_subtract(self.center);

        // awful awful approach
        // find the perpendicular line through the origin
        // and the distance to the intersection with that line
        let perp = shifted_line.perp_origin();
        let inter = perp.intersects_at(&shifted_line)[0];
        let dist = inter.norm();

        if dist == r {
            // shift back to original coordinate frame
            return vec![inter + self.center];
        } else if dist > r {
            return vec![];
        }

        // half of the chord of the circle
        let half_chord = (r.powi(2) - dist.powi(2)).sqrt();

        let unit_radial = inter.to_unit();
        let unit_tangential = unit_radial.perp();

        let p1 = (unit_radial * dist) + (unit_tangential * half_chord);
        let p2 = (unit_radial * dist) - (unit_tangential * half_chord);

        // shift back to original coordinate frame
        vec![p1 + self.center, p2 + self.center]
    }
}

impl Intersect<Circle> for Line {
    fn intersects_at(&self, other: &Circle) -> Vec<Point> {
        other.intersects_at(self)
    }
}

impl Intersect<Circle> for Ray {
    fn intersects_at(&self, other: &Circle) -> Vec<Point> {
        let line = self.to_line();
        line.intersects_at(other)
            .into_iter()
            .filter(|p| self.bounds_contain(*p))
            .collect()
    }
}

impl Intersect<Ray> for Circle {
    fn intersects_at(&self, other: &Ray) -> Vec<Point> {
        other.intersects_at(self)
    }
}

impl Intersect<Ray> for Arc {
    fn intersects_at(&self, other: &Ray) -> Vec<Point> {
        let circle = self.to_circle();
        circle
            .intersects_at(other)
            .into_iter()
            .filter(|p| self.bounds_contain(*p))
            .collect()
    }
}

impl Intersect<Arc> for Ray {
    fn intersects_at(&self, other: &Arc) -> Vec<Point> {
        other.intersects_at(&self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_line_one() {
        let a = Line::new(2.0, 3.0, 2.0);
        let b = Line::new(1.0, -1.0, 6.0);

        assert_eq!(a.intersects(&b), Count::One);
        assert_eq!(a.intersects(&b), a.intersects_at(&b).len().into());

        let x = a.intersects_at(&b)[0];
        assert!(x.dist((4.0, -2.0).into()) < 1e-6);
    }

    #[test]
    fn line_line_zero() {
        let a = Line::new(2.0, 3.0, 2.0);
        let b = Line::new(2.0, 3.0, 4.0);

        assert_eq!(a.intersects(&b), Count::Zero);
        assert_eq!(a.intersects(&b), a.intersects_at(&b).len().into());
    }

    #[test]
    fn line_segment_one() {
        let a = Line::new(2.0, 3.0, 2.0);
        let b = Segment::new((3.0, -3.0).into(), (5.0, -1.0).into());

        assert_eq!(a.intersects(&b), Count::One);
        assert_eq!(a.intersects(&b), a.intersects_at(&b).len().into());

        let x = a.intersects_at(&b)[0];
        assert!(x.dist((4.0, -2.0).into()) < 1e-6);
    }

    #[test]
    fn line_segment_zero() {
        let a = Line::new(2.0, 3.0, 8.0);
        let b = Segment::new((3.0, -3.0).into(), (5.0, -1.0).into());

        assert_eq!(a.intersects(&b), Count::Zero);
        assert_eq!(a.intersects(&b), a.intersects_at(&b).len().into());
    }

    #[test]
    fn line_circle_two() {
        let a = Line::new(2.0, 3.0, 2.0);
        let b = Circle::new((1.0, 2.0).into(), 2.0);

        assert_eq!(a.intersects(&b), Count::Many(2));
        assert_eq!(a.intersects(&b), a.intersects_at(&b).len().into());

        let x = a.intersects_at(&b)[0];
        let y = a.intersects_at(&b)[1];
        let p = (-0.846, 1.231).into();
        let q = (1.0, 0.0).into();

        let case_a = (x.dist(p) < 1e-3) && (y.dist(q) < 1e-3);
        let case_b = (x.dist(q) < 1e-3) && (y.dist(p) < 1e-3);

        assert!(case_a || case_b);
    }

    #[test]
    fn line_circle_zero() {
        let a = Line::new(2.0, 3.0, -3.0);
        let b = Circle::new((1.0, 2.0).into(), 2.0);

        assert_eq!(a.intersects(&b), Count::Zero);
        assert_eq!(a.intersects(&b), a.intersects_at(&b).len().into());
    }

    #[test]
    fn ray_circle_two() {
        let a = Ray::new((-2.0, 1.0).into(), (2.0f64 / 3.0).atan());
        let b = Circle::new((1.0, 2.0).into(), 2.0);

        assert_eq!(a.intersects(&b), Count::Many(2));
        assert_eq!(a.intersects(&b), a.intersects_at(&b).len().into());

        let x = a.intersects_at(&b)[0];
        let y = a.intersects_at(&b)[1];
        let p = (-0.975, 1.683).into();
        let q = (2.052, 3.701).into();

        let case_a = (x.dist(p) < 1e-3) && (y.dist(q) < 1e-3);
        let case_b = (x.dist(q) < 1e-3) && (y.dist(p) < 1e-3);

        assert!(case_a || case_b);
    }

    #[test]
    fn ray_circle_one() {
        let a = Ray::new((0.0, 7.0 / 3.0).into(), (2.0f64 / 3.0).atan());
        let b = Circle::new((1.0, 2.0).into(), 2.0);

        assert_eq!(a.intersects(&b), Count::One);
        assert_eq!(a.intersects(&b), a.intersects_at(&b).len().into());

        let x = a.intersects_at(&b)[0];
        let p = (2.052, 3.701).into();

        assert!(x.dist(p) < 1e-3);
    }

    #[test]
    fn ray_circle_zero() {
        let a = Ray::new((-2.0, 4.0).into(), (2.0f64 / 3.0).atan());
        let b = Circle::new((1.0, 2.0).into(), 2.0);

        assert_eq!(a.intersects(&b), Count::Zero);
        assert_eq!(a.intersects(&b), a.intersects_at(&b).len().into());
    }

    #[test]
    fn ray_arc_two() {
        let a = Ray::new((-2.0, 1.0).into(), (2.0f64 / 3.0).atan());
        let b = Arc::from_center_ang((1.0, 2.0).into(), 2.0, 0.0, 4.5);

        assert_eq!(a.intersects(&b), Count::Many(2));
        assert_eq!(a.intersects(&b), a.intersects_at(&b).len().into());

        let x = a.intersects_at(&b)[0];
        let y = a.intersects_at(&b)[1];
        let p = (-0.975, 1.683).into();
        let q = (2.052, 3.701).into();

        let case_a = (x.dist(p) < 1e-3) && (y.dist(q) < 1e-3);
        let case_b = (x.dist(q) < 1e-3) && (y.dist(p) < 1e-3);

        assert!(case_a || case_b);
    }

    #[test]
    fn ray_arc_one() {
        let a = Ray::new((-2.0, 1.0).into(), (2.0f64 / 3.0).atan());
        let b = Arc::from_center_ang((1.0, 2.0).into(), 2.0, 1.5, 4.5);

        assert_eq!(a.intersects(&b), Count::One);
        assert_eq!(a.intersects(&b), a.intersects_at(&b).len().into());

        let x = a.intersects_at(&b)[0];
        let p = (-0.975, 1.683).into();

        assert!(x.dist(p) < 1e-3);
    }
}
