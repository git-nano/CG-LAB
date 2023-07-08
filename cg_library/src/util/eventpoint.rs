use crate::linesegment2d::LineSegment2D;
use crate::point2d::Point2D;
use std::cmp::Ordering;
use std::fmt;
use std::collections::BTreeSet;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EventType {
    IsLeftEndpoint,
    IsRightEndpoint,
    IsIntersection,
}


impl fmt::Display for EventType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EventType::IsLeftEndpoint => write!(f, "LeftEndpoint"),
            EventType::IsRightEndpoint => write!(f, "RightEndpoint"),
            EventType::IsIntersection => write!(f, "Intersection"),
        }
    }
}

#[derive(Clone, Copy)]
pub struct EventPoint {
    pub point: Point2D,
    pub event_type: EventType,
    pub first_line: LineSegment2D,
    pub second_line: Option<LineSegment2D>,
}

impl Eq for EventPoint {}

impl Ord for EventPoint {
    fn cmp(&self, other: &EventPoint) -> Ordering {
        self.point.partial_cmp(&other.point).unwrap()
    }
}

impl PartialOrd for EventPoint {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.point.cmp(&other.point))
    }
}

impl PartialEq for EventPoint {
    fn eq(&self, other: &Self) -> bool {
        self.point == other.point
    }
}


impl fmt::Display for EventPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"e: {}, p: {}, s: {}", self.event_type, self.point, self.first_line)
    }
}

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

    #[test]
    fn test_order(){
        let p1: Point2D = Point2D { x: 0.0, y: 0.0 };
        let p2: Point2D = Point2D { x: 1.0, y: 1.0 };
        let s1: LineSegment2D = LineSegment2D::new(p1, p2);
        let e1:EventPoint = EventPoint { point: s1.p1, event_type: EventType::IsLeftEndpoint, first_line: s1, second_line: None };
        let e2:EventPoint = EventPoint { point: s1.p2, event_type: EventType::IsRightEndpoint, first_line: s1, second_line: None };
        assert_eq!(true, e2 > e1);
    }

    #[test]
    fn test_btree() {
        
        let mut queue: BTreeSet<EventPoint> = BTreeSet::new();

        let p1: Point2D = Point2D { x: 0.0, y: 2.0 };
        let p2: Point2D = Point2D { x: 2.0, y: 0.0 };
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

        queue.extend(vec![e1_1,e1_2,e2_1,e2_2, e12]);
        let mut i = queue.into_iter();
        assert_eq!(EventType::IsLeftEndpoint, i.next().unwrap().event_type);
        assert_eq!(EventType::IsLeftEndpoint, i.next().unwrap().event_type);
        assert_eq!(EventType::IsIntersection, i.next().unwrap().event_type);
        assert_eq!(EventType::IsRightEndpoint, i.next().unwrap().event_type);
        assert_eq!(EventType::IsRightEndpoint, i.next().unwrap().event_type);
    }

    



}

