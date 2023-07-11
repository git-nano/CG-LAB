#![allow(dead_code)]

mod line;
mod point;
use line::{LineRelation, LineVec};

use indicatif::{ProgressBar, ProgressStyle};
use std::env;
use std::time::Instant;

/*

   This code calculates whether lines defined in a file intersect each other or not.
   The set of lines can be read with a string or a file.
   (p1 and p2 are the line start and end of the first line, q1 and q2 of the second)

   Usage:
   Use by giving the filename as a commandline argument in cargo run, use release to compile for a faster mode.
   `cargo run --release -- <filename>`

   Result:
   The Result is devided into four Enumerators:
   - Intersecting: This means exactly one point lies on the another line, and they are not colinear
   - NonIntersecting: This means no point lies on the another line, and they are not colinear
   - ColinearOverlap: This means the lines are colinear and at least one point lies on the other line
   - ColinearNonOverlap: This means the lines are colinear and no point lies on the other line

   Calculation:
   For the calculation we use the counter clock wise method, this gives following condition tree:
   if ccw(p1,p2,q1) == ccw(p1,p2,q2) == 0 && p1 != p2:
       definitely colinear! (check if overlap)
   if ccw(q1,q2,p1) == ccw(q1,q2,p2) == 0 && q1 != q2:
       definitely colinear! (check if overlap)
   else if ccw(p1, p2, q1) * ccw(p1, p2, q2) <= 0.0 && ccw(q1, q2, p1) * ccw(q1, q2, p2) <= 0.0:
       definitely intersecting
   else:
       definitely not intersecting

   Note: We have to check if p1 != p2, since a line segment containing 2 equal points is still considered
   as a line segment, the ccw of such a line will be 0 even though, it is not considered colinear.

   Methhods:
   There are 3-Structs used:
   - Point: This contains x and y and basic functions to add, sub, mul...
   - Line: This uses two Points to indicate start and end of line segment
           it implements functionality to find intersections as well as find wether a point lies on a line
   - LineVec: This uses a vector of lines, it implements functionality to read from a file or a string in the format:
           File: "x01 y01 x02 y02\nx11 y11 x12 y12\n..."
           Line: "x01 y01 x02 y02;x11 y11 x12 y12;..."

   Overlap:
   The method used for overlap does a pivot search:
   1. The line is examined at its dimension of most variance, then each point of a line segment, that fulfills
   the condition is `colinear` is put to test whether is element in the range of the minimal and maximal pivot dimension.

   Main:
   The main reads a file via commandline and counts for a set of linesegments all intersections and colinear_overlaps
   The results were compared with different approaches of other groups.

*/

fn main() {
    let args: Vec<String> = env::args().collect();
    let path: &str;
    if args.len() == 1 {
        path = "s_10000_1.dat";
    } else {
        path = &args[1];
    }
    let line_vec = LineVec::from_file(path);

    println!("The progress speed will increase over time!");

    let bar = ProgressBar::new(line_vec.rows as u64);
    bar.set_style(ProgressStyle::with_template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7}[{percent}%] {msg} Estimated: {eta}")
        .unwrap()
        .progress_chars("##-"));

    let mut intersections = 0;
    let mut colinear_overlaps = 0;

    let now = Instant::now();

    for (index1, line1) in line_vec.data.iter().enumerate() {
        for line2 in line_vec.data.iter().skip(index1 + 1) {
            match line1.intersect(line2) {
                LineRelation::Intersecting => intersections += 1,
                LineRelation::NonIntersecting => (),
                LineRelation::ColinearOverlap => colinear_overlaps += 1,
                LineRelation::ColinearNonOverlap => (),
            }
        }
        bar.inc(1);
    }
    bar.finish();
    println!(
        "\nIntersecting lines: {}\nColinear & overlapping lines: {}\nDone in: {}ms",
        intersections,
        colinear_overlaps,
        now.elapsed().as_millis()
    );
}

#[cfg(test)]
mod test;
