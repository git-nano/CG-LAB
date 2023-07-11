use cg_library::linesegment2d::LineSegment2D;
use cg_library::point2d::Point2D;
use cg_library::util::eventpoint::{EventPoint, EventType};

use std::collections::{BTreeMap, BTreeSet, HashMap, LinkedList};

use num_traits::Float;
use ordered_float::OrderedFloat;
use std::ops::Bound::{Excluded, Unbounded};

pub struct SweepLine {
    pub event_queue: BTreeSet<EventPoint>,
    segments: BTreeMap<OrderedFloat<f64>, LineSegment2D>,
    current_event: Option<EventPoint>,
    current_x: f64,
    events_order: OrderedFloat<f64>,
    pub intersection_points: Vec<Point2D>,
}

impl SweepLine {
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
                let seg_e = e.first_line;
                let seg_a = self.get_next_neighbor(self.events_order);
                let seg_b = self.get_prev_neighbor(self.events_order);
                self.segments.remove(&self.events_order);

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

    pub fn update_segments(&mut self) {
        let epsilon = if self.current_event.unwrap().event_type != EventType::IsIntersection {
            0.0
        } else {
            1e-8
        };

        let mut temp_map: BTreeMap<OrderedFloat<f64>, LineSegment2D> = BTreeMap::new();

        for (&key, &value) in &self.segments {
            let updated_key = OrderedFloat(value.line.y_from_x(self.current_x + epsilon));
            temp_map.insert(updated_key, value);
        }

        std::mem::swap(&mut self.segments, &mut temp_map);
    }

    pub fn print(&self) {
        println!("\nCurrent x: {}", self.current_x);
        println!("Current event: {}", self.current_event.unwrap());
        println!("Current key: {}", self.events_order);
        for (key, value) in &self.segments {
            println!("( key: {} , slope: {} )", key, value.line.slope);
        }
    }

    pub fn get_next_neighbor(&self, key: OrderedFloat<f64>) -> Option<LineSegment2D> {
        let next = self.segments.range((Excluded(&key), Unbounded)).next();
        if let Some((next_key, next_value)) = next {
            return Some(*next_value);
        } else {
            return None;
        }
    }
    pub fn get_prev_neighbor(&self, key: OrderedFloat<f64>) -> Option<LineSegment2D> {
        let prev = self.segments.range((Unbounded, Excluded(&key))).next_back();
        if let Some((prev_key, prev_value)) = prev {
            return Some(*prev_value);
        } else {
            return None;
        }
    }
}
