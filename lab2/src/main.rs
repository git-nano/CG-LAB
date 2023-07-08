#![allow(dead_code)]

mod regions;
use regions::Country;

/*
    New version of german states: https://upload.wikimedia.org/wikipedia/commons/2/2c/Karte_Bundesrepublik_Deutschland.svg
    Areas of Germany listed: https://de.wikipedia.org/wiki/Liste_der_Deutschen_Bundesl%C3%A4nder_nach_Fl%C3%A4che
    Width and height of German: https://www.lernhelfer.de/schuelerlexikon/geografie/artikel/bundesrepublik-deutschland#
    Width and height of SVG: Read SVG-Header
*/

fn main() {
    let mut germany = Country::from_svg("./DeutschlandMitStaedten.svg", String::from("Germany"));
    germany.fill();
    germany.print();
}
