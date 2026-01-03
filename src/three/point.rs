use std::fmt;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point { x, y, z }
    }

    pub fn origin() -> Self {
        Point {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn mid(&self, other: Point) -> Point {
        // find the midpoint between two points
        Point {
            x: (self.x + other.x) / 2.0,
            y: (self.y + other.y) / 2.0,
            z: (self.z + other.z) / 2.0,
        }
    }

    pub fn norm(self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn dist(self, other: Point) -> f64 {
        (other - self).norm()
    }

    pub fn dot(self, other: Point) -> f64 {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }

    pub fn cross(self, other: Point) -> Point {
        Point {
            x: (self.y * other.z) - (self.z * other.y),
            y: (self.z * other.x) - (self.x * other.z),
            z: (self.x * other.y) - (self.y * other.x),
        }
    }

    pub fn to_unit(self) -> Point {
        let norm = self.norm();
        self / norm
    }

    pub fn to_vec(self) -> Vec<f64> {
        vec![self.x, self.y, self.z]
    }

    pub fn eps_eq(self, other: Self, epsilon: f64) -> bool {
        // shorthand for equality within epsilon
        self.dist(other) <= epsilon
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(")?;
        fmt::Display::fmt(&self.x, f)?;
        write!(f, ", ")?;
        fmt::Display::fmt(&self.y, f)?;
        write!(f, ", ")?;
        fmt::Display::fmt(&self.z, f)?;
        write!(f, ")")
    }
}

impl From<(f64, f64, f64)> for Point {
    fn from(x: (f64, f64, f64)) -> Self {
        Point::new(x.0, x.1, x.2)
    }
}

impl From<Point> for (f64, f64, f64) {
    fn from(x: Point) -> Self {
        (x.x, x.y, x.z)
    }
}

impl From<Point> for Vec<f64> {
    fn from(x: Point) -> Self {
        vec![x.x, x.y, x.z]
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<f64> for Point {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Point {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Div<f64> for Point {
    type Output = Self;

    fn div(self, other: f64) -> Self {
        Point {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}
