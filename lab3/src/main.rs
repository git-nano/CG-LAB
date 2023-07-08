#![allow(dead_code, unused, unused_variables, unused_assignments)]

use cg_library::util::eventpoint::{EventPoint,EventType};
use cg_library::point2d::Point2D;
use cg_library::linesegment2d::LineSegment2D;

mod read_line_segments;
use read_line_segments::read_segments_from_file;

mod sweepline;
use sweepline::SweepLine;


fn main() {
    
    let mut intersections: Vec<EventPoint> = Vec::new(); 
    let mut sl: SweepLine = SweepLine::new();

    let segments = read_segments_from_file("s_1000_10.dat");
    
    for segment in segments {
        if segment.line.is_vertical() {
            continue;
        }

        sl.event_queue.insert(EventPoint { point: segment.p1, event_type: EventType::IsLeftEndpoint, first_line: segment, second_line: None });
        sl.event_queue.insert(EventPoint { point: segment.p2, event_type: EventType::IsRightEndpoint, first_line: segment, second_line: None });
    }

    while !sl.event_queue.is_empty() {
        sl.process_next_event();
    }
    
    sl.intersection_points.sort();

    for i in sl.intersection_points.iter() {
        // println!("{:.5}",i);
    }
    println!("Found Intersections: {}", sl.intersection_points.len());

}










#[cfg(test)]
mod test_sweep_line {
    use super::*;

    //#[test]
    fn test_update() {

        let mut sl: SweepLine = SweepLine::new();

        let p1: Point2D = Point2D { x: -1.0, y: 3.0 };
        let p2: Point2D = Point2D { x: 3.0, y: -1.0 };
        let s1: LineSegment2D = LineSegment2D::new(p1, p2);
        let e1_1:EventPoint = EventPoint { point: s1.p1, event_type: EventType::IsLeftEndpoint, first_line: s1, second_line: None };
        let e1_2:EventPoint = EventPoint { point: s1.p2, event_type: EventType::IsRightEndpoint, first_line: s1, second_line: None };

        let p1: Point2D = Point2D { x: 0.0, y: 0.0 };
        let p2: Point2D = Point2D { x: 2.0, y: 2.0 };
        let s2: LineSegment2D = LineSegment2D::new(p1, p2);
        let e2_1:EventPoint = EventPoint { point: s2.p1, event_type: EventType::IsLeftEndpoint, first_line: s2, second_line: None };
        let e2_2:EventPoint = EventPoint { point: s2.p2, event_type: EventType::IsRightEndpoint, first_line: s2, second_line: None };

        let p1: Point2D = Point2D { x: 1.0, y: 1.0 };
        let e12:EventPoint = EventPoint { point: p1, event_type: EventType::IsIntersection, first_line: s1, second_line: Some(s2) };

        sl.event_queue.extend(vec![e1_1,e1_2,e2_1,e2_2, e12]);
        while !sl.event_queue.is_empty() {
            sl.process_next_event();
        }    

    }

    #[test]
    fn test_neighbors () {
        let mut sl: SweepLine = SweepLine::new();

        let p1: Point2D = Point2D { x: -1.0, y: 3.0 };
        let p2: Point2D = Point2D { x: 3.0, y: -1.0 };
        let s1: LineSegment2D = LineSegment2D::new(p1, p2);
        let e1_1:EventPoint = EventPoint { point: s1.p1, event_type: EventType::IsLeftEndpoint, first_line: s1, second_line: None };
        let e1_2:EventPoint = EventPoint { point: s1.p2, event_type: EventType::IsRightEndpoint, first_line: s1, second_line: None };

        let p1: Point2D = Point2D { x: 0.0, y: 0.0 };
        let p2: Point2D = Point2D { x: 2.0, y: 2.0 };
        let s2: LineSegment2D = LineSegment2D::new(p1, p2);
        let e2_1:EventPoint = EventPoint { point: s2.p1, event_type: EventType::IsLeftEndpoint, first_line: s2, second_line: None };
        let e2_2:EventPoint = EventPoint { point: s2.p2, event_type: EventType::IsRightEndpoint, first_line: s2, second_line: None };

        let p1: Point2D = Point2D { x: -1.5, y: 4.0 };
        let p2: Point2D = Point2D { x: 4.0, y: 5.0 };
        let s3: LineSegment2D = LineSegment2D::new(p1, p2);
        let e3_1:EventPoint = EventPoint { point: s3.p1, event_type: EventType::IsLeftEndpoint, first_line: s3, second_line: None };
        let e3_2:EventPoint = EventPoint { point: s3.p2, event_type: EventType::IsRightEndpoint, first_line: s3, second_line: None };

        let p1: Point2D = Point2D { x: 1.0, y: 1.0 };
        let e12:EventPoint = EventPoint { point: p1, event_type: EventType::IsIntersection, first_line: s1, second_line: Some(s2) };

        sl.event_queue.extend(vec![e1_1,e1_2,e2_1,e2_2, e12, e3_1, e3_2]);
        while !sl.event_queue.is_empty() {
            sl.process_next_event();
        }    
    }




}

