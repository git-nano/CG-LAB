use crate::point2d::Point2D;

pub fn ccw(p: &Point2D, q: &Point2D, r: &Point2D) -> f64 {
    return (p.x * q.y - p.y * q.x) + (q.x * r.y - q.y * r.x) + (p.y * r.x - p.x * r.y);
}
