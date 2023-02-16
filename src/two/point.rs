use std::ops::{Add, Index, IndexMut, Mul, Div, Sub};

#[derive(Clone, Copy, PartialEq)]
pub struct Point {
	pub x: f64,
	pub y: f64,
};

impl Point {
	pub fn new(x: f64, y: f64) -> Self {
        Point {x, y}
    }

    pub fn unit(ang: f64) -> Self {
    	Point { x: ang.cos(), y: ang.sin() }
    }

    pub fn origin() -> Self {
        Point { x: 0.0, y: 0.0 }
    }

    pub fn mid(&self, other: Point) -> Point {
        // find the midpoint between two points
        Point {x: (self.x + other.x) / 2.0), y: (self.y + other.y) / 2.0}
    }

    pub fn norm(self) -> f64 {
    	(self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn dist(self, other: Point) -> f64 {
        let v = self - other;
        v.norm()
    }

    pub fn dot(self, other: Point) -> f64 {
        (self.x * other.x) + (self.y * other.y)
    }

    pub fn to_unit(self) -> Point {
        let norm = self.norm();
        self / norm
    }

    pub fn to_vec(self) -> Vec<f64> {
        vec![self.x, self.y]
    }

    pub fn ang(self) -> f64 {
    	self.y.atan2(self.x)
    }

    pub fn perp(self) -> Self {
    	Point { x: -1.0 * self.y, y: self.x }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y),
    }
}

impl From<(f64, f64)> for Point {
    fn from(x: (f64, f64)) -> Self {
        Point::new(&[x.0, x.1])
    }
}

impl From<Point> for (f64, f64) {
	fn from(x: Point) -> Self {
		(x.x, x.y)
	}
}

impl From<Point> for Vec<f64> {
    fn from(x: Point) -> Self {
        vec![x.x, x.y]
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
    	Point{x: self.x + other.x, y: self.y + other.y}
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Point{x: self.x - other.x, y: self.y - other.y}
    }
}

impl Mul<f64> for Point {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
    	Point{x: self.x * other, y: self.y * other}
    }
}

impl Div<f64> for Point {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
    	Point{x: self.x / other, y: self.y / other}
    }
}