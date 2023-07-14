use cg_library::linesegment2d::LineSegment2D;
use cg_library::point2d::Point2D;

mod read_line_segments;
use read_line_segments::read_segments_from_file;

use cg_library::tools2d::{bently_ottmann,save_points};

fn main() {
    let segments = read_segments_from_file("../data/s_1000_10.dat");

    let intersections = bently_ottmann(segments);

    println!("Found Intersections: {}", intersections.len());
    
    save_points(intersections, "intersection_points.dat");
}

