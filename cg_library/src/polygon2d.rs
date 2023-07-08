use crate::point2d::Point2D;
use crate::linesegment2d::LineSegment2D;
use crate::tools2d::ccw;

pub struct Polygon2D {

    /// All points of the polygon
    pub points: Vec<Point2D>,

    /// All segments of the polygon
    segments: Vec<LineSegment2D>,
    
    /// Whether the polygon is a simple one
    simple: bool,
    
    /// Whether the polygon is a convex one
    convex: bool,

    /// Maximum x value of the bounding box of the Segment
    max_x: f64,
    
    /// Maximum y value of the bounding box of the Segment
    max_y: f64,
    
    /// Minimum x value of the bounding box of the Segment
    min_x: f64,

    /// Minimum y value of the bounding box of the Segment
    min_y: f64,
    
}

impl Polygon2D {
    pub fn new( points: Vec<Point2D> ) -> Polygon2D {
        if points.len() <= 2 {
            panic!("A polygon consisting of two points is no polygon!");
        }

        let mut segments: Vec<LineSegment2D> = Vec::new();

        let mut old_point = points.first().unwrap();

        let mut min_x = std::f64::MAX.min(old_point.x);
        let mut max_x = std::f64::MIN.max(old_point.x);
        let mut min_y = std::f64::MAX.min(old_point.y);
        let mut max_y = std::f64::MIN.max(old_point.y);
        for point in points.iter().skip(1) {
            segments.push(LineSegment2D::new(*old_point, *point));
            old_point = point;
            min_x = min_x.min(point.x);
            max_x = max_x.max(point.x);
            min_y = min_y.min(point.y);
            max_y = max_y.max(point.y);

        }

        return Polygon2D { points, segments, simple: false, convex: false, max_x, max_y, min_x, min_y };
    }

    pub fn print(&self) {
        println!("Points:");
        for i in &self.points {
            println!("{}", i);
        }
        println!("Segments:");
        for i in &self.segments {
            println!("{}", i);
        }

    }

    /// This actually does not work for points on the polygon
    pub fn contains(&self, q: &Point2D) -> bool {
        
        // Get a point outside of the polygon
        let p_outside: Point2D = Point2D { x: self.max_x + 1.0, y: self.max_y + 1.0 };

        // Retrieve polygon point that is not part of the segment p_outside q 
        let mut i = 0;
        while 0.0 == ccw(&p_outside, &q, &self.points[i]) {
            i+=1;
        }
        // println!("point not between p_outside and q: {}",self.points[i]);
        // println!("p_outside: {}", p_outside);
        // println!("q: {}", q);

        let mut s = 0;
        let mut lr = ccw(&p_outside, q, &self.points[i]).signum();
        for j in (i + 1)..self.points.len() {
            // let g = if j == self.points.len() {0} else {j};
            let lrnew = ccw(&p_outside, q, &self.points[j]).signum();
            // println!("\nlr: {}, lrnew: {}, j: {}",&lr, &lrnew, &j);
            if (lrnew - lr).abs() == 2.0 {
                lr = lrnew;
                if ccw(&self.points[j - 1], &self.points[j], &p_outside)
                    * ccw(&self.points[j - 1], &self.points[j], &q)
                    <= 0.0
                {
                    s+=1;
                    // println!("Adding one: s = {}",s);
                    // println!("\np[j-1]: {} and p[j]: {} means p~: {} and q: {} lie on opposite sides",&self.points[j-1], &self.points[g], p_outside, q);
                }
            }
        }
        return s % 2 == 1;
    }

    /// This actually works for all points
    pub fn contains_point(&self, p: &Point2D) -> bool {
        let mut crossings = 0;
        let n = self.points.len();

        for i in 0..n {
            let j = (i + 1) % n;

            if (self.points[i].y <= p.y && p.y < self.points[j].y)
                || (self.points[j].y <= p.y && p.y < self.points[i].y)
            {
                if p.x < (self.points[j].x - self.points[i].x) * (p.y - self.points[i].y)
                    / (self.points[j].y - self.points[i].y)
                    + self.points[i].x
                {
                    crossings += 1;
                }
            }
        }

        crossings % 2 == 1
    }

}


#[cfg(test)]
mod test_polygon {
    use super::*;

    #[test]
    fn test_new() {
        let points = vec![
            Point2D { x: 0.0, y: 0.0},
            Point2D { x: 1.0, y: 1.0},
            Point2D { x: 1.0, y: 2.0},
            Point2D { x: 2.0, y: 2.0},
            Point2D { x: 3.0, y: 3.0},
        ];

        let _ = Polygon2D::new(points);
        // poly.print();
    }

