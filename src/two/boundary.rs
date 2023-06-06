use super::dist::Dist;
use super::intersect::{reflexive_intersect, Intersect};
use super::line::Ray;
use super::Point;

#[derive(Clone, Copy, Debug)]
pub enum Edge {
    Arc(super::Arc),
    Segment(super::Segment),
}

impl Edge {
    pub fn p(&self) -> Point {
        match self {
            Edge::Arc(a) => a.p(),
            Edge::Segment(s) => s.p(),
        }
    }

    pub fn q(&self) -> Point {
        match self {
            Edge::Arc(a) => a.q(),
            Edge::Segment(s) => s.q(),
        }
    }

    pub fn shoelace(self) -> f64 {
        // for use in gauss' area formula
        match self {
            Edge::Segment(s) => s.p().shoelace(s.q()) / 2.0,
            Edge::Arc(a) => {
                // a little hacky? There's probably an analytic solution for the triangle-arc area
                let points = a.sample_points(10);
                points
                    .iter()
                    .zip(points.iter().skip(1))
                    .map(|(p, q)| p.shoelace(*q))
                    .sum::<f64>()
                    / 2.0
            }
        }
    }

    pub fn reverse(self) -> Self {
        match self {
            Edge::Segment(s) => Edge::Segment(s.reverse()),
            Edge::Arc(a) => Edge::Arc(a.reverse()),
        }
    }

    pub fn into_segments(self, len: f64) -> Vec<Self> {
        // splits the edge into segments, with a target length len
        // if len is too large, only returns one segment from p to q

        match self {
            Edge::Segment(s) => vec![Edge::Segment(s)],
            Edge::Arc(a) => {
                let intervals = (a.arc_length() / len).max(1.0).round() as usize;
                let points = a.sample_points(intervals + 1);

                let mut res = Vec::new();
                for pq in points.windows(2) {
                    res.push(Edge::Segment(super::Segment::new(pq[0], pq[1])));
                }

                res
            }
        }
    }
}

impl Intersect<Edge> for Edge {
    fn intersects_at(&self, other: &Edge) -> Vec<Point> {
        use Edge::*;

        match (*self, *other) {
            (Arc(x), Arc(y)) => x.intersects_at(&y),
            (Arc(x), Segment(y)) => x.intersects_at(&y),
            (Segment(x), Arc(y)) => x.intersects_at(&y),
            (Segment(x), Segment(y)) => x.intersects_at(&y),
        }
    }
}

impl Intersect<Ray> for Edge {
    fn intersects_at(&self, other: &Ray) -> Vec<Point> {
        match self {
            Edge::Arc(a) => a.intersects_at(other),
            Edge::Segment(s) => s.intersects_at(other),
        }
    }
}

reflexive_intersect!(Ray, Edge);

impl From<super::Arc> for Edge {
    fn from(x: super::Arc) -> Self {
        Self::Arc(x)
    }
}

impl From<super::Segment> for Edge {
    fn from(x: super::Segment) -> Self {
        Self::Segment(x)
    }
}

impl Dist for Edge {
    fn dist(&self, r: Point) -> f64 {
        match self {
            &Edge::Arc(ref a) => a.dist(r),
            &Edge::Segment(ref c) => c.dist(r),
        }
    }
}

// a closed 2d boundary
#[derive(Clone, Debug)]
pub struct Boundary {
    edges: Vec<Edge>,
    points: Vec<Point>,
}

impl Boundary {
    pub fn new<T, U>(edges: T) -> Self
    where
        T: IntoIterator<Item = U>,
        U: Into<Edge>,
    {
        let edges: Vec<_> = edges.into_iter().map(|e| e.into()).collect();

        assert!(!edges.is_empty());

        // check the nodes all agree
        // TODO replace with some tolerance interface
        assert!(edges
            .iter()
            .zip(edges.iter().skip(1))
            .all(|(e1, e2)| e1.q().dist(e2.p()) < 1e-6));

        let start = edges.first().unwrap().p();
        let end = edges.last().unwrap().q();
        assert!(start.dist(end) < 1e-6);

        let points = edges.iter().map(|e| e.p()).collect();

        Self { edges, points }
    }

    pub fn reverse(&mut self) {
        let rev_edges = self
            .edges
            .iter()
            .rev()
            .map(|e| e.clone().reverse())
            .collect();
        self.edges = rev_edges;
    }

    pub fn area(&self) -> f64 {
        self.edges.iter().map(|e| e.shoelace()).sum()
    }

    pub fn points(&self) -> Vec<Point> {
        // maybe lend instead of clone
        self.points.clone()
    }

    pub fn num_edges(&self) -> usize {
        self.edges.len()
    }

