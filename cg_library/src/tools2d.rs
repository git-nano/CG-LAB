use crate::point2d::Point2D;

pub fn ccw(p: &Point2D, q: &Point2D, r: &Point2D) -> f64 {
    return (p.x * q.y - p.y * q.x) + (q.x * r.y - q.y * r.x) + (p.y * r.x - p.x * r.y);
}

/// This function rounds to a given integer of decimal places to filter numerical errors.
pub fn round_to_decimal_places(value: f64, decimal_places: u32) -> f64 {
    let multiplier = 10u64.pow(decimal_places);
    let rounded_value = (value * (multiplier as f64)).round() / (multiplier as f64);
    rounded_value
}
