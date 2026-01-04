use super::Line;
use super::Plane;
use super::Point;

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
}

impl From<(Point, Point, Point)> for Triangle {
    fn from(tri: (Point, Point, Point)) -> Self {
        Self(tri.0, tri.1, tri.2)
    }
}
