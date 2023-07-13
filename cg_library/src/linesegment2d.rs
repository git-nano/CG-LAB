//! LineSegment in a 2-Dimensional vector space.
//!
//! Provides a linesegment struct for the computational geometry library [cg_library](crate).

use crate::line2d::Line2D;
use crate::point2d::Point2D;
use crate::tools2d::ccw;
use std::cmp::Ordering;
use std::fmt;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
/// A line segment in a 2D vector space.
///
/// A line segment consists of two points, that are the endpoints of the segment. It is used in
/// combination with the bently_ottmann algorithm as well as a
/// [Polygon2D](crate::polygon2d::Polygon2D).
///
/// # Example
///
/// ```
/// use cg_library::linesegment2d::LineSegment2D;
/// use cg_library::point2d::Point2D;
/// let p1: Point2D = Point2D::new();
/// let p2: Point2D = Point2D{x: 1.0, y: 1.0};
/// let ls: LineSegment2D = LineSegment2D::new(p1, p2);
/// ```
pub struct LineSegment2D {
    /// The line that runs through this Segment
    pub line: Line2D,

    /// The [point](crate::point2d::Point2D) that has the smallest x-coordinate, or if equal, the smallest y-coordinate
    pub p1: Point2D,

    /// The [point](crate::point2d::Point2D) that has the highest x-coordinate, or if equal, the highest y-coordinate
    pub p2: Point2D,

    /// Maximum x-coordinate of the bounding box of the segment
    pub max_x: f64,

    /// Maximum y-coordinate of the bounding box of the segment
    pub max_y: f64,

    /// Minimum x-coordinate of the bounding box of the segment
    pub min_x: f64,

    /// Minimum y-coordinate of the bounding box of the segment
    pub min_y: f64,
}

/// This trait needs to be implemented to satisfy PartialOrd, it is not yet used.
impl Eq for LineSegment2D {}

/// This trait is implemented to satisfy PartialOrd, and sorts line segments by its starting point.
impl Ord for LineSegment2D {
    fn cmp(&self, other: &LineSegment2D) -> Ordering {
        self.p1.partial_cmp(&other.p1).unwrap()
    }
}

impl LineSegment2D {
    /// Returns a new instance given two points.
    ///
    /// # Panics
    /// This function is not allowed to be given two points that are equal.
    pub fn new(p_a: Point2D, p_b: Point2D) -> LineSegment2D {
        if p_a == p_b {
            panic!("A segment with two Points at the same position is not allowed!");
        }

        let (p1, p2) = if p_a < p_b { (p_a, p_b) } else { (p_b, p_a) };
        let max_x = p1.x.max(p2.x);
        let max_y = p1.y.max(p2.y);
        let min_x = p1.x.min(p2.x);
        let min_y = p1.y.min(p2.y);

        LineSegment2D {
            line: Line2D::from_point_and_point(p1, p2),
            p1,
            p2,
            max_x,
            max_y,
            min_x,
            min_y,
        }
    }

    /// Returns the euclidean distance from start to endpoint.
    pub fn length_xy(self) -> f64 {
        self.p1.distance_to(&self.p2)
    }

    /// Returns `true` iff the point of the argument is among the segment endpoints.
    pub fn has_endpoint(self, p: &Point2D) -> bool {
        self.p1 == *p || self.p2 == *p
    }

    /// Returns the center point of the line segment.
    pub fn center(self) -> Point2D {
        let dx = (self.p1.x - self.p2.x).abs();
        let dy = (self.p1.y - self.p2.y).abs();
        Point2D {
            x: self.p1.x + dx / 2.0,
            y: self.p1.y + dy / 2.0,
        }
    }

    /// Returns `true` iff a point is element of the segment.
    pub fn contains(self, p: &Point2D) -> bool {
        self.has_endpoint(p)
            || (self.line.contains(p)
                && (p.x >= self.min_x
                    && p.x <= self.max_x
                    && p.y >= self.min_y
                    && p.y <= self.max_y))
    }

    /// Calculate the intersection point with another line segment.
    ///
    /// This returns an intersection point to another line segment if exists. If not `None` is
    /// returned. This function uses the counter clock wise ([ccw](crate::tools2d::ccw)) implementation.
    /// If the lines overlap colinear, also `None` is returned.
    pub fn intersects(self, other: &LineSegment2D) -> Option<Point2D> {
        let (p1, p2, q1, q2) = (self.p1, self.p2, other.p1, other.p2);

        if self == *other {
            return None;
        }

        if self.has_endpoint(&q1) {
            return Some(q1);
        }

        if self.has_endpoint(&q2) {
            return Some(q2);
        }

        if ccw(&p1, &p2, &q1) * ccw(&p1, &p2, &q2) <= 0.0
            && ccw(&q1, &q2, &p1) * ccw(&q1, &q2, &p2) <= 0.0
        {
            return self.line.intersection(&other.line);
        }

        None
    }

    /// This prints a geogebra style object that can be copied into the [geogebra calculator](https://www.geogebra.org/calculator).
    pub fn geogebra(self) {
        println!(
            "Segment(({},{}),({},{}))",
            self.p1.x, self.p1.y, self.p2.x, self.p2.y
        );
    }
}

/// This trait allows a line segment to be displayed in the form of `p1: (x1,y1), p2: (x2,y2)`.
impl fmt::Display for LineSegment2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "p1: {}, p2: {}", self.p1, self.p2)
    }
}

#[cfg(test)]
mod test_linesegemnt2d {
    use super::*;

