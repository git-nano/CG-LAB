#![allow(dead_code)]

use crate::point2d::Point2D;
use std::fmt;

#[derive(Debug,PartialEq, PartialOrd, Clone, Copy)]
pub struct Line2D {
    pub slope: f64,
    pub intercept: f64
}

impl Eq for Line2D {}

impl Line2D {

    pub fn is_vertical(self) -> bool {
        self.slope.is_infinite()
    }
    pub fn is_horizontal(self) -> bool {
        self.slope == 0.0
    }

    pub fn from_slope_and_point (slope: f64, p: Point2D) -> Line2D {
        let mut intercept = 0.0;
        if slope.is_infinite() {
            intercept = p.x;
        } else {
            intercept = p.y - slope * p.x;
        }
        return Line2D {
            slope,
            intercept
        };
    }
    pub fn from_point_and_point (p1: Point2D, p2: Point2D) -> Line2D {
        let mut slope = 0.0;
        let mut intercept = 0.0;
        if p1.x == p2.x {
            slope = std::f64::INFINITY;
            intercept = p1.x;
        } else {
            slope = (p2.y - p1.y) / (p2.x - p1.x);
            intercept = p2.y - slope * p2.x;
        }
        return Line2D {
            slope,
            intercept
        };
    }

    pub fn y_from_x(self, x:f64) -> f64 {
        if self.is_vertical() {
            return self.intercept;
        }
        return self.slope * x + self.intercept;

    }

    pub fn contains (self, p: &Point2D) -> bool {
        if self.is_vertical() {
            return self.intercept == p.x;
        } else {
            return self.slope * p.x + self.intercept == p.y;
        }
    }

    pub fn is_parallel_to (self, other: &Line2D) -> bool {
        self.slope == other.slope
    }

    pub fn intersection(self, other: &Line2D) -> Option<Point2D> {
        if self.is_parallel_to(other) {
            return None;
        }

        let (x, y) = if self.is_vertical() {
            (self.intercept, other.slope * self.intercept + other.intercept)
        } else if other.is_vertical() {
            (other.intercept, self.slope * other.intercept + self.intercept)
        } else {
            let x = (self.intercept - other.intercept) / (other.slope - self.slope);
            let y = self.slope * x + self.intercept;
            (x, y)
        };

        Some(Point2D { x, y })
    }
}

impl fmt::Display for Line2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_vertical() {
            write!(f, "x = {:+}", self.intercept)
        } else {
            if self.is_horizontal() {
                write!(f, "f(x) -> {:+}", self.intercept)
            } else {
            write!(f,"f(x) -> {:+} * x {:+}", self.slope, self.intercept)
            }
        }
    }
}

#[cfg(test)]
mod test_line2d {
    use super::*;

    #[test]
    fn test_vertical() {
        let l1 = Line2D { slope: std::f64::INFINITY, intercept: 10.0};
        assert!(l1.is_vertical());
    }

    #[test]
    fn test_display() {
        let l1 = Line2D { slope: std::f64::INFINITY, intercept: 10.0};
        let l2 = Line2D { slope: -12.0, intercept: -4.0};
        let l3 = Line2D { slope: 0.0, intercept: -4.0};
        assert_eq!("x = +10", l1.to_string());
        assert_eq!("f(x) -> -12 * x -4", l2.to_string());
        assert_eq!("f(x) -> -4", l3.to_string());
    }

    #[test]
    fn test_from(){
        let l1: Line2D = Line2D::from_slope_and_point(1.0, Point2D { x: 1.0, y: 1.0 });
        let l2: Line2D = Line2D::from_point_and_point(Point2D { x: 0.0, y: 2.0 }, Point2D { x: 2.0, y: 0.0 });
        assert_eq!("f(x) -> +1 * x +0", l1.to_string());
        assert_eq!("f(x) -> -1 * x +2", l2.to_string());
    }

    #[test]
    fn test_intersection() {
        let l1: Line2D = Line2D { slope: std::f64::INFINITY, intercept: 1.0 };
        let l2: Line2D = Line2D { slope: std::f64::INFINITY, intercept: 2.0 };
        let l3: Line2D = Line2D { slope: 4.0, intercept: 1.0 };
        assert_eq!(None, l1.intersection(&l2));
        assert_eq!(Some(Point2D { x: 1.0, y: 5.0 }), l1.intersection(&l3));
        assert_eq!(Some(Point2D { x: 1.0, y: 5.0 }), l3.intersection(&l1));

    }


}
