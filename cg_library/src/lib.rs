//! Library for calculations in the field of computational geometry.
//!
//! Provides datatypes and tools for calculating in the field of geometry. The peak of this
//! library is the implementation of the `sweep-line` or [bently
//! ottmann](tools2d::bently_ottmann) algorithm to calculate a set of intersection points of
//! $N$ line segments. The underlying structure provides following datatypes:
//! - [Point2D](point2d::Point2D)
//! - [Line2D](line2d::Line2D)
//! - [LineSegment2D](linesegment2d::LineSegment2D)
//! - [Polygon2D](polygon2d::Polygon2D)
//! - [EventPoint](util::eventpoint::EventPoint)
//! - [SweepLine](util::sweepline::SweepLine)
//!
//! The library was created in order to fulfill all requirements for the course `computational
//! geometry` in the first master semester.

#![allow(dead_code)]

pub mod line2d;
pub mod linesegment2d;
pub mod point2d;
pub mod polygon2d;
pub mod tools2d;
pub mod util {
    //! This section provides more advanced datatypes.
    pub mod eventpoint;
    pub mod sweepline;
}
