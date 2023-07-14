use crate::point::Point;

#[test]
fn test_point_addition_f64() {
    let p1: Point<f64> = Point { x: 1.1, y: 2.2 };
    let p2: Point<f64> = Point { x: 3.5, y: 4.2 };
    assert!(Point { x: 4.6, y: 6.4 } == p1 + p2);
}
#[test]
fn test_point_addition_u64() {
    let p1: Point<u64> = Point { x: 1, y: 2 };
    let p2: Point<u64> = Point { x: 3, y: 4 };
    assert!(Point { x: 4, y: 6 } == p1 + p2);
}
#[test]
fn test_point_addition_str() {
    let p1: Point<&str> = Point {
        x: "Hello",
        y: "Its",
    };
    let _p2: Point<&str> = Point {
        x: "World",
        y: "Me",
    };
    println!("{:?}", p1);
}
