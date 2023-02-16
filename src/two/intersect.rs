use super::Point;
use super::shift::Shift;

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
	// count of intersections
	fn intersects(&self, other: &T) -> Count;

	fn intersects_at(&self, other: &T) -> Vec<Point> {
		unimplemented!()
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
				let x = ((self.b * other.c) - (self.c * other.b)) /
					((self.b * other.a) - (self.a * other.b));
				let y = ((self.c * other.a) - (self.a * other.c)) /
					((self.b * other.a) - (self.a * other.b));

				vec![Point::new(x, y)]
			},
			_ => Vec::new()
		}
	}
}

impl Intersect<Line> for Segment {
	fn intersects(&self, other: &Line) -> Count {
		other.intersects_at().count().into()
	}

	fn intersects_at(&self, other: &Line) -> Vec<Point> {
		self.to_line().intersects_at(*other).into_iter()
			.filter(|p| self.bounds_contain(p)).collect()
	}
}

impl Intersect<Segment> for Line {
	fn intersects(&self, other: &Segment) -> Count {
		other.intersects(&self)
	}

	fn intersects_at(&self, other: &Segment) -> Vec<Point> {
		other.intersects_at(&self)
	}
}

impl Intersect<Circle> for Line {
	fn intersects(&self, other: &Line) -> Count {
		other.intersects_at().count().into()
	}

	fn intersects_at(&self, other: &Line) -> Vec<Point> {
		// simplify the problem to a line through a circle at the origin
		let r = self.radius;
		let shifted_line = other.shift_subtract(self.center);

		// awful awful approach
		// find the perpendicular line through the origin
		// and the distance to the intersection with that line
		let perp = shifted_line.perp_origin();
		let inter = perp.intersects_at(&shifted_line)[0]
		let dist = inter.norm();

		if dist == r {
			// shift back to original coordinate frame
			return vec![inter + self.center]
		} else if dist > r {
			return vec![]
		}

		// half of the chord of the circle
		let half_chord = (dist/r).asin().cos() * r;

		let unit_radial = inter.to_unit();
		let unit_tangential = unit_radial.perp();

		let p1 = (unit_radial * dist) + (unit_tangential * half_chord);
		let p2 = (unit_radial * dist) - (unit_tangential * half_chord);

		// shift back to original coordinate frame
		vec![p1 + self.center, p2 + self.center]
	}
}