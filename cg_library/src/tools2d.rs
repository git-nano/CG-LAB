use crate::point2d::Point2D;
use crate::linesegment2d::LineSegment2D;
use crate::util::eventpoint::{EventPoint, EventType};
use crate::util::sweepline::SweepLine;
use std::collections::BTreeSet;
use std::fs;
use std::io::Write;

pub fn ccw(p: &Point2D, q: &Point2D, r: &Point2D) -> f64 {
    return (p.x * q.y - p.y * q.x) + (q.x * r.y - q.y * r.x) + (p.y * r.x - p.x * r.y);
}

/// This function rounds to a given integer of decimal places to filter numerical errors.
pub fn round_to_decimal_places(value: f64, decimal_places: u32) -> f64 {
    let multiplier = 10u64.pow(decimal_places);
    let rounded_value = (value * (multiplier as f64)).round() / (multiplier as f64);
    rounded_value
}


pub fn save_points(points: Vec<Point2D>, path: &str){
    let mut file = fs::File::create(path).expect("Failed to create file!");
    for point in points {
        write!(file, "{} {}\n", point.x, point.y).expect("Failed to write to file!");
    }
}

pub fn bently_ottmann (segments: BTreeSet<LineSegment2D>) -> Vec<Point2D> {
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

    while !sl.event_queue.is_empty(){
        sl.process_next_event();
    }
    sl.intersection_points.sort();
    
    return sl.intersection_points;

}
