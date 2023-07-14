//! This is the y-structure for the bently ottmann algorithm.
//!
//! This the line that sweeps from left to right above all the event points.

use crate::linesegment2d::LineSegment2D;
use crate::point2d::Point2D;
use crate::util::eventpoint::{EventPoint, EventType};

use std::collections::{BTreeMap, BTreeSet};

use ordered_float::OrderedFloat;
use std::ops::Bound::{Excluded, Unbounded};

/// This is the heart of the bently ottmann algorithm, it contains all the elements important like
/// event queue, sweep line and intersection points.
pub struct SweepLine {
    /// The event queue of the algorithm, it contains all the event points sorted in a binary
    /// tree.
    ///
    /// From this event queue are points popped from the front ant inserted in the correct order.
    /// Points are ordered with the event's point coordinate.
    pub event_queue: BTreeSet<EventPoint>,

    /// The y-structure representation of the segments with the `key` being the y-coordinate of the
    /// associated line that gets updated at every event.
    segments: BTreeMap<OrderedFloat<f64>, LineSegment2D>,

    /// The current event that is handled at the moment.
    current_event: Option<EventPoint>,

    /// That is the current x-coordinate that the sweepline is at.
    current_x: f64,

    /// This is the current y-coordinate of the event's current y-coordinate.
    events_order: OrderedFloat<f64>,

    /// This is the vector of all intersecting points.
    pub intersection_points: Vec<Point2D>,
}

impl SweepLine {
    /// Returns a zero initialized `SweepLine` instance.
    pub fn new() -> SweepLine {
        return SweepLine {
            event_queue: BTreeSet::new(),
            segments: BTreeMap::new(),
            current_event: None,
            events_order: OrderedFloat(0.0),
            current_x: 0.0,
            intersection_points: Vec::new(),
        };
    }
    /// This pops a new event point from the event queue and handles it.
    ///
    /// 1. Pops event form queue.
    /// 2. Update segments in the y-structure with event's x-coordinate.
    /// 3. Handle event type:
    ///     - LeftEndpoint
    ///     - RightEndpoint
    ///     - Intersection
    ///
    pub fn process_next_event(&mut self) {
        let e: EventPoint = self.event_queue.pop_first().unwrap();

        self.current_event = Some(e);
        self.current_x = e.point.x;
        self.events_order = OrderedFloat(e.first_line.line.y_from_x(self.current_x));
        self.update_segments();

        match e.event_type {
            EventType::IsLeftEndpoint => {
                let seg_e = e.first_line;
                self.segments.insert(self.events_order, seg_e);
                let seg_a = self.get_next_neighbor(self.events_order);
                let seg_b = self.get_prev_neighbor(self.events_order);

                // If the segment above exists and intersects the events segment
                if let Some(seg_a) = seg_a {
                    if let Some(intersection) = seg_a.intersects(&seg_e) {
                        if intersection.x > self.current_x {
                            self.event_queue.insert(EventPoint {
                                point: intersection.round(9),
                                event_type: EventType::IsIntersection,
                                first_line: seg_e,
                                second_line: Some(seg_a),
                            });
                        }
                    }
                }

                // If the segment below exists and intersects the events segment
                if let Some(seg_b) = seg_b {
                    if let Some(intersection) = seg_b.intersects(&seg_e) {
                        if intersection.x > self.current_x {
                            self.event_queue.insert(EventPoint {
                                point: intersection.round(9),
                                event_type: EventType::IsIntersection,
                                first_line: seg_e,
                                second_line: Some(seg_b),
                            });
                        }
                    }
                }
            }
            EventType::IsRightEndpoint => {
                let seg_a = self.get_next_neighbor(self.events_order);
                let seg_b = self.get_prev_neighbor(self.events_order);
                self.segments.remove(&self.events_order);

                // If the segment above and below both exist and intersects
                if let (Some(seg_a), Some(seg_b)) = (seg_a, seg_b) {
                    if let Some(intersection) = seg_a.intersects(&seg_b) {
                        if intersection.x > self.current_x {
                            self.event_queue.insert(EventPoint {
                                point: intersection.round(9),
                                event_type: EventType::IsIntersection,
                                first_line: seg_a,
                                second_line: Some(seg_b),
                            });
                        }
                    }
                }
            }
            EventType::IsIntersection => {
                // println!("Intersection at {} of {} and {}", e.point, e.first_line, e.second_line.unwrap());
                self.intersection_points.push(e.point);
                let mut seg_e1 = e.first_line;
                let mut seg_e2 = e.second_line.unwrap();
                if seg_e2 > seg_e1 {
                    (seg_e1, seg_e2) = (seg_e2, seg_e1);
                } // seg_e1 is now above seg_e2
                let order_e1 = OrderedFloat(seg_e1.line.y_from_x(self.current_x + 1e-8));
                let order_e2 = OrderedFloat(seg_e2.line.y_from_x(self.current_x + 1e-8));
                let seg_a = self.get_prev_neighbor(order_e2);
                let seg_b = self.get_next_neighbor(order_e1);

                // If the upper segment now has a next neighbor and intersects it
                if let Some(seg_a) = seg_a {
                    if let Some(intersection) = seg_a.intersects(&seg_e2) {
                        if intersection.x > self.current_x {
                            self.event_queue.insert(EventPoint {
                                point: intersection.round(9),
                                event_type: EventType::IsIntersection,
                                first_line: seg_e2,
                                second_line: Some(seg_a),
                            });
                        }
                    }
                }

                // If the lower segment now has a previous neighbor and intersects it
                if let Some(seg_b) = seg_b {
                    if let Some(intersection) = seg_b.intersects(&seg_e1) {
                        if intersection.x > self.current_x {
                            self.event_queue.insert(EventPoint {
                                point: intersection.round(9),
                                event_type: EventType::IsIntersection,
                                first_line: seg_b,
                                second_line: Some(seg_e1),
                            });
                        }
                    }
                }
            }
        }
    }

