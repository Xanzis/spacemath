use super::Line;
use super::Plane;
use super::Point;

use crate::two;

#[derive(Clone, Copy, Debug)]
pub struct Triangle(pub Point, pub Point, pub Point);

impl Triangle {
    pub fn into_points(self) -> (Point, Point, Point) {
        (self.0, self.1, self.2)
    }

    pub fn to_plane(self) -> Plane {
        Plane::from_points(self.0, self.1, self.2)
    }

    pub fn unfold(self, x: Point) -> Point {
        // unfold x into plane of triangle ABC along edge BC with point A as the root
        // for unfolded point Y, A and Y are on opposite side of BC
        // for use in MMP geodesic finding

        let bc_line = Line::from_points(self.1, self.2);
        let a = self.0;
        let away = bc_line.away(a); // vector pointing away from A and perp to BC

        let projected = bc_line.project(x);
        let dist = projected.dist(x);

        projected + (away * dist)
    }

    pub fn proj_to_two(self, p: Point) -> two::Point {
        // project to a two dimensional point in an orthonormal coordinate system on the triangle
        let ab = (self.1 - self.0).to_unit();
        let ac = (self.2 - self.0).to_unit();
        let n = ab.cross(ac);
        let u = ab;
        let v = n.cross(ab).to_unit();

        let p = p - self.0;

        let x = p.dot(u);
        let y = p.dot(v);
        (x, y).into()
    }

    pub fn proj_from_two(self, p: two::Point) -> Point {
        // invert proj_to_two
        let ab = (self.1 - self.0).to_unit();
        let ac = (self.2 - self.0).to_unit();
        let n = ab.cross(ac);
        let u = ab;
        let v = n.cross(ab).to_unit();

        let (x, y) = p.into();
        let p = (u * x) + (v * y);
        self.0 + p
    }
}

impl From<(Point, Point, Point)> for Triangle {
    fn from(tri: (Point, Point, Point)) -> Self {
        Self(tri.0, tri.1, tri.2)
    }
}

mod tests {
    #[test]
    fn proj() {
        use super::Triangle;
        use crate::two;

        let a = (1.0, 3.0, 5.0).into();
        let b = (1.0, 4.0, 5.0).into();
        let c = (2.0, 6.0, 2.0).into();

        let t = Triangle(a, b, c);

        let p_a = t.proj_to_two(a);
        let p_b = t.proj_to_two(b);

        let p_a_goal: two::Point = (0.0, 0.0).into();
        let p_b_goal: two::Point = (1.0, 0.0).into();

        let ppa = t.proj_from_two(p_a);
        let ppb = t.proj_from_two(p_b);

        assert!((p_a - p_a_goal).norm() < 1e-6);
        assert!((p_b - p_b_goal).norm() < 1e-6);
        assert!((a - ppa).norm() < 1e-6);
        assert!((b - ppb).norm() < 1e-6);
    }

    #[test]
    fn proj_other() {
        use super::Triangle;
        use crate::two;

        let a = (0.0, 2.5, 0.0).into();
        let b = (-2.0, 0.0, 0.0).into();
        let c = (1.0, -2.0, 0.0).into();

        let t = Triangle(a, b, c);

        let p_c = t.proj_to_two(c);
        let ppc = t.proj_from_two(p_c);

        dbg!(c);
        dbg!(p_c);
        dbg!(ppc);

        assert!((c - ppc).norm() < 1e-6);
    }
}