    pub fn edges<'a>(&'a self) -> impl Iterator<Item = &'a Edge> + 'a {
        self.edges.iter()
    }

    pub fn orient_positive(&mut self) {
        // order the edges so the boundary is positively oriented (counterclockwise)
        if self.area() < 0.0 {
            self.reverse();
        }
    }

    pub fn orient_negative(&mut self) {
        if self.area() > 0.0 {
            self.reverse();
        }
    }

    pub fn contains(&self, x: Point) -> bool {
        // check whether x is inside the boundary (whether or not boundary is oriented positively)
        let ray = Ray::new(x, 0.1337); // use non-horizontal ray to avoid common edge case of coincident lines

        let mut even_crossing = true;

        for e in self.edges.iter() {
            if ray.intersects(e).is_odd() {
                even_crossing = !even_crossing;
            }
        }

        !even_crossing
    }

    pub fn contains_boundary(&self, other: &Boundary) -> bool {
        // panic if the boundaries intersect, TODO find better solution
        assert!(self.intersects(other).is_zero());

        // note: if boundaries do not intersect arcs don't need to be handled separately
        // also, only need to check one point, if one is contained all are
        self.contains(other.points[0])
    }

    pub fn bounding_box(&self) -> (Point, Point) {
        // finds the (left bottom, right top) corners of the boundary's bounding box

        let points = self.points();

        let x_max = points
            .iter()
            .map(|p| p.x)
            .max_by(|x, y| x.partial_cmp(y).unwrap())
            .unwrap();
        let y_max = points
            .iter()
            .map(|p| p.y)
            .max_by(|x, y| x.partial_cmp(y).unwrap())
            .unwrap();
        let x_min = points
            .iter()
            .map(|p| p.x)
            .min_by(|x, y| x.partial_cmp(y).unwrap())
            .unwrap();
        let y_min = points
            .iter()
            .map(|p| p.y)
            .min_by(|x, y| x.partial_cmp(y).unwrap())
            .unwrap();

        ((x_min, y_min).into(), (x_max, y_max).into())
    }
}

impl Intersect<Boundary> for Boundary {
    fn intersects_at(&self, other: &Boundary) -> Vec<Point> {
        let mut res = Vec::new();

        // n^2 approach, check every possible edge pairing
        let pairings = self
            .edges
            .iter()
            .flat_map(|e| std::iter::repeat(e).zip(&other.edges));
        for (e1, e2) in pairings {
            res.extend(e1.intersects_at(e2));
        }

        res
    }
}

impl Dist for Boundary {
    fn dist(&self, r: Point) -> f64 {
        self.edges()
            .map(|e| e.dist(r))
            .min_by(|x, y| x.partial_cmp(y).unwrap())
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn square_bound() {
        use super::super::line::Segment;
        use super::Boundary;

        let edges = vec![
            Segment::new((0.0, 0.0).into(), (1.0, 0.0).into()),
            Segment::new((1.0, 0.0).into(), (1.0, 1.0).into()),
            Segment::new((1.0, 1.0).into(), (0.0, 1.0).into()),
            Segment::new((0.0, 1.0).into(), (0.0, 0.0).into()),
        ];

        let mut bound = Boundary::new(edges);

        assert!((bound.area() - 1.0) < 1e-6);
        assert!(bound.contains((0.5, 0.5).into()));

        bound.reverse();
        assert!((bound.area() + 1.0) < 1e-6);
        assert!(bound.contains((0.5, 0.5).into()));
    }

    #[test]
    fn circle_bound() {
        use super::super::line::Arc;
        use super::Boundary;

        let edges = vec![Arc::from_center_ang((0.0, 0.0).into(), 2.0, 0.0, 0.0, true)];

        let bound = Boundary::new(edges);
        assert!(bound.contains((0.1, 0.4).into()));
    }

    #[test]
    fn bounds_intersect() {
        use super::super::line::{Arc, Segment};
        use super::Boundary;
        use super::Intersect;

        let edges = vec![
            Segment::new((0.0, 0.0).into(), (1.0, 0.0).into()),
            Segment::new((1.0, 0.0).into(), (1.0, 1.0).into()),
            Segment::new((1.0, 1.0).into(), (0.0, 1.0).into()),
            Segment::new((0.0, 1.0).into(), (0.0, 0.0).into()),
        ];

        let mut square_bound = Boundary::new(edges);

        let edges = vec![Arc::from_center_ang((0.0, 0.0).into(), 1.0, 0.0, 0.0, true)];

        let circle_bound = Boundary::new(edges);

        assert!(!circle_bound.intersects(&square_bound).is_zero());
    }

    #[test]
    fn bounds_contain() {
        use super::super::line::{Arc, Segment};
        use super::Boundary;
        use super::Intersect;

        let edges = vec![
            Segment::new((0.1, 0.0).into(), (1.0, 0.0).into()),
            Segment::new((1.0, 0.0).into(), (1.0, 1.0).into()),
            Segment::new((1.0, 1.0).into(), (0.1, 1.0).into()),
            Segment::new((0.1, 1.0).into(), (0.1, 0.0).into()),
        ];

        let mut square_bound = Boundary::new(edges);

        let edges: Vec<super::Edge> = vec![
            Arc::from_center_ang(
                (0.0, 0.0).into(),
                2.0,
                (3.0 * std::f64::consts::PI) / 2.0,
                std::f64::consts::PI / 2.0,
                true,
            )
            .into(),
            Segment::new((0.0, 2.0).into(), (0.0, -2.0).into()).into(),
        ];

        dbg!(edges[0].p());
        dbg!(edges[0].q());

        let d_bound = Boundary::new(edges);

        assert!(d_bound.contains_boundary(&d_bound));
    }
}
