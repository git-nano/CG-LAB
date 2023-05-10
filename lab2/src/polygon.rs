use std::fs;
use svg::node::element::path::{Command, Data, Position};
use svg::node::element::tag;
use svg::parser::Event;

use crate::point::{ccw, Point};

#[derive(Clone, Debug)]
pub struct Polygon {
    pub length: usize,
    pub data: Vec<Point<f64>>,
    pub closed: bool,
}

#[derive(Clone, Debug)]
pub struct PolygonVec {
    pub length: usize,
    pub data: Vec<(String, Polygon)>,
}

impl Polygon {
    pub fn new(length: usize) -> Polygon {
        let data: Vec<Point<f64>> = vec![Point { x: 0.0, y: 0.0 }; length];
        return Polygon {
            length,
            data,
            closed: true,
        };
    }
    pub fn from_file(path: &str) -> Polygon {
        let content = fs::read_to_string(path).unwrap_or_else(|e| panic!("{e}"));
        let mut point_vec: Vec<Point<f64>> = Vec::new();
        let mut closed = false;
        for points in content.lines() {
            let values: Vec<&str> = points.split_whitespace().collect();
            point_vec.push(Point {
                x: values[0].parse::<f64>().unwrap(),
                y: values[1].parse::<f64>().unwrap(),
            });
        }
        if point_vec.last() == point_vec.first() {
            closed = true;
        }
        return Polygon {
            length: (point_vec.len()),
            data: (point_vec),
            closed: (closed),
        };
    }
    pub fn from_string(input: &str) -> Polygon {
        let content: Vec<&str> = input.split(";").collect();
        let mut point_vec: Vec<Point<f64>> = Vec::new();
        let mut closed = false;
        for points in content {
            let values: Vec<&str> = points.split_whitespace().collect();
            point_vec.push(Point {
                x: values[0].parse::<f64>().unwrap(),
                y: values[1].parse::<f64>().unwrap(),
            })
        }
        if point_vec.last() == point_vec.first() {
            closed = true;
        }
        return Polygon {
            length: (point_vec.len()),
            data: (point_vec),
            closed: (closed),
        };
    }
    pub fn calculate_area(&self) -> f64 {
        let mut area = 0.0;
        for i in 0..(self.length - 1) {
            area += 0.5 * ccw(&Point { x: 0.0, y: 0.0 }, &self.data[i], &self.data[i + 1]);
        }
        return area;
    }
}

impl PolygonVec {
    pub fn from_svg(path: &str) -> PolygonVec {
        let mut content = String::new();
        let mut polygon_vec: Vec<(String, Polygon)> = Vec::new();
        let mut start_of_poly_group: bool = false;
        for event in svg::open(path, &mut content).expect("Could not open SVG file!") {
            let mut poly: Vec<Point<f64>> = Vec::new();
            let mut poly_id = String::new();
            match event {
                Event::Tag(tag::Path, _, attributes) => {
                    match attributes.get("id") {
                        Some(data) => {
                            if start_of_poly_group {
                                poly_id = data.to_string();
                            }
                        }
                        None => {}
                    };
                    match attributes.get("d") {
                        Some(data) => {
                            let parsed_data = Data::parse(data).expect("Could not parse Data!");
                            for command in parsed_data.iter() {
                                match command {
                                    Command::Move(rel_or_abs, parameters) => poly.push(Point {
                                        x: parameters[0].clone() as f64,
                                        y: parameters[1].clone() as f64,
                                    }),
                                    Command::Line(rel_or_abs, parameters) => match rel_or_abs {
                                        Position::Relative => poly.push(Point {
                                            x: poly
                                                .last()
                                                .expect("Can only do Relative on Absoulte")
                                                .x
                                                .clone()
                                                + parameters[0].clone() as f64,
                                            y: poly
                                                .last()
                                                .expect("Can only do Relative on Absoulte")
                                                .y
                                                .clone()
                                                + parameters[1].clone() as f64,
                                        }),
                                        Position::Absolute => poly.push(Point {
                                            x: parameters[0].clone() as f64,
                                            y: parameters[1].clone() as f64,
                                        }),
                                    },
                                    Command::Close => {
                                        polygon_vec.push((
                                            poly_id.clone(),
                                            Polygon {
                                                length: poly.len().clone(),
                                                data: poly.clone(),
                                                closed: poly.first() == poly.last(),
                                            },
                                        ));
                                        poly = Vec::new();
                                    }
                                    _ => {}
                                }
                            }
                        }
                        None => {}
                    };
                }
                Event::Tag(tag::SVG, _, attributes) => {
                    match attributes.get("height") {
                        Some(data) => println!("SVG height: {}", data),
                        None => {}
                    }
                    match attributes.get("width") {
                        Some(data) => println!("SVG width: {}", data),
                        None => {}
                    }
                }
                Event::Tag(tag::Group, _, _) => {
                    start_of_poly_group = !start_of_poly_group;
                }
                _ => (),
            }
            let closed = poly.first() == poly.last();
        }
        return PolygonVec {
            length: (polygon_vec.len()),
            data: (polygon_vec),
        };
    }
}
