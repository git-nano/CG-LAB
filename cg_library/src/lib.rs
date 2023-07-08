#![allow(unused_assignments, dead_code)]

pub mod point2d;
pub mod line2d;
pub mod linesegment2d;
pub mod tools2d;
pub mod polygon2d;
pub mod util {
    pub mod eventpoint;
    // pub mod sweepline;
    // pub mod eventqueue;
}


pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
