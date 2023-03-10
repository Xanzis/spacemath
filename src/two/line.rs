use super::Point;

// a line, represented as ax + by = c
#[derive(Clone, Copy, Debug)]
pub struct Line {
    pub (crate) a: f64,
    pub (crate) b: f64,
    pub (crate) c: f64,
}

impl Line {
	pub fn new(a: f64, b: f64, c: f64) -> Self {
		Self {a, b, c}
	}

	pub fn perp_origin(&self) -> Line {
		// a perpendicular line passing through the origin
		let c = 0.0;
        let a = -1.0 * self.b;
        let b = self.a;
        Line { a, b, c }
	}

	// public to crate only - use the Shift trait
	pub(crate) fn offset_x(&mut self, u: f64) {
		// shift the line by some x value
		// a(x-u) + by = c => ax + by = c + au
		
		self.c += self.a * u;
	}

	pub(crate) fn offset_y(&mut self, v: f64) {
		// shift the line by some y value
		// ax + b(y - v) = c => ax + by = c + bv

		self.c += self.b * v;
	}
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Line ({} x + {} y = {})", self.a, self.b, self.c),
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Segment {
	p: Point,
	q: Point,
}

impl Segment {
	pub fn to_line(self) -> Line {
		let a = self.q.y - self.p.y;
        let b = self.p.x - self.q.x;
        let c = a * self.p.x + b * self.p.y;
        Line::new(a, b, c)
	}

	pub fn mid(&self) -> Point {
		self.p.mid(self.q)
	}

	pub fn perp_bisect(s: Segment) -> Line {
        // contruct a perpendicular bisector of pq
        let mid = s.mid();
        let along = s.to_line();

        let c = (-1.0 * along.b * mid.x) + (along.a * mid.y);
        let a = -1.0 * along.b;
        let b = along.a;
        Line::new(a, b, c)
    }

	pub fn bounds_contain(&self, r: Point) -> bool {
    	// find whether r lies within the segment's bounding box
    	// useful for checking if a colinear point is on the segment
    	let (p, q) = (self.p, self.q);
    	r.x <= p.x.max(q.x)
        	&& r.x >= p.x.min(q.x)
        	&& r.y <= p.y.max(q.y)
        	&& r.y >= p.y.min(q.y)
	}
}

#[derive(Clone, Copy, Debug)]
pub struct Ray {
	pub (crate) init: Point,
	pub (crate) ang: f64,
}

impl Ray {
	pub fn new(init: Point, ang: f64) -> Self {
		Ray { init, ang }
	}

	pub fn dir(&self) -> Point {
		// unit vector representing the pointing direction
		Point::unit(self.ang)
	}

	pub fn to_line(self) -> Line {
		// coincident line
		let a = -1.0 * self.ang.sin();
		let b = self.ang.cos();
		let c = (a * self.init.x) + (b * self.init.y);
		Line { a, b, c }
	}

	// public to crate only - use the Shift trait
	pub(crate) fn offset_x(&mut self, u: f64) {
		self.init.x += u;
	}

	pub(crate) fn offset_y(&mut self, v: f64) {
		self.init.y += v;
	}
}

#[derive(Clone, Copy, Debug)]
pub struct Circle {
	pub (crate) center: Point,
	pub (crate) radius: f64,
}

impl Circle {
	pub fn new(center: Point, radius: f64) -> Self {
		Circle {center, radius}
	}

	// public to crate only - use the Shift trait
	pub(crate) fn offset_x(&mut self, u: f64) {
		self.center.x += u;
	}

	pub(crate) fn offset_y(&mut self, v: f64) {
		self.center.y += v;
	}
}

#[derive(Clone, Copy, Debug)]
pub struct Arc {
	center: Point,
	radius: f64,
	p_ang: f64,
	q_ang: f64,
}

impl Arc {
	pub fn from_center_ang(center: Point, radius: f64, p_ang: f64, q_ang: f64) -> Self {
		assert!(q_ang > p_ang);
		Arc { center, radius, p_ang, q_ang }
	}

	pub fn to_circle(self) -> Circle {
		Circle {center: self.center, radius: self.radius}
	}

	pub fn p(&self) -> Point {
		(Point::unit(self.p_ang) * self.radius) + self.center
	}

	pub fn q(&self) -> Point {
		(Point::unit(self.p_ang) * self.radius) + self.center
	}

	// public to crate only - use the Shift trait
	pub(crate) fn offset_x(&mut self, u: f64) {
		self.center.x += u;
	}

	pub(crate) fn offset_y(&mut self, v: f64) {
		self.center.y += v;
	}

	pub fn bounds_contain(&self, r: Point) -> bool {
		// find whether r lies on the wedge described by center, p, and q
		// useful for determining whether a coradial r lies in the arc
		let ang = r.ang();
		ang >= p_ang && ang <= q_ang;
	}
}