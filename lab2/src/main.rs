#![allow(dead_code)]

mod regions;
use regions::Country;

fn main() {
    let mut germany = Country::from_svg("./DeutschlandMitStaedten.svg", String::from("Germany"));
    germany.fill();
    germany.print();
}
