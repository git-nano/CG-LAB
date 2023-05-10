#![allow(dead_code)]

use std::ops::{Add,Sub,Mul,Div,Index};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

pub fn ccw(p: &Point<f64>, q: &Point<f64>, r: &Point<f64>) -> f64 {
    return (p.x * q.y - p.y * q.x) + (q.x * r.y - q.y * r.x) + (p.y * r.x - p.x * r.y);
}

pub trait Abs {
    type Output;
    fn abs(self) -> Self::Output;
}

impl Abs for Point<f64> {
    type Output = Self;
    fn abs(self) -> Self::Output {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }
}

impl <T>Add for Point<T> 
where T: Add<Output = T>{
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl <T>Sub for Point<T> 
where T: Sub<Output = T>{
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl <T>Mul for Point<T> 
where T: Mul<Output = T>{
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl <T>Div for Point<T> 
where T: Div<Output = T>{
    type Output = Self;
    fn div(self, other: Self) -> Self::Output {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

impl <T>Index<usize> for Point<T> {
    type Output = T;
    fn index(&self, i: usize) -> &Self::Output {
        match i {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Add more index values in the index function, if dimension is increased!"),
        }
    }
}

impl<T> Point<T> 
where T: PartialOrd {
    pub fn index_of_significance(&self) -> usize {
        if self.x > self.y {
            return 0;
        }
        return 1;
    }
}

