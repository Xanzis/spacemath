use super::shift::Shift;
use super::{Arc, Circle, Line, Point, Ray, Segment};

use super::dist::Dist;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Count {
    Zero,
    One,
    Many(u8),
    Inf,
}

impl Count {
    pub fn is_even(self) -> bool {
        match self {
            Count::Zero => true,
            Count::One => false,
            Count::Many(x) => x % 2 == 0,
            Count::Inf => false,
        }
    }

    pub fn is_odd(self) -> bool {
        match self {
            Count::Zero => false,
            Count::One => true,
            Count::Many(x) => x % 2 == 1,
            Count::Inf => false,
        }
    }

    pub fn is_zero(self) -> bool {
        match self {
            Count::Zero => true,
            _ => false,
        }
    }

    pub fn is_nonzero(self) -> bool {
        match self {
            Count::Zero => false,
            _ => true,
        }
    }
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

// used instead of a vec to avoid allocations in simple cases
#[derive(Clone, Debug, PartialEq)]
pub enum Intersections {
    Zero,
    One(Point),
    Two(Point, Point),
    Many(Vec<Point>),
}

impl Intersections {
    pub fn is_zero(&self) -> bool {
        match self {
            &Intersections::Zero => true,
            _ => false,
        }
    }

    pub fn is_nonzero(&self) -> bool {
        match self {
            &Intersections::Zero => false,
            _ => true,
        }
    }

    pub fn count(&self) -> Count {
        match self {
            &Self::Zero => 0,
            &Self::One(_) => 1,
            &Self::Two(_, _) => 2,
            &Self::Many(ref v) => v.len(),
        }
        .into()
    }

    pub fn from_vec(points: Vec<Point>) -> Self {
        match points.len() {
            0 => Self::Zero,
            1 => Self::One(points[0]),
            2 => Self::Two(points[0], points[1]),
            _ => Self::Many(points),
        }
    }

    pub fn from_zero() -> Self {
        Self::Zero
    }

    pub fn from_one(point: Point) -> Self {
        Self::One(point)
    }

    pub fn from_two(point_a: Point, point_b: Point) -> Self {
        Self::Two(point_a, point_b)
    }

    pub fn into_vec(self) -> Vec<Point> {
        match self {
            Self::Zero => Vec::new(),
            Self::One(a) => vec![a],
            Self::Two(a, b) => vec![a, b],
            Self::Many(v) => v,
        }
    }

    pub fn get_one(&self) -> Option<Point> {
        match self {
            &Self::Zero => None,
            &Self::One(a) => Some(a),
            &Self::Two(a, _) => Some(a),
            &Self::Many(ref v) => Some(v[0]),
        }
    }

    pub fn filter<T: FnMut(&Point) -> bool>(self, mut predicate: T) -> Self {
        match self {
            Self::Zero => Self::Zero,
            Self::One(a) => {
                if predicate(&a) {
                    Self::One(a)
                } else {
                    Self::Zero
                }
            }
            Self::Two(a, b) => {
                if predicate(&a) {
                    if predicate(&b) {
                        Self::Two(a, b)
                    } else {
                        Self::One(a)
                    }
                } else {
                    if predicate(&b) {
                        Self::One(b)
                    } else {
                        Self::Zero
                    }
                }
            }
            Self::Many(v) => {
                let res = v.into_iter().filter(predicate).collect();
                Self::from_vec(res)
            }
        }
    }

