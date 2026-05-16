use std::ops::Mul;

use crate::features::{compare_equal, operators::Dot};

#[derive(Clone, Copy, Debug)]
pub struct Color<T> (pub[T; 3]);

#[derive(Clone, Copy, Debug)]
pub struct Tuple<T> (pub [T; 4]);

#[derive(Clone, Copy, Debug)]
pub struct Point<T> (pub[T; 3]);

#[derive(Clone, Copy, Debug)]
pub struct Vector<T> (pub[T; 3]);

impl<T> Tuple<T> {
    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        Self ([x, y, z, w])
    }
    
}

impl Tuple<f32> {
    pub fn is_point(&self) -> bool {
        compare_equal(self.w(), 1.0)
    }
    
    pub fn is_vector(&self) -> bool {
        compare_equal(self.w(), 0.)
    }

    pub fn x(&self) -> f32 {
        self.0[0]
    }

    pub fn y(&self) -> f32 {
        self.0[1]
    }

    pub fn z(&self) -> f32 {
        self.0[2]
    }

    pub fn w(&self) -> f32 {
        self.0[3]
    }

    pub fn as_point(&self) -> Point<f32> {
        if self.is_point() {
            Point::new(self.x(), self.y(), self.z())
        } else {
            panic!("Tried changing into a point when not a point");
        }
        
    }

    pub fn as_vector(&self) -> Vector<f32> {
        if self.is_vector() {
            Vector::new(self.x(), self.y(), self.z())
        } else {
            panic!("Tried changing into a vector when not a vector");
        }
    }

    pub fn at(&self, row: usize) -> f32 {
        match row {
            0 => self.x(),
            1 => self.y(),
            2 => self.z(),
            3 => self.w(),
            _=> panic!("Row out of bounds.")
        }
    }

    pub fn assign(&mut self, row: usize, n: f32) -> Self {
        match row {
            0 => self.0[0] = n,
            1 => self.0[1] = n,
            2 => self.0[2] = n,
            3 => self.0[3] = n,
            _=> panic!("Row out of bounds.")
        }
        *self
    }

    pub fn negate(&self) -> Self {
        Tuple::new(
            -self.x(),
            -self.y(),
            -self.z(),
            -self.w()
        )
    }
}

impl<T> Point<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self ([x, y, z])
    }
}

impl Point<f32> {
    pub fn x(&self) -> f32 {
        self.0[0]
    }

    pub fn y(&self) -> f32 {
        self.0[1]
    }

    pub fn z(&self) -> f32 {
        self.0[2]
    }
}

impl Vector<f32> {
    pub fn x(&self) -> f32 {
        self.0[0]
    }

    pub fn y(&self) -> f32 {
        self.0[1]
    }

    pub fn z(&self) -> f32 {
        self.0[2]
    }
    
    pub fn negate(&self) -> Self {
        Vector::new(
            -self.x(),
            -self.y(),
            -self.z(),
        )
    }

    pub fn reflect(&self, normal: Vector<f32>) -> Self {
        *self - normal * 2. * self.dot(&normal)
    }
}

pub trait TFTuple {
    fn as_tuple(&self) -> Tuple<f32>;
}

impl TFTuple for Point<f32> {
    fn as_tuple(&self) -> Tuple<f32> {
        return Tuple::new(self.x(), self.y(), self.z(), 1.0)
    }
}

impl TFTuple for Vector<f32> {
    fn as_tuple(&self) -> Tuple<f32> {
        return Tuple::new(self.x(), self.y(), self.z(), 0.0)
    }
}

impl TFTuple for Color<f32> {
    fn as_tuple(&self) -> Tuple<f32> {
        return Tuple::new(self.red(), self.green(), self.blue(), 0.0)
    }
}

impl<T> Vector<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self ([x, y, z])
    }
}

impl<T> Color<T> {
    pub fn new(red: T, green: T, blue: T) -> Self {
        Self ([red, green, blue])
    }
}

impl Color<f32> {
    pub fn ppm_components(&self) -> [String; 3] {
        [
            (self.red().clamp(0., 1.).mul(255.).round() as i32).to_string(),
            (self.green().clamp(0., 1.).mul(255.).round() as i32).to_string(),
            (self.blue().clamp(0., 1.).mul(255.).round() as i32).to_string(),
        ]
    }

    pub fn tuple_string(&self) -> String {
        self.ppm_components().join(" ")
    }

    pub fn red(&self) -> f32 {
        self.0[0]
    }

    pub fn green(&self) -> f32 {
        self.0[1]
    }

    pub fn blue(&self) -> f32 {
        self.0[2]
    }
}

impl PartialEq for Tuple<f32> {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..4 {
            if self.0[i] != other.0[i] {
                return false
            }
        }
        true
    }
}

impl PartialEq for Vector<f32> {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..3 {
            if !compare_equal(self.0[i], other.0[i]) {
                return false
            }
        }
        true
    }
}

impl PartialEq for Point<f32> {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..3 {
            if !compare_equal(self.0[i], other.0[i]) {
                return false
            }
        }
        true
    }
}

impl PartialEq for Color<f32> {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..3 {
            if !compare_equal(self.0[i], other.0[i]) {
                return false
            }
        }
        true
    }
}
