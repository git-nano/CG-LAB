//! Calculation tools in a 2-Dimensional vector space.
//!
//! Provides tools like file operations and float operations for the computational geometry library [cg_library](crate).

use crate::linesegment2d::LineSegment2D;
use crate::point2d::Point2D;
use crate::util::eventpoint::{EventPoint, EventType};
use crate::util::sweepline::SweepLine;
use std::collections::BTreeSet;
use std::fs;
use std::io::Write;

/// Returns the counter clock wise value for three points.
///
/// This function returns zero if the point r is on the line stretched by the points p and q.
/// It returns a value smaller than zero if the point r follows in a clock wise direction. If it is
/// bigger than zero if the point r follows in a counter clock wise direction.
pub fn ccw(p: &Point2D, q: &Point2D, r: &Point2D) -> f64 {
    return (p.x * q.y - p.y * q.x) + (q.x * r.y - q.y * r.x) + (p.y * r.x - p.x * r.y);
}

/// This function rounds to a given integer of decimal places to filter numerical errors.
pub fn round_to_decimal_places(value: f64, decimal_places: u32) -> f64 {
    let multiplier = 10u64.pow(decimal_places);
    let rounded_value = (value * (multiplier as f64)).round() / (multiplier as f64);
    rounded_value
}

/// This function reads a set of line segments from a file.
///
/// It returns this file in the form of a binary tree set of line segments.
///
/// The file needs to be in the form:
/// x1 y1 x2 y2
/// ...
pub fn read_segments_from_file(path: &str) -> BTreeSet<LineSegment2D> {
    let content = fs::read_to_string(path).unwrap_or_else(|e| panic!("{e}"));
    let mut line_segments: BTreeSet<LineSegment2D> = BTreeSet::new();
    for segment in content.lines() {
        let values: Vec<&str> = segment.split_whitespace().collect();
        let p1 = Point2D {
            x: (values[0].parse::<f64>().unwrap()),
            y: (values[1].parse::<f64>().unwrap()),
        };
        let p2 = Point2D {
            x: (values[2].parse::<f64>().unwrap()),
            y: (values[3].parse::<f64>().unwrap()),
        };
        line_segments.insert(LineSegment2D::new(p1, p2));
    }
    return line_segments;
}

/// This function writes a vector of points into a file.
///
/// The format of the file is:
/// x1 y1
/// x2 y2
/// ...
pub fn save_points(points: Vec<Point2D>, path: &str) {
    let mut file = fs::File::create(path).expect("Failed to create file!");
    for point in points {
        write!(file, "{} {}\n", point.x, point.y).expect("Failed to write to file!");
    }
}

/// This function calculates the intersection points of a set of line segments using the
/// bently ottmann algorithm.
///
/// # Examples
/// ```
/// use cg_library::tools2d::*;
/// let segments = read_segments_from_file("../data/s_1000_10.dat");
/// let intersections = bently_ottmann(segments);
/// ```
pub fn bently_ottmann(segments: BTreeSet<LineSegment2D>) -> Vec<Point2D> {
    let mut sl: SweepLine = SweepLine::new();
    for segment in segments {
        if segment.line.is_vertical() {
            continue;
        }

        sl.event_queue.insert(EventPoint {
            point: segment.p1,
            event_type: EventType::IsLeftEndpoint,
            first_line: segment,
            second_line: None,
        });
        sl.event_queue.insert(EventPoint {
            point: segment.p2,
            event_type: EventType::IsRightEndpoint,
            first_line: segment,
            second_line: None,
        });
    }

    while !sl.event_queue.is_empty() {
        sl.process_next_event();
    }
    sl.intersection_points.sort();

    return sl.intersection_points;
}
