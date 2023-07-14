//! This is a event point for an event queue.
//!
//! Especially usefull for the event queue in the [bently
//! ottmann](crate::tools2d::bently_ottmann) algorithm where it acts as the x-structure.

use crate::linesegment2d::LineSegment2D;
use crate::point2d::Point2D;
use std::cmp::Ordering;
use std::fmt;

/// An event in the bently ottmann algorithm can have one of three types.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EventType {
    /// In case of a point being the left endpoint of a segment.
    IsLeftEndpoint,

    /// In case of a point being the right endpoint of a segment.
    IsRightEndpoint,

    /// In case of a point being the intersection point of two or more segments.
    IsIntersection,
}

/// This trait allows a EventType to be displayed in text form.
impl fmt::Display for EventType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EventType::IsLeftEndpoint => write!(f, "LeftEndpoint"),
            EventType::IsRightEndpoint => write!(f, "RightEndpoint"),
            EventType::IsIntersection => write!(f, "Intersection"),
        }
    }
}

/// An event is handled by an event queue. They need to be sorted and they have certain contents.
#[derive(Clone, Copy)]
pub struct EventPoint {
    /// The point associated with the event.
    pub point: Point2D,

    /// This is the type of the event.
    pub event_type: EventType,

    /// This is the segment of the point associated with an event.
    pub first_line: LineSegment2D,

    /// In case of an intersection event is this the second line associated with the event.
    ///
    /// In the future this could be replaced by a vector, if one intersection allows more than two
    /// lines being part of.
    pub second_line: Option<LineSegment2D>,
}

/// This trait needs to be implemented to satisfy PartialOrd, it is not yet used.
impl Eq for EventPoint {}

/// This trait is added to allow events to be ordered.
///
/// Events are ordered after rising x-coordinates at first and at tie after the y-coordinates of
/// the associated point.
impl Ord for EventPoint {
    fn cmp(&self, other: &EventPoint) -> Ordering {
        self.point.partial_cmp(&other.point).unwrap()
    }
}

/// This trait is added to allow events to be ordered.
///
/// Events are ordered after rising x-coordinates at first and at tie after the y-coordinates of
/// the associated point.
impl PartialOrd for EventPoint {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.point.cmp(&other.point))
    }
}

/// This trait allows the comparison of two points, it returns true if two points match.
impl PartialEq for EventPoint {
    fn eq(&self, other: &Self) -> bool {
        self.point == other.point
    }
}

/// This trait allows the display of a event in the form of `e: {event_type}, p: {point}, s:
/// {segment}`
impl fmt::Display for EventPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "e: {}, p: {}, s: {}",
            self.event_type, self.point, self.first_line
        )
    }
}

/// This trait allows the display in debug mode with all fields in a differnet way, so that for
/// debugs of large event queues this will conclude in a clear representation of the events.
impl fmt::Debug for EventPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("EventPoint")
            .field("Point: ", &self.point)
            .field("Line: ", &self.first_line)
            .field("Event: ", &self.event_type)
            .field("LineB: ", &self.second_line)
            .finish()
    }
}

#[cfg(test)]
mod test_eventpoints {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn test_order() {
        let p1: Point2D = Point2D { x: 0.0, y: 0.0 };
        let p2: Point2D = Point2D { x: 1.0, y: 1.0 };
        let s1: LineSegment2D = LineSegment2D::new(p1, p2);
        let e1: EventPoint = EventPoint {
            point: s1.p1,
            event_type: EventType::IsLeftEndpoint,
            first_line: s1,
            second_line: None,
        };
        let e2: EventPoint = EventPoint {
            point: s1.p2,
            event_type: EventType::IsRightEndpoint,
            first_line: s1,
            second_line: None,
        };
        assert_eq!(true, e2 > e1);
    }

    #[test]
    fn test_btree() {
        let mut queue: BTreeSet<EventPoint> = BTreeSet::new();

        let p1: Point2D = Point2D { x: 0.0, y: 2.0 };
        let p2: Point2D = Point2D { x: 2.0, y: 0.0 };
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

        queue.extend(vec![e1_1, e1_2, e2_1, e2_2, e12]);
        let mut i = queue.into_iter();
        assert_eq!(EventType::IsLeftEndpoint, i.next().unwrap().event_type);
        assert_eq!(EventType::IsLeftEndpoint, i.next().unwrap().event_type);
        assert_eq!(EventType::IsIntersection, i.next().unwrap().event_type);
        assert_eq!(EventType::IsRightEndpoint, i.next().unwrap().event_type);
        assert_eq!(EventType::IsRightEndpoint, i.next().unwrap().event_type);
    }
}
