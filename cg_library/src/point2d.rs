//! Point in a 2-Dimensional vector space.
//!
//! Provides a point struct for the computational geometry library [cg_library](crate).

use crate::tools2d::round_to_decimal_places;
use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Sub};

/// A Point in a 2-Dimensional vector space.
///
/// The point is the basis unit of our 2D space and builds the foundament behind various constructs
/// like [Polygon2D](crate::polygon2d::Polygon2D), [LineSegment2D](crate::linesegment2d::LineSegment2D) and [Line2D](crate::line2d::Line2D).
///
/// # Example
///
/// This creates a zero-initialized point:
/// ```
/// use cg_library::point2d::Point2D;
/// let mut p = Point2D::new();
/// ```
///
/// This creates a point with two coordinates:
/// ```
/// use cg_library::point2d::Point2D;
/// let p: Point2D = Point2D {x: 1.0, y: 2.0};
/// ```
///
#[derive(Default, Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Point2D {
    /// The x-coordinate.
    pub x: f64,
    /// The y-coordinate.
    pub y: f64,
}

/// This trait needs to be implemented to satisfy PartialOrd, it is not yet used.
impl Eq for Point2D {}

/// This trait is added to allow points to be ordered.
///
/// Points are ordered after rising x-coordinates at first and at tie after the y-coordinates.
impl Ord for Point2D {
    fn cmp(&self, other: &Point2D) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

/// This trait allows implicit addition of one point and another.
impl Add for Point2D {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

/// This trait allows implicit substraction of one point and another.
impl Sub for Point2D {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

/// This trait allows a point to be displayed in the form of `(x,y)`.
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl Point2D {
    /// Returns a Point2D with zeros as default coordinates.
    pub fn new() -> Point2D {
        Point2D {
            ..Default::default()
        }
    }

    /// Returns `true` iff the points' y-coordinate is greater than the one given in the argument.
    pub fn is_above_of(&self, other: &Point2D) -> bool {
        self.y > other.y
    }

    /// Returns `true` iff the points' y-coordinate is smaller than the one given in the argument.
    pub fn is_below_of(&self, other: &Point2D) -> bool {
        self.y < other.y
    }

    /// Returns `true` iff the points' x-coordinate is smaller than the one given in the argument.
    pub fn is_left_of(&self, other: &Point2D) -> bool {
        self.x < other.x
    }

    /// Returns `true` iff the points' x-coordinate is greater than the one given in the argument.
    pub fn is_right_of(&self, other: &Point2D) -> bool {
        self.x > other.x
    }

    /// Returns the euclidean distance to another point.
    pub fn distance_to(&self, other: &Point2D) -> f64 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        (dx * dx + dy * dy).sqrt()
    }

    /// Returns an instance of the same point with rounded coordinates.
    pub fn round(&self, decimal_places: u32) -> Point2D {
        let x = round_to_decimal_places(self.x, decimal_places);
        let y = round_to_decimal_places(self.y, decimal_places);
        Point2D { x, y }
    }
}

#[cfg(test)]
mod test_point2d {
    use super::*;

    #[test]
    fn test_print() {
        let p: Point2D = Point2D { x: 4.0, y: 8.0 };
        assert_eq!("(4,8)", p.to_string());
    }

    #[test]
    fn test_default() {
        let p1: Point2D = Point2D {
            ..Default::default()
        };
        let p2: Point2D = Point2D::new();
        assert_eq!(p1, Point2D { x: 0.0, y: 0.0 });
        assert_eq!(p2, Point2D { x: 0.0, y: 0.0 });
    }

    #[test]
    fn test_order() {
        let p0: Point2D = Point2D { x: 0.0, y: 0.0 };
        let p1: Point2D = Point2D { x: 0.0, y: 1.0 };
        let p3: Point2D = Point2D { x: 1.0, y: 1.0 };
        let p4: Point2D = Point2D { x: 1.0, y: 0.0 };

        assert_eq!(true, p1 > p0);
        assert_eq!(true, p1 == p1);
        assert_eq!(true, p3 > p1);
        assert_eq!(true, p4 > p0);
    }

    #[test]
    fn test_relation() {
        // Above of and Below of
        let p0: Point2D = Point2D { x: 0.0, y: 0.0 };
        let p1: Point2D = Point2D { x: 0.0, y: 1.0 };
        assert!(p0.is_below_of(&p1));
        assert!(p1.is_above_of(&p0));
        assert_eq!(false, p0.is_above_of(&p1));
        assert_eq!(false, p0.is_above_of(&p0));
        assert_eq!(false, p1.is_below_of(&p0));
        assert_eq!(false, p0.is_below_of(&p0));

        // Right of and Left of
        let p0: Point2D = Point2D { x: 0.0, y: 0.0 };
        let p1: Point2D = Point2D { x: 1.0, y: 0.0 };
        assert!(p0.is_left_of(&p1));
        assert!(p1.is_right_of(&p0));
        assert_eq!(false, p0.is_right_of(&p1));
        assert_eq!(false, p0.is_right_of(&p0));
        assert_eq!(false, p1.is_left_of(&p0));
        assert_eq!(false, p0.is_left_of(&p0));
    }
}
