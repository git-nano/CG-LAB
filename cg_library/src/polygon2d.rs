//! Polygon in a 2-Dimensional vector space.
//!
//! Provides a polygon struct for the computational geometry library [cg_library](crate).

use crate::linesegment2d::LineSegment2D;
use crate::point2d::Point2D;
use crate::tools2d::ccw;

/// A polygon in a 2-Dimensional vector space.
///
/// This polygon is the basis unit for regions in a 2D space. It builds the fundament for the usage
/// of all polygons in a 2D space and can be used for the area of a country.
///
/// # Example
///
/// ```
/// use cg_library::point2d::Point2D;
/// use cg_library::polygon2d::Polygon2D;
///
/// let points = vec![
///     Point2D { x: 0.0, y: 0.0 },
///     Point2D { x: 1.0, y: 1.0 },
///     Point2D { x: 1.0, y: 2.0 },
///     Point2D { x: 2.0, y: 2.0 },
///     Point2D { x: 3.0, y: 3.0 },
///     Point2D { x: 3.0, y: 0.0 },
/// ];
/// let poly: Polygon2D = Polygon2D::new(points);
///
/// ```
#[derive(Debug, Clone)]
pub struct Polygon2D {
    /// All points of the polygon
    pub points: Vec<Point2D>,

    /// All segments of the polygon
    segments: Vec<LineSegment2D>,

    /// Maximum x value of the bounding box of the Segment
    max_x: f64,

    /// Maximum y value of the bounding box of the Segment
    max_y: f64,

    /// Minimum x value of the bounding box of the Segment
    min_x: f64,

    /// Minimum y value of the bounding box of the Segment
    min_y: f64,
}

impl Polygon2D {
    /// Returns an instance of a polygon initialized with a vector of points.
    ///
    /// If the first point of the vector does not fit the last, the first is appended to make a
    /// closed polygon.
    ///
    /// # Panics
    ///
    /// This function will panic if there are not at least $2$ points.
    pub fn new(mut points: Vec<Point2D>) -> Polygon2D {
        if points.len() <= 2 {
            panic!("A polygon consisting of two points is no polygon!");
        } else if points.first() != points.last() {
            println!("The first point does not match the last point!\nIt gets appended now!");
            points.push(*points.first().unwrap());
        }

        let mut segments: Vec<LineSegment2D> = Vec::new();

        let mut old_point = points.first().unwrap();

        let mut min_x = std::f64::MAX.min(old_point.x);
        let mut max_x = std::f64::MIN.max(old_point.x);
        let mut min_y = std::f64::MAX.min(old_point.y);
        let mut max_y = std::f64::MIN.max(old_point.y);
        for point in points.iter().skip(1) {
            segments.push(LineSegment2D::new(*old_point, *point));
            old_point = point;
            min_x = min_x.min(point.x);
            max_x = max_x.max(point.x);
            min_y = min_y.min(point.y);
            max_y = max_y.max(point.y);
        }

        return Polygon2D {
            points,
            segments,
            max_x,
            max_y,
            min_x,
            min_y,
        };
    }

    /// Prints out all the points and segments of the polygon.
    pub fn print(&self) {
        println!("Points:");
        for i in &self.points {
            println!("{}", i);
        }
        println!("Segments:");
        for i in &self.segments {
            println!("{}", i);
        }
    }

    /// Returns `true` iff a point `p` is inside a polygon.
    ///
    /// This does not work for all points ontop of the polygon.
    pub fn contains(&self, q: &Point2D) -> bool {
        // Get a point outside of the polygon
        let p_outside: Point2D = Point2D {
            x: self.max_x + 1.0,
            y: self.max_y + 1.0,
        };

        // Retrieve polygon point that is not part of the segment p_outside q
        let mut i = 0;
        while 0.0 == ccw(&p_outside, &q, &self.points[i]) {
            i += 1;
        }

        let mut s = 0;
        let mut lr = ccw(&p_outside, q, &self.points[i]).signum();
        for j in (i + 1)..self.points.len() {
            // let g = if j == self.points.len() {0} else {j};
            let lrnew = ccw(&p_outside, q, &self.points[j]).signum();
            // println!("\nlr: {}, lrnew: {}, j: {}",&lr, &lrnew, &j);
            if (lrnew - lr).abs() == 2.0 {
                lr = lrnew;
                if ccw(&self.points[j - 1], &self.points[j], &p_outside)
                    * ccw(&self.points[j - 1], &self.points[j], &q)
                    <= 0.0
                {
                    s += 1;
                }
            }
        }
        return s % 2 == 1;
    }

    /// Returns `true` iff a point `p` is inside or ontop of the polygon.
    pub fn contains_point(&self, p: &Point2D) -> bool {
        let mut crossings = 0;
        let n = self.points.len();

        for i in 0..n {
            let j = (i + 1) % n;

            if (self.points[i].y <= p.y && p.y < self.points[j].y)
                || (self.points[j].y <= p.y && p.y < self.points[i].y)
            {
                if p.x
                    < (self.points[j].x - self.points[i].x) * (p.y - self.points[i].y)
                        / (self.points[j].y - self.points[i].y)
                        + self.points[i].x
                {
                    crossings += 1;
                }
            }
        }

        crossings % 2 == 1
    }

    /// Returns the area of the polygon, if the polygon is counter clockwise the area is negative.
    pub fn calculate_area(&self) -> f64 {
        let mut area = 0.0;
        for i in 0..(self.points.len() - 1) {
            area += ccw(&&Point2D::new(), &self.points[i], &self.points[i + 1]);
        }
        return area * 0.5;
    }