    #[test]
    fn test_new() {
        let s1 = LineSegment2D::new(Point2D { x: 0.0, y: 0.0 }, Point2D { x: 0.0, y: 1.0 });
        let s2 = LineSegment2D::new(Point2D { x: 0.0, y: 0.0 }, Point2D { x: 1.0, y: 0.0 });
        let s3 = LineSegment2D::new(Point2D { x: 1.0, y: 0.0 }, Point2D { x: 0.0, y: 0.0 });
        let s4 = LineSegment2D::new(Point2D { x: 0.0, y: 1.0 }, Point2D { x: 0.0, y: 0.0 });
        assert_eq!("p1: (0,0), p2: (0,1)", s1.to_string());
        assert_eq!("p1: (0,0), p2: (1,0)", s2.to_string());
        assert_eq!("p1: (0,0), p2: (1,0)", s3.to_string());
        assert_eq!("p1: (0,0), p2: (0,1)", s4.to_string());
    }

    #[test]
    #[should_panic]
    fn test_new_fail() {
        let _s1 = LineSegment2D::new(Point2D { x: 0.0, y: 0.0 }, Point2D { x: 0.0, y: 0.0 });
    }

    #[test]
    fn test_line() {
        let s1 = LineSegment2D::new(Point2D { x: 1.0, y: 0.0 }, Point2D { x: 1.0, y: 1.0 });
        assert_eq!("f(y) -> +1", s1.line.to_string());
    }

    #[test]
    fn test_length() {
        let s1 = LineSegment2D::new(Point2D { x: 1.0, y: 0.0 }, Point2D { x: 1.0, y: 1.0 });
        assert_eq!(1.0, s1.length_xy());
    }

    #[test]
    fn test_equals() {
        let s1 = LineSegment2D::new(Point2D { x: 1.0, y: 0.0 }, Point2D { x: 1.0, y: 1.0 });
        let s2 = LineSegment2D::new(Point2D { x: 1.0, y: 0.0 }, Point2D { x: 1.0, y: 1.0 });
        assert!(s1 == s2);
    }

    #[test]
    fn test_center() {
        let s1 = LineSegment2D::new(Point2D { x: 0.0, y: 0.0 }, Point2D { x: 2.0, y: 2.0 });
        assert_eq!(Point2D { x: 1.0, y: 1.0 }, s1.center());
    }

    #[test]
    fn test_contains() {
        let s1 = LineSegment2D::new(Point2D { x: 0.0, y: 0.0 }, Point2D { x: 2.0, y: 2.0 });
        let p1 = Point2D { x: 1.0, y: 1.0 };
        let p2 = Point2D { x: 2.0, y: 1.0 };
        assert_eq!(true, s1.contains(&p1));
        assert_eq!(false, s1.contains(&p2));
    }

    #[test]
    fn test_intersections() {
        // s1 and s2 are colinear and they share one endpoint (2,2)
        let s1 = LineSegment2D::new(Point2D { x: 0.0, y: 0.0 }, Point2D { x: 2.0, y: 2.0 });
        let s2 = LineSegment2D::new(Point2D { x: 3.0, y: 3.0 }, Point2D { x: 2.0, y: 2.0 });
        assert_eq!(Some(Point2D { x: 2.0, y: 2.0 }), s1.intersects(&s2));

        // s1 and s2 share one endpoint (0,0)
        let s1 = LineSegment2D::new(Point2D { x: 0.0, y: 0.0 }, Point2D { x: 2.0, y: 2.0 });
        let s2 = LineSegment2D::new(Point2D { x: 0.0, y: 0.0 }, Point2D { x: 2.0, y: -2.0 });
        assert_eq!(Some(Point2D { x: 0.0, y: 0.0 }), s1.intersects(&s2));

        // s1 and s2 are colinear and don't overlap
        let s1 = LineSegment2D::new(Point2D { x: 0.0, y: 0.0 }, Point2D { x: 2.0, y: 2.0 });
        let s2 = LineSegment2D::new(Point2D { x: 3.0, y: 3.0 }, Point2D { x: 2.5, y: 2.5 });
        assert_eq!(None, s1.intersects(&s2));

        // s1 and s2 are colinear and overlap
        let s1 = LineSegment2D::new(Point2D { x: 0.0, y: 0.0 }, Point2D { x: 2.0, y: 2.0 });
        let s2 = LineSegment2D::new(Point2D { x: 3.0, y: 3.0 }, Point2D { x: 1.5, y: 1.5 });
        assert_eq!(None, s1.intersects(&s2));

        // s1 and s2 share no point
        let s1 = LineSegment2D::new(Point2D { x: 0.0, y: 0.0 }, Point2D { x: 2.0, y: 2.0 });
        let s2 = LineSegment2D::new(Point2D { x: -2.0, y: 1.0 }, Point2D { x: 1.0, y: -2.0 });
        assert_eq!(None, s1.intersects(&s2));

        // s1 and s2 share one point at (1,1)
        let s1 = LineSegment2D::new(Point2D { x: 0.0, y: 0.0 }, Point2D { x: 2.0, y: 2.0 });
        let s2 = LineSegment2D::new(Point2D { x: 0.0, y: 2.0 }, Point2D { x: 2.0, y: 0.0 });
        assert_eq!(Some(Point2D { x: 1.0, y: 1.0 }), s1.intersects(&s2));

        // s1 and s2 are equal
        let s1 = LineSegment2D::new(Point2D { x: 0.0, y: 0.0 }, Point2D { x: 2.0, y: 2.0 });
        let s2 = LineSegment2D::new(Point2D { x: 0.0, y: 0.0 }, Point2D { x: 2.0, y: 2.0 });
        assert_eq!(None, s1.intersects(&s2));
    }
}
