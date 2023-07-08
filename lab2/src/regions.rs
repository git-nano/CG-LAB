use cg_library::point2d::Point2D;
use cg_library::polygon2d::Polygon2D;
use svg::node::element::path::{Command, Data, Position};
use svg::node::element::tag;
use svg::parser::Event;

const SVG_WIDTH: f64 = 591.504;
const SVG_HEIGHT: f64 = 800.504;
const GERMANY_WIDTH: f64 = 640.0;
const GERMANY_HEIGHT: f64 = 876.0;
const WIDTH_SCALER: f64 = GERMANY_WIDTH / SVG_WIDTH;
const HEIGHT_SCALER: f64 = GERMANY_HEIGHT / SVG_HEIGHT;
const AREA_SCALER: f64 = WIDTH_SCALER * HEIGHT_SCALER;

#[derive(Debug)]
pub struct Polygon2DArea {
    borders: Vec<Polygon2D>,
    holes: Vec<Polygon2D>,
}

impl Polygon2DArea {
    pub fn contains(&self, p: &Point2D) -> bool {
        
        for hole in &self.holes {
            if hole.contains_point(p) {
                return false;
            }
        }
        for border in &self.borders {
            if border.contains_point(p) {
                return true;
            }
        }
        return false;
    }
    pub fn calculate_area(&self) -> f64 {
        let mut area: f64 = 0.0;

        for hole in &self.holes {
            area -= hole.calculate_area().abs();
        }
        for border in &self.borders {
            area += border.calculate_area().abs();
        }
        return area;

    }
}

#[derive(Debug, Clone)]
pub struct City {
    name: String,
    pos: Point2D,
}

#[derive(Debug)]
pub struct State {
    name: String,
    capital: City,
    area: Polygon2DArea,
}

impl State {
    fn fill_holes(&mut self, other: Vec<Polygon2D>) {
        for own_border in &self.area.borders {
            for other_border in &other {
                if own_border.contains_polygon(&other_border) {
                    self.area.holes.push(other_border.clone());
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct Country {
    name: String,
    states: Vec<State>,
    state_capitals: Vec<City>,
    borders: Vec<Polygon2D>,
    holes: Vec<Polygon2D>,
}


impl Country {
    pub fn from_svg(path: &str, country_name: String) -> Country {
        let mut content = String::new();
        let mut group_counter = 0;
        let mut name = String::new();
        let mut cities: Vec<City> = Vec::new();
        let mut states: Vec<State> = Vec::new();

        for event in svg::open(path, &mut content).expect("Could not open SVG file!") {
            match event {
     
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
                    group_counter+=1;
                }

                Event::Tag(tag::Path, _, attributes) => {

                    match attributes.get("id") {
                        Some(data) => {
                            name = data.to_string();
                        }
                        None => {}
                    };

                    // We are now in the state group
                    if group_counter == 1 {
                        
                        let mut poly: Vec<Point2D> = Vec::new();
                        let mut borders: Vec<Polygon2D> = Vec::new();

                        match attributes.get("d") {
                            Some(data) => {
                                let parsed_data = Data::parse(data).expect("Could not parse Data!");
                                for command in parsed_data.iter() {
                                    match command {
                                        Command::Move(_rel_or_abs, parameters) => poly.push(Point2D {
                                            x: parameters[0].clone() as f64,
                                            y: parameters[1].clone() as f64,
                                        }),
                                        Command::Line(rel_or_abs, parameters) => match rel_or_abs {
                                            Position::Relative => poly.push(Point2D {
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
                                            Position::Absolute => poly.push(Point2D {
                                                x: parameters[0].clone() as f64,
                                                y: parameters[1].clone() as f64,
                                            }),
                                        },
                                        Command::Close => {
                                            borders.push(Polygon2D::new(poly));
                                            poly = Vec::new();
                                        }
                                        _ => {}
                                    }
                                }
                            }
                            None => {}
                        };
                        states.push(State { name: name.clone(), capital: City { name: String::new(), pos: Point2D::new()}, area: Polygon2DArea { borders, holes: Vec::new() }});
                    }

                    // We are now in the city group
                    if group_counter == 2 {

                        let mut pos: Point2D = Point2D::new();
                        match attributes.get("sodipodi:cx") {
                            Some(data) => {
                                pos.x = data.parse::<f64>().unwrap();
                            }
                            None => {}
                        };
                        match attributes.get("sodipodi:cy") {
                            Some(data) => {
                                pos.y = data.parse::<f64>().unwrap();
                            }
                            None => {}
                        };
                        cities.push(City {name: name.clone(), pos});
                    }
                }
                _ => {},
            }
        }
        return Country {name: country_name, states, state_capitals: cities,borders: Vec::new(), holes: Vec::new()};
    }

    pub fn fill(&mut self) {

        for i in 0..self.states.len() {
            for j in 0..self.states.len() {
                if j != i {
                    let borders = self.states[j].area.borders.clone();
                    self.states[i].fill_holes(borders);
                }
            }
            for city in &self.state_capitals {
                if self.states[i].area.contains(&city.pos) {
                    self.states[i].capital = city.clone();
                }
            }
        }

    }
    pub fn print(self) {
        println!("Country: {}", self.name);
        for state in self.states {
            println!("\tState: {}", state.name);
            println!("\t\tCapital: {}", state.capital.name);
            println!("\t\tBorders: {}", state.area.borders.len());
            println!("\t\tHoles: {}", state.area.holes.len());
            println!("\t\tArea: {:.1}", state.area.calculate_area());
            println!("\t\tArea in m²: {:.1}", state.area.calculate_area() * AREA_SCALER);
        }
    }
}

#[cfg(test)]
mod test_city {
    use super::*;

    #[test]
    fn test_from() {
        let mut germany = Country::from_svg("./DeutschlandMitStaedten.svg", String::from("Germany"));
        germany.fill();
        germany.print();
    }
}



