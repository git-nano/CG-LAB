use std::fs;
use svg::node::element::path::{Command, Data, Position};
use svg::node::element::tag;
use svg::parser::Event;

use crate::point::{ccw, Point, point_in_polygon};
use crate::polygon::Polygon;

#[derive(Clone, Debug)]
pub struct Stadt {
    pub id: String,
    pub location: Point<f64>,
}

#[derive(Clone, Debug)]
pub struct StaedteVec {
    pub length: usize,
    pub data: Vec<Stadt>,
}

pub fn stadt_in_bundesland(s: Stadt, b: Polygon) -> bool{
    return point_in_polygon(&s.location, &b);
}

impl StaedteVec{
    pub fn from_svg(path: &str) -> StaedteVec{
        let mut content = String::new();
        let mut staedte_vec: Vec<Stadt> = Vec::new();
        let mut id_vec: Vec<String> = Vec::new();
        let mut x_vec: Vec<f64> = Vec::new();
        let mut y_vec: Vec<f64> = Vec::new();
        let mut cy: f64;
        let mut cx: f64;
        for event in svg::open(path, &mut content).expect("Could not open SVG file!") {
            let mut stadt: Vec<Point<f64>> = Vec::new();
            let mut stadt_id = String::new();
            match event {
                Event::Tag(tag::Path, _, attributes) => {
                    match attributes.get("d") {
                        Some(data) => {continue;}
                        None => {}
                    };
                    match attributes.get("id") {
                        Some(data) => {
                            stadt_id = data.to_string();
                            id_vec.push(stadt_id);
                            }
                        None => {}
                    };
                    match attributes.get("sodipodi:cx") {
                        Some(data) => {
                            cx = data.parse::<f64>().unwrap();
                            x_vec.push(cx);
                            }
                        None => {}
                    };
                    match attributes.get("sodipodi:cy") {
                        Some(data) => {
                            cy = data.parse::<f64>().unwrap();
                            y_vec.push(cy);
                            }
                        None => {}
                    };
                }
                _ => {}
            }
            
        }
        for i in 0..x_vec.len(){
            staedte_vec.push(
                Stadt{
                    id: id_vec[i].clone(), // Start nach den Bundesländern (+16), die am Anfang stehen
                    location: Point{
                        x: x_vec[i],
                        y: y_vec[i],
                    }
                }
            );
        }
        return StaedteVec {
            length: (staedte_vec.len()),
            data: (staedte_vec),
        };
    }
}
