use cg_library::tools2d::{bently_ottmann, read_segments_from_file, save_points};

fn main() {
    let segments = read_segments_from_file("../data/s_1000_10.dat");

    let intersections = bently_ottmann(segments);

    println!("Found Intersections: {}", intersections.len());

    save_points(intersections, "intersection_points.dat");
}
