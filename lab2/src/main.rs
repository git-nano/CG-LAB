#![allow(dead_code, unused)]

mod point;
mod polygon;
use polygon::PolygonVec;

/*
    New version of german states: https://upload.wikimedia.org/wikipedia/commons/2/2c/Karte_Bundesrepublik_Deutschland.svg
    Areas of Germany listed: https://de.wikipedia.org/wiki/Liste_der_Deutschen_Bundesl%C3%A4nder_nach_Fl%C3%A4che
    Width and height of German: https://www.lernhelfer.de/schuelerlexikon/geografie/artikel/bundesrepublik-deutschland#
    Width and height of SVG: Read SVG-Header
*/

const SVG_WIDTH: f64 = 591.504;
const SVG_HEIGHT: f64 = 800.504;
const GERMANY_WIDTH: f64 = 640.0;
const GERMANY_HEIGHT: f64 = 876.0;
const WIDTH_SCALER: f64 = GERMANY_WIDTH / SVG_WIDTH;
const HEIGHT_SCALER: f64 = GERMANY_HEIGHT / SVG_HEIGHT;
const AREA_SCALER: f64 = WIDTH_SCALER * HEIGHT_SCALER;

fn main() {
    let file = "Karte_Bundesrepublik_Deutschland.svg";
    let file = "DeutschlandMitStaedten.svg";
    let polygon = PolygonVec::from_svg(file);
    let mut area = 0.0;
    let mut old_bundesland = polygon.data[0].0.clone();
    for i in 0..polygon.length {
        let (bundesland, data) = &polygon.data[i];
        if bundesland == &old_bundesland {
            area += data.calculate_area().abs() * AREA_SCALER;
        } else {
            println!("Area of {}: {:.0}km²", old_bundesland, area);
            area = data.calculate_area() * AREA_SCALER;
            old_bundesland = bundesland.clone();
        }
    }
}

mod test {
    use super::polygon::Polygon;

    #[test]
    fn test_area() {
        let poly = Polygon::from_string("1 1;3 1;3 2;2 3;1 2; 1 1");
        println!("Calculated Area: {}", poly.calculate_area());

        // println!("Polygon data: {:?}",poly.data);
        assert!(poly.calculate_area() == 1.0);
    }
}
