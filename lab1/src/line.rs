use crate::point::{Point,Abs,ccw};

use std::fs;

pub enum LineRelation {
    Intersecting,
    NonIntersecting,
    ColinearOverlap,
    ColinearNonOverlap,
}



pub fn max(f1: &f64, f2: &f64) -> f64 {
    if f1 >= f2 {
        return *f1;
    } else {
        return *f2;
    }
}

pub fn min(f1: &f64, f2: &f64) -> f64 {
    if f1 >= f2 {
        return *f2;
    } else {
        return *f1;
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Line {
    pub start: Point<f64>,
    pub end: Point<f64>,
}

impl Line {
    fn parse_string_vec(values: &Vec<&str>) -> Line {
        return Line {
            start: Point {
                x: (values[0].parse::<f64>().unwrap()),
                y: (values[1].parse::<f64>().unwrap()),
            },
            end: Point {
                x: (values[2].parse::<f64>().unwrap()),
                y: (values[3].parse::<f64>().unwrap()),
            },
        };
    }

    fn colinear_overlap(&self, p: &Point<f64>) -> bool {
        let pivot_index: usize = (self.start - self.end).abs().index_of_significance();
        if p[pivot_index] < min(&self.start[pivot_index], &self.end[pivot_index])
            || p[pivot_index] > max(&self.start[pivot_index], &self.end[pivot_index])
        {
            return false;
        }
        return true;
    }
    pub fn intersect(&self, line: &Line) -> LineRelation {
        let (p1, p2, q1, q2) = (self.start, self.end, line.start, line.end);
        if ccw(&p1, &p2, &q1) == 0.0 && ccw(&p1, &p2, &q2) == 0.0 && &p1 != &p2 {
            if self.colinear_overlap(&q1)
                || self.colinear_overlap(&q2)
                || line.colinear_overlap(&p1)
                || line.colinear_overlap(&p2)
            {
                return LineRelation::ColinearOverlap;
            }
            return LineRelation::ColinearNonOverlap;
        } else if ccw(&q1, &q2, &p1) == 0.0 && ccw(&q1, &q2, &p2) == 0.0 && &q1 != &q2 {
            if self.colinear_overlap(&q1)
                || self.colinear_overlap(&q2)
                || line.colinear_overlap(&p1)
                || line.colinear_overlap(&p2)
            {
                return LineRelation::ColinearOverlap;
            }
            return LineRelation::ColinearNonOverlap;
        } else if ccw(&p1, &p2, &q1) * ccw(&p1, &p2, &q2) <= 0.0
            && ccw(&q1, &q2, &p1) * ccw(&q1, &q2, &p2) <= 0.0
        {
            return LineRelation::Intersecting;
        }
        return LineRelation::NonIntersecting;
    }
}

#[derive(Clone, Debug)]
pub struct LineVec {
    pub rows: usize,
    pub data: Vec<Line>,
}

impl LineVec {
    pub fn new(rows: usize) -> LineVec {
        let data: Vec<Line> = vec![
            Line {
                start: Point { x: 0.0, y: 0.0 },
                end: Point { x: 0.0, y: 0.0 }
            };
            rows
        ];
        return LineVec { rows, data };
    }
    pub fn from_file(path: &str) -> LineVec {
        let content = fs::read_to_string(path).unwrap_or_else(|e| panic!("{e}"));
        let mut line_vec: Vec<Line> = Vec::new();
        for lines in content.lines() {
            let values: Vec<&str> = lines.split_whitespace().collect();
            line_vec.push(Line::parse_string_vec(&values))
        }
        return LineVec {
            rows: (line_vec.len()),
            data: (line_vec),
        };
    }
    pub fn from_string(input: &str) -> LineVec {
        let content: Vec<&str> = input.split(";").collect();
        let mut line_vec: Vec<Line> = Vec::new();
        for lines in content {
            let values: Vec<&str> = lines.split_whitespace().collect();
            line_vec.push(Line::parse_string_vec(&values))
        }
        return LineVec {
            rows: (line_vec.len()),
            data: (line_vec),
        };
    }
}

