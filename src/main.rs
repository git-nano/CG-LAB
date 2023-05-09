#![allow(dead_code)]

mod point;
mod line;
use line::{LineVec, LineRelation};

use std::time::Instant;
use indicatif::{ProgressBar, ProgressStyle};
use std::env;

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