    /// This rearranges all line segments in the `segments` map.
    ///
    /// The rearranging works by calculating every y-coordinate of each line segment with the
    /// current x-coordinate of the sweep line. In the case of an intersection, where two lines
    /// would have the same y-coordinate, a small epsilon value is added to retrieve the position
    /// after the intersection x-coordinate.
    pub fn update_segments(&mut self) {
        let epsilon = if self.current_event.unwrap().event_type != EventType::IsIntersection {
            0.0
        } else {
            1e-8
        };

        let mut temp_map: BTreeMap<OrderedFloat<f64>, LineSegment2D> = BTreeMap::new();

        for (&_key, &value) in &self.segments {
            let updated_key = OrderedFloat(value.line.y_from_x(self.current_x + epsilon));
            temp_map.insert(updated_key, value);
        }

        std::mem::swap(&mut self.segments, &mut temp_map);
    }

    /// This enables a print of the current state of the sweep line segments.
    pub fn print(&self) {
        println!("\nCurrent x: {}", self.current_x);
        println!("Current event: {}", self.current_event.unwrap());
        println!("Current key: {}", self.events_order);
        for (key, value) in &self.segments {
            println!("( key: {} , slope: {} )", key, value.line.slope);
        }
    }

    /// Returns the next segment neighbor of a given key value in the y-structure.
    pub fn get_next_neighbor(&self, key: OrderedFloat<f64>) -> Option<LineSegment2D> {
        let next = self.segments.range((Excluded(&key), Unbounded)).next();
        if let Some((_next_key, next_value)) = next {
            return Some(*next_value);
        } else {
            return None;
        }
    }