    pub fn combine(self, other: Intersections) -> Self {
        if self == Self::Zero {
            return other;
        }

        if other == Self::Zero {
            return self;
        }

        match self {
            Self::One(a) => match other {
                Self::One(x) => Self::Two(a, x),
                Self::Two(x, y) => Self::Many(vec![a, x, y]),
                Self::Many(mut v) => {
                    v.push(a);
                    Self::Many(v)
                }
                _ => unreachable!(),
            },
            Self::Two(a, b) => match other {
                Self::One(x) => Self::Many(vec![a, b, x]),
                Self::Two(x, y) => Self::Many(vec![a, b, x, y]),
                Self::Many(mut v) => {
                    v.push(a);
                    v.push(b);
                    Self::Many(v)
                }
                _ => unreachable!(),
            },
            Self::Many(mut v) => match other {
                Self::One(x) => {
                    v.push(x);
                    Self::Many(v)
                }
                Self::Two(x, y) => {
                    v.push(x);
                    v.push(y);
                    Self::Many(v)
                }
                Self::Many(w) => {
                    v.extend(w);
                    Self::Many(v)
                }
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
}

pub trait Intersect<T> {
    fn intersects_at(&self, _other: &T) -> Intersections {
        unimplemented!()
    }

    // count of intersections
    fn intersects(&self, other: &T) -> Count {
        self.intersects_at(other).count()
    }
}

macro_rules! reflexive_intersect {
    ($a:ident, $b:ident) => {
        impl Intersect<$b> for $a {
            fn intersects_at(&self, other: &$b) -> Intersections {
                other.intersects_at(self)
            }
        }
    };
}

pub(crate) use reflexive_intersect;

// Line intersection definitions

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

    fn intersects_at(&self, other: &Line) -> Intersections {
        match self.intersects(other) {
            Count::One => {
                let x = ((self.b * other.c) - (self.c * other.b))
                    / ((self.b * other.a) - (self.a * other.b));
                let y = ((self.c * other.a) - (self.a * other.c))
                    / ((self.b * other.a) - (self.a * other.b));

                Intersections::One(Point::new(x, y))
            }
            _ => Intersections::Zero,
        }
    }
}

// Segment intersection definitions

impl<T: Intersect<Line>> Intersect<T> for Segment {
    fn intersects_at(&self, other: &T) -> Intersections {
        other
            .intersects_at(&self.to_line())
            .filter(|p| self.bounds_contain(*p))
    }
}

// each bounded edge type (segment, ray, arc) needs relexive definitions to the base edge types
reflexive_intersect!(Line, Segment);
reflexive_intersect!(Circle, Segment);

// Ray intersection definitions

impl<T: Intersect<Line>> Intersect<T> for Ray {
    fn intersects_at(&self, other: &T) -> Intersections {
        other
            .intersects_at(&self.to_line())
            .filter(|p| self.bounds_contain(*p))
    }
}

reflexive_intersect!(Line, Ray);
reflexive_intersect!(Circle, Ray);

// Circle intersection definitions

impl Intersect<Circle> for Circle {
    fn intersects_at(&self, other: &Circle) -> Intersections {
        let c1 = self.center;
        let r1 = self.radius;
        let c2 = other.center;
        let r2 = other.radius;

        let dist = c1.dist(c2);

        if dist > (r1 + r2) {
            return Intersections::Zero;
        }

        if dist == (r1 + r2) {
            return Intersections::from_one(((c1 * r2) + (c2 * r1)) / dist);
        }

        let mid = c1.mid(c2);

        let a = (r1.powi(2) - r2.powi(2)) / (2.0 * dist.powi(2));

        let inner = (2.0 * (r1.powi(2) + r2.powi(2)) / dist.powi(2)) - (a * 2.0).powi(2);
        let b = (inner - 1.0).sqrt() / 2.0;

        let b_dir = Point::new(c2.y - c1.y, c1.x - c2.x);

        let p1 = (mid + ((c2 - c1) * a)) + (b_dir * b);
        let p2 = (mid + ((c2 - c1) * a)) - (b_dir * b);

        Intersections::Two(p1, p2)
    }
}

impl Intersect<Line> for Circle {
    fn intersects_at(&self, other: &Line) -> Intersections {
        // simplify the problem to a line through a circle at the origin
        let r = self.radius;
        let shifted_line = other.shift_subtract(self.center);

        // awful awful approach
        // find the perpendicular line through the origin
        // and the distance to the intersection with that line
        let perp = shifted_line.perp_origin();
        let inter = perp.intersects_at(&shifted_line).get_one().unwrap();
        let dist = inter.norm();

        if dist == r {
            // shift back to original coordinate frame
            return Intersections::from_one(inter + self.center);
        } else if dist > r {
            return Intersections::Zero;
        }

        // half of the chord of the circle
        let half_chord = (r.powi(2) - dist.powi(2)).sqrt();

        let unit_radial = inter.to_unit();
        let unit_tangential = unit_radial.perp();

        let p1 = (unit_radial * dist) + (unit_tangential * half_chord);
        let p2 = (unit_radial * dist) - (unit_tangential * half_chord);

        // shift back to original coordinate frame
        Intersections::Two(p1 + self.center, p2 + self.center)
    }
}

reflexive_intersect!(Line, Circle);

// Arc intersection definitions

impl<T: Intersect<Circle>> Intersect<T> for Arc {
    fn intersects_at(&self, other: &T) -> Intersections {
        other
            .intersects_at(&self.to_circle())
            .filter(|p| self.bounds_contain(*p))
    }
}

reflexive_intersect!(Line, Arc);
reflexive_intersect!(Circle, Arc);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_line_one() {
        let a = Line::new(2.0, 3.0, 2.0);
        let b = Line::new(1.0, -1.0, 6.0);

        assert_eq!(a.intersects(&b), Count::One);
        assert_eq!(a.intersects(&b), a.intersects_at(&b).count());

        let x = a.intersects_at(&b).get_one().unwrap();
        assert!(x.dist((4.0, -2.0).into()) < 1e-6);
    }

    #[test]
    fn line_line_zero() {
        let a = Line::new(2.0, 3.0, 2.0);
        let b = Line::new(2.0, 3.0, 4.0);

        assert_eq!(a.intersects(&b), Count::Zero);
        assert_eq!(a.intersects(&b), a.intersects_at(&b).count());
    }

    #[test]
    fn line_segment_one() {
        let a = Line::new(2.0, 3.0, 2.0);
        let b = Segment::new((3.0, -3.0).into(), (5.0, -1.0).into());

        assert_eq!(a.intersects(&b), Count::One);
        assert_eq!(a.intersects(&b), a.intersects_at(&b).count());

        let x = a.intersects_at(&b).get_one().unwrap();
        assert!(x.dist((4.0, -2.0).into()) < 1e-6);
    }

    #[test]
    fn line_segment_zero() {
        let a = Line::new(2.0, 3.0, 8.0);
        let b = Segment::new((3.0, -3.0).into(), (5.0, -1.0).into());

        assert_eq!(a.intersects(&b), Count::Zero);
        assert_eq!(a.intersects(&b), a.intersects_at(&b).count());
    }

    #[test]
    fn line_circle_two() {
        let a = Line::new(2.0, 3.0, 2.0);
        let b = Circle::new((1.0, 2.0).into(), 2.0);

        assert_eq!(a.intersects(&b), Count::Many(2));
        assert_eq!(a.intersects(&b), a.intersects_at(&b).count());

        let Intersections::Two(x, y) = a.intersects_at(&b) else {
            unreachable!()
        };
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
        assert_eq!(a.intersects(&b), a.intersects_at(&b).count());
    }

    #[test]
    fn ray_circle_two() {
        let a = Ray::new((-2.0, 1.0).into(), (2.0f64 / 3.0).atan());
        let b = Circle::new((1.0, 2.0).into(), 2.0);

        assert_eq!(a.intersects(&b), Count::Many(2));
        assert_eq!(a.intersects(&b), a.intersects_at(&b).count());

        let Intersections::Two(x, y) = a.intersects_at(&b) else {
            unreachable!()
        };
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
        assert_eq!(a.intersects(&b), a.intersects_at(&b).count());

        let x = a.intersects_at(&b).get_one().unwrap();
        let p = (2.052, 3.701).into();

        assert!(x.dist(p) < 1e-3);
    }

    #[test]
    fn ray_circle_zero() {
        let a = Ray::new((-2.0, 4.0).into(), (2.0f64 / 3.0).atan());
        let b = Circle::new((1.0, 2.0).into(), 2.0);

        assert_eq!(a.intersects(&b), Count::Zero);
        assert_eq!(a.intersects(&b), a.intersects_at(&b).count());
    }

    #[test]
    fn ray_arc_two() {
        let a = Ray::new((-2.0, 1.0).into(), (2.0f64 / 3.0).atan());
        let b = Arc::from_center_ang((1.0, 2.0).into(), 2.0, 0.0, 4.5, true);

        assert_eq!(a.intersects(&b), Count::Many(2));
        assert_eq!(a.intersects(&b), a.intersects_at(&b).count());

        let Intersections::Two(x, y) = a.intersects_at(&b) else {
            unreachable!()
        };
        let p = (-0.975, 1.683).into();
        let q = (2.052, 3.701).into();

        let case_a = (x.dist(p) < 1e-3) && (y.dist(q) < 1e-3);
        let case_b = (x.dist(q) < 1e-3) && (y.dist(p) < 1e-3);

        assert!(case_a || case_b);
    }

    #[test]
    fn ray_arc_one() {
        let a = Ray::new((-2.0, 1.0).into(), (2.0f64 / 3.0).atan());
        let b = Arc::from_center_ang((1.0, 2.0).into(), 2.0, 1.5, 4.5, true);

        assert_eq!(a.intersects(&b), Count::One);
        assert_eq!(a.intersects(&b), a.intersects_at(&b).count());

        let x = a.intersects_at(&b).get_one().unwrap();
        let p = (-0.975, 1.683).into();

        assert!(x.dist(p) < 1e-3);
    }

    #[test]
    fn circle_circle_zero() {
        let a = Circle::new((-3.0, 4.0).into(), 2.0);
        let b = Circle::new((20.0, 3.0).into(), 4.0);

        assert_eq!(a.intersects(&b), Count::Zero);
        assert_eq!(a.intersects(&b), a.intersects_at(&b).count());
    }

    #[test]
    fn circle_circle_two() {
        let a = Circle::new((1.0, -2.0).into(), 5.0);
        let b = Circle::new((4.0, 3.0).into(), 2.0);

        assert_eq!(a.intersects(&b), Count::Many(2));
        assert_eq!(a.intersects(&b), a.intersects_at(&b).count());

        let Intersections::Two(x, y) = a.intersects_at(&b) else {
            unreachable!()
        };
        let p = (2.003, 2.898).into();
        let q = (4.850, 1.190).into();

        println!("{:?}", x);
        println!("{:?}", y);

        let case_a = (x.dist(p) < 1e-3) && (y.dist(q) < 1e-3);
        let case_b = (x.dist(q) < 1e-3) && (y.dist(p) < 1e-3);

        assert!(case_a || case_b);
    }
}