    /// Returns `true` iff all points of another polygon is inside the polygon.
    pub fn contains_polygon(&self, poly: &Polygon2D) -> bool {
        for point in &poly.points {
            if !self.contains_point(point) {
                return false;
            }
        }
        return true;
    }
}

#[cfg(test)]
mod test_polygon {
    use super::*;

    #[test]
    fn test_new() {
        let points = vec![
            Point2D { x: 0.0, y: 0.0 },
            Point2D { x: 1.0, y: 1.0 },
            Point2D { x: 1.0, y: 2.0 },
            Point2D { x: 2.0, y: 2.0 },
            Point2D { x: 3.0, y: 3.0 },
        ];

        let _ = Polygon2D::new(points);
        // poly.print();
    }

    #[test]
    #[should_panic]
    fn test_panic() {
        let points = vec![Point2D { x: 0.0, y: 0.0 }, Point2D { x: 1.0, y: 1.0 }];
        let _ = Polygon2D::new(points);
    }

    #[test]
    fn test_inside() {
        let points = vec![
            Point2D { x: 0.0, y: 0.0 },
            Point2D { x: 1.0, y: 1.0 },
            Point2D { x: 1.0, y: 2.0 },
            Point2D { x: 2.0, y: 2.0 },
            Point2D { x: 3.0, y: 3.0 },
            Point2D { x: 3.0, y: 0.0 },
        ];
        let poly: Polygon2D = Polygon2D::new(points);

        // This point is a point on the polygon
        let p1: Point2D = Point2D { x: 1.0, y: 1.0 };
        assert_eq!(true, poly.contains(&p1));

        // This point is a point on the polygon line
        let p1: Point2D = Point2D { x: 2.0, y: 1.0 };
        assert_eq!(true, poly.contains(&p1));

        // This point is a point on the polygon line
        let p1: Point2D = Point2D { x: 1.0, y: 3.0 };
        assert_eq!(false, poly.contains(&p1));

        // This point is a point on the polygon line
        let p1: Point2D = Point2D { x: 1.1, y: 1.9 };
        assert_eq!(true, poly.contains(&p1));

        // This point is a point on the polygon line
        let p1: Point2D = Point2D { x: 0.9, y: 1.1 };
        assert_eq!(false, poly.contains(&p1));

        // This point is a point on the polygon line
        let p1: Point2D = Point2D { x: 0.5, y: 0.5 };
        assert_eq!(true, poly.contains(&p1));

        // poly.print();

        // 3 - # - # - # - o
        // 2 - # - o - o - #
        // 1 - # - o - # - #
        // 0 - o - # - # - #
        //     0 - 1 - 2 - 3
    }

    #[test]
    fn test_should_work() {
        let points = vec![
            Point2D { x: 1.0, y: 1.0 },
            Point2D { x: 1.0, y: 2.0 },
            Point2D { x: 2.0, y: 2.0 },
        ];
        let poly: Polygon2D = Polygon2D::new(points);

        // This point is a point on the polygon line
        let p1: Point2D = Point2D { x: 1.0, y: 1.5 };
        // This is wrong !!!!!
        assert_ne!(true, poly.contains(&p1));
    }

    #[test]
    fn test_contains_point() {
        let points = vec![
            Point2D { x: 0.0, y: 0.0 },
            Point2D { x: 1.0, y: 1.0 },
            Point2D { x: 1.0, y: 2.0 },
            Point2D { x: 2.0, y: 2.0 },
            Point2D { x: 3.0, y: 3.0 },
            Point2D { x: 3.0, y: 0.0 },
        ];
        let poly: Polygon2D = Polygon2D::new(points);

        // This point is a point on the polygon
        let p1: Point2D = Point2D { x: 1.0, y: 1.0 };
        assert_eq!(true, poly.contains_point(&p1));

        // This point is a point on the polygon line
        let p1: Point2D = Point2D { x: 2.0, y: 1.0 };
        assert_eq!(true, poly.contains_point(&p1));

        // This point is a point on the polygon line
        let p1: Point2D = Point2D { x: 1.0, y: 3.0 };
        assert_eq!(false, poly.contains_point(&p1));

        // This point is a point on the polygon line
        let p1: Point2D = Point2D { x: 1.1, y: 1.9 };
        assert_eq!(true, poly.contains_point(&p1));

        // This point is a point on the polygon line
        let p1: Point2D = Point2D { x: 0.9, y: 1.1 };
        assert_eq!(false, poly.contains_point(&p1));

        // This point is a point on the polygon line
        let p1: Point2D = Point2D { x: 0.5, y: 0.5 };
        assert_eq!(true, poly.contains_point(&p1));

        // This point is a point on the polygon line
        let p1: Point2D = Point2D { x: 1.0, y: 1.5 };
        assert_eq!(true, poly.contains_point(&p1));

        // 3 - # - # - # - o
        // 2 - # - o - o - #
        // 1 - # - o - # - #
        // 0 - o - # - # - #
        //     0 - 1 - 2 - 3
    }

    #[test]
    fn test_area() {
        let points = vec![
            Point2D { x: 1.0, y: 1.0 },
            Point2D { x: 1.0, y: 2.0 },
            Point2D { x: 2.0, y: 2.0 },
            Point2D { x: 2.0, y: 1.0 },
            Point2D { x: 1.0, y: 1.0 },
        ];

        let poly = Polygon2D::new(points);
        assert_eq!(-1.0, poly.calculate_area());

        let points = vec![
            Point2D { x: 1.0, y: 1.0 },
            Point2D { x: 1.0, y: 2.0 },
            Point2D { x: 2.0, y: 2.0 },
            Point2D { x: 2.0, y: 1.0 },
        ];

        let poly = Polygon2D::new(points);
        assert_eq!(-1.0, poly.calculate_area());
    }
}
