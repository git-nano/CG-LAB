use std::fs;
use svg::node::element::path::{Command, Data, Position};
use svg::node::element::tag;
use svg::parser::Event;

use crate::point::{ccw, Point};

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

impl StaedteVec{
    pub fn from_svg(path: &str) -> StaedteVec{
        let mut content = String::new();
        let mut staedte_vec: Vec<Stadt> = Vec::new();
        let mut id_vec: Vec<String> = Vec::new();
        let mut x_vec: Vec<f64> = Vec::new();
        let mut y_vec: Vec<f64> = Vec::new();
        let mut cy: f64;
        let mut cx: f64;
        let mut i = 0;
        for event in svg::open(path, &mut content).expect("Could not open SVG file!") {
            let mut stadt: Vec<Point<f64>> = Vec::new();
            let mut stadt_id = String::new();
            match event {
                Event::Tag(tag::Path, _, attributes) => {
                    match attributes.get("id") {
                        Some(data) => {
                            stadt_id = data.to_string();
                            id_vec.push(stadt_id);
                            //println!("{}", stadt_id);
                            }
                        None => {}
                    };
                    //println!("{}", stadt_id);
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
                    
                    i = i + 1;
                    
                    //let p = Point {
                    //    x: x_vec,
                    //    y: y_vec,
                    //};
                    /* 
                                    staedte_vec.push((
                                        stadt_id.clone(),
                                        Point {
                                            length: poly.len().clone(),
                                            data: poly.clone(),
                                            closed: poly.first() == poly.last(),
                                        },
                                    ));
                                    poly = Vec::new();
                */
                }
                _ => {}
            }
        }
        for i in 0..x_vec.len(){
            staedte_vec.push(
                Stadt{
                    id: id_vec[i+16].clone(), // Start nach den Bundesländern (+16), die am Anfang stehen
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