    #[test]
    #[should_panic]
    fn test_panic() {
        let points = vec![
            Point2D { x: 0.0, y: 0.0},
            Point2D { x: 1.0, y: 1.0},
        ];
        let _ = Polygon2D::new(points);
    }

    #[test]
    fn test_inside() {
        let points = vec![
            Point2D { x: 0.0, y: 0.0},
            Point2D { x: 1.0, y: 1.0},
            Point2D { x: 1.0, y: 2.0},
            Point2D { x: 2.0, y: 2.0},
            Point2D { x: 3.0, y: 3.0},
            Point2D { x: 3.0, y: 0.0},
        ];
        let poly:Polygon2D = Polygon2D::new(points);

        // This point is a point on the polygon
        let p1: Point2D = Point2D { x: 1.0, y: 1.0 };
        assert_eq!(true,poly.contains(&p1));

        // This point is a point on the polygon line
        let p1: Point2D = Point2D { x: 2.0, y: 1.0 };
        assert_eq!(true, poly.contains(&p1));

        // This point is a point on the polygon line
        let p1: Point2D = Point2D { x: 1.0, y: 3.0 };
        assert_eq!(false, poly.contains(&p1));

        // This point is a point on the polygon line
        let p1: Point2D = Point2D { x: 1.1, y: 1.9 };
        assert_eq!(true, poly.contains(&p1));

        // This point is a point on the polygon line
        let p1: Point2D = Point2D { x: 0.9, y: 1.1 };
        assert_eq!(false, poly.contains(&p1));

        // This point is a point on the polygon line
        let p1: Point2D = Point2D { x: 0.5, y: 0.5 };
        assert_eq!(true, poly.contains(&p1));
            
        // poly.print();

        // 3 - # - # - # - o
        // 2 - # - o - o - #
        // 1 - # - o - # - #
        // 0 - o - # - # - #
        //     0 - 1 - 2 - 3

    }

    #[test]
    fn test_should_work() {
        let points = vec![
            Point2D { x: 1.0, y: 1.0},
            Point2D { x: 1.0, y: 2.0},
            Point2D { x: 2.0, y: 2.0},
        ];
        let poly:Polygon2D = Polygon2D::new(points);

        // This point is a point on the polygon line
        let p1: Point2D = Point2D { x: 1.0, y: 1.5 };
        // This is wrong !!!!!
        assert_ne!(true, poly.contains(&p1));
    }


    #[test]
    fn test_contains_point() {
        let points = vec![
            Point2D { x: 0.0, y: 0.0},
            Point2D { x: 1.0, y: 1.0},
            Point2D { x: 1.0, y: 2.0},
            Point2D { x: 2.0, y: 2.0},
            Point2D { x: 3.0, y: 3.0},
            Point2D { x: 3.0, y: 0.0},
        ];
        let poly:Polygon2D = Polygon2D::new(points);

        // This point is a point on the polygon
        let p1: Point2D = Point2D { x: 1.0, y: 1.0 };
        assert_eq!(true,poly.contains_point(&p1));

        // This point is a point on the polygon line
        let p1: Point2D = Point2D { x: 2.0, y: 1.0 };
        assert_eq!(true, poly.contains_point(&p1));

        // This point is a point on the polygon line
        let p1: Point2D = Point2D { x: 1.0, y: 3.0 };
        assert_eq!(false, poly.contains_point(&p1));

        // This point is a point on the polygon line
        let p1: Point2D = Point2D { x: 1.1, y: 1.9 };
        assert_eq!(true, poly.contains_point(&p1));

        // This point is a point on the polygon line
        let p1: Point2D = Point2D { x: 0.9, y: 1.1 };
        assert_eq!(false, poly.contains_point(&p1));

        // This point is a point on the polygon line
        let p1: Point2D = Point2D { x: 0.5, y: 0.5 };
        assert_eq!(true, poly.contains_point(&p1));
            
        // This point is a point on the polygon line
        let p1: Point2D = Point2D { x: 1.0, y: 1.5 };
        assert_eq!(true, poly.contains_point(&p1));

        // 3 - # - # - # - o
        // 2 - # - o - o - #
        // 1 - # - o - # - #
        // 0 - o - # - # - #
        //     0 - 1 - 2 - 3

    }


}
