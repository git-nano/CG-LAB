#![allow(dead_code)]

use std::cmp::Ordering;
use std::ops::{Add,Sub};
use std::fmt;

fn round_to_decimal_places(value: f64, decimal_places: u32) -> f64 {
    let multiplier = 10u64.pow(decimal_places);
    let rounded_value = (value * (multiplier as f64)).round() / (multiplier as f64);
    rounded_value
}

#[derive(Default, Debug,PartialEq,PartialOrd, Clone, Copy)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

impl Eq for Point2D {}

impl Ord for Point2D {
    fn cmp(&self, other: &Point2D) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Add for Point2D {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point2D {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})",self.x, self.y)
    }
}

impl Point2D {
    pub fn new() -> Point2D {
        Point2D {..Default::default()}
    }

    pub fn is_above_of(&self, other: &Point2D) -> bool {
        self.y > other.y
    }
    pub fn is_below_of(&self, other: &Point2D) -> bool {
        self.y < other.y
    }
    pub fn is_left_of(&self, other: &Point2D) -> bool {
        self.x < other.x
    }
    pub fn is_right_of(&self, other: &Point2D) -> bool {
        self.x > other.x
    }
    pub fn distance_to(&self, other: &Point2D) -> f64 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        (dx * dx + dy * dy).sqrt()
    }

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
        let p: Point2D = Point2D {x: 4.0, y: 8.0};
        assert_eq!("(4,8)",p.to_string());
    }

    #[test]
    fn test_default () {
        let p1: Point2D = Point2D{..Default::default()};
        let p2: Point2D = Point2D::new();
        assert_eq!(p1, Point2D{x:0.0, y:0.0});
        assert_eq!(p2, Point2D{x:0.0, y:0.0});
    }

    #[test]
    fn test_order () {
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
    fn test_relation () {

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