    /// Returns the previous segment neighbor of a given key value in the y-structure.
    pub fn get_prev_neighbor(&self, key: OrderedFloat<f64>) -> Option<LineSegment2D> {
        let prev = self.segments.range((Unbounded, Excluded(&key))).next_back();
        if let Some((_prev_key, prev_value)) = prev {
            return Some(*prev_value);
        } else {
            return None;
        }
    }
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
        let e1_1: EventPoint = EventPoint {
            point: s1.p1,
            event_type: EventType::IsLeftEndpoint,
            first_line: s1,
            second_line: None,
        };
        let e1_2: EventPoint = EventPoint {
            point: s1.p2,
            event_type: EventType::IsRightEndpoint,
            first_line: s1,
            second_line: None,
        };

        let p1: Point2D = Point2D { x: 0.0, y: 0.0 };
        let p2: Point2D = Point2D { x: 2.0, y: 2.0 };
        let s2: LineSegment2D = LineSegment2D::new(p1, p2);
        let e2_1: EventPoint = EventPoint {
            point: s2.p1,
            event_type: EventType::IsLeftEndpoint,
            first_line: s2,
            second_line: None,
        };
        let e2_2: EventPoint = EventPoint {
            point: s2.p2,
            event_type: EventType::IsRightEndpoint,
            first_line: s2,
            second_line: None,
        };

        let p1: Point2D = Point2D { x: 1.0, y: 1.0 };
        let e12: EventPoint = EventPoint {
            point: p1,
            event_type: EventType::IsIntersection,
            first_line: s1,
            second_line: Some(s2),
        };

        sl.event_queue.extend(vec![e1_1, e1_2, e2_1, e2_2, e12]);
        while !sl.event_queue.is_empty() {
            sl.process_next_event();
        }
    }

    #[test]
    fn test_neighbors() {
        let mut sl: SweepLine = SweepLine::new();

        let p1: Point2D = Point2D { x: -1.0, y: 3.0 };
        let p2: Point2D = Point2D { x: 3.0, y: -1.0 };
        let s1: LineSegment2D = LineSegment2D::new(p1, p2);
        let e1_1: EventPoint = EventPoint {
            point: s1.p1,
            event_type: EventType::IsLeftEndpoint,
            first_line: s1,
            second_line: None,
        };
        let e1_2: EventPoint = EventPoint {
            point: s1.p2,
            event_type: EventType::IsRightEndpoint,
            first_line: s1,
            second_line: None,
        };

        let p1: Point2D = Point2D { x: 0.0, y: 0.0 };
        let p2: Point2D = Point2D { x: 2.0, y: 2.0 };
        let s2: LineSegment2D = LineSegment2D::new(p1, p2);
        let e2_1: EventPoint = EventPoint {
            point: s2.p1,
            event_type: EventType::IsLeftEndpoint,
            first_line: s2,
            second_line: None,
        };
        let e2_2: EventPoint = EventPoint {
            point: s2.p2,
            event_type: EventType::IsRightEndpoint,
            first_line: s2,
            second_line: None,
        };

        let p1: Point2D = Point2D { x: -1.5, y: 4.0 };
        let p2: Point2D = Point2D { x: 4.0, y: 5.0 };
        let s3: LineSegment2D = LineSegment2D::new(p1, p2);
        let e3_1: EventPoint = EventPoint {
            point: s3.p1,
            event_type: EventType::IsLeftEndpoint,
            first_line: s3,
            second_line: None,
        };
        let e3_2: EventPoint = EventPoint {
            point: s3.p2,
            event_type: EventType::IsRightEndpoint,
            first_line: s3,
            second_line: None,
        };

        let p1: Point2D = Point2D { x: 1.0, y: 1.0 };
        let e12: EventPoint = EventPoint {
            point: p1,
            event_type: EventType::IsIntersection,
            first_line: s1,
            second_line: Some(s2),
        };

        sl.event_queue
            .extend(vec![e1_1, e1_2, e2_1, e2_2, e12, e3_1, e3_2]);
        while !sl.event_queue.is_empty() {
            sl.process_next_event();
        }
    }
}
