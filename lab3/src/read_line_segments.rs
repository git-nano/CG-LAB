use std::collections::BTreeSet;
use std::fs;

use crate::LineSegment2D;
use crate::Point2D;

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
