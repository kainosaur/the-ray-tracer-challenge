use std::ops::{Add, Div, Mul, Sub};

use crate::features::tuple::{Color, Point, Tuple, Vector};

pub trait Magnitude {
    fn magnitude(&self) -> f32;
}

pub trait Normalize {
    fn normalize(&self) -> Self;
}

pub trait Dot {
    fn dot(&self, other: &Self) -> f32;
}

impl Add for Tuple<f32> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Tuple::new(
            self.x() + other.x(),
            self.y() + other.y(),
            self.z() + other.z(),
            self.w() + other.w()
        )
    }
}

impl Sub for Tuple<f32> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Tuple::new(
            self.x() - other.x(),
            self.y() - other.y(),
            self.z() - other.z(),
            self.w() - other.w()
        )
    }
}

impl Mul<f32> for Tuple<f32> {
    type Output = Tuple<f32>;

    fn mul(self, rhs: f32) -> Self::Output {
        Tuple::new(
            self.x() * rhs,
            self.y() * rhs,
            self.z() * rhs,
            self.w() * rhs
        )
    }
}

impl Div<f32> for Tuple<f32> {
    type Output = Tuple<f32>;

    fn div(self, rhs: f32) -> Self::Output {
        Tuple::new(
            self.x() / rhs,
            self.y() / rhs,
            self.z() / rhs,
            self.w() / rhs
        )
    }
}

impl Magnitude for Tuple<f32> {
    fn magnitude(&self) -> f32 {
        (self.x().powf(2.) + self.y().powf(2.) + self.z().powf(2.) + self.w().powf(2.)).sqrt()
    }
}

impl Normalize for Tuple<f32> {
    fn normalize(&self) -> Self {
        let m = self.magnitude();
            Tuple::new(
                self.x()/m,
                self.y()/m,
                self.z()/m,
                self.z()/m
            )
    }
}

impl Magnitude for Vector<f32> {
    fn magnitude(&self) -> f32 {
        (self.x().powf(2.) + self.y().powf(2.) + self.z().powf(2.)).sqrt()
    }
}

impl Normalize for Vector<f32> {
    fn normalize(&self) -> Self {
        let m = self.magnitude();
        Vector::new(
            self.x()/m,
            self.y()/m,
            self.z()/m
        )
    }
}

impl Dot for Vector<f32> {
    fn dot(&self, other: &Self) -> f32 {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z()
    }
}

impl Vector<f32> {
    pub fn cross(&self, other: &Self) -> Self {
        Vector::new(
            self.y() * other.z() - self.z() * other.y(),
            self.z() * other.x() - self.x() * other.z(),
            self.x() * other.y() - self.y() * other.x()
        )
    }
}

impl Sub for Point<f32> {
    // Subtracting points produces a vector.
    type Output = Vector<f32>;

    fn sub(self, other: Self) -> Self::Output {
        Vector::new(
            self.x() - other.x(),
            self.y() - other.y(),
            self.z() - other.z(),
        )
    }
}

impl Add<Vector<f32>> for Point<f32> {
    // Adding a vector and point produces a point.
    type Output = Point<f32>;

    fn add(self, other: Vector<f32>) -> Self::Output {
        Point::new(
            self.x() + other.x(),
            self.y() + other.y(),
            self.z() + other.z(),
        )
    }
}

impl Sub<Vector<f32>> for Point<f32> {
    // Subtracting a vector from a point is a point
    type Output = Point<f32>;

    fn sub(self, other: Vector<f32>) -> Self::Output {
        Point::new(
            self.x() - other.x(),
            self.y() - other.y(),
            self.z() - other.z(),
        )
    }
}

impl Add for Vector<f32> {
    // Adding vectors produces a vector.
    type Output = Vector<f32>;

    fn add(self, other: Self) -> Self::Output {
        Vector::new(
            self.x() + other.x(),
            self.y() + other.y(),
            self.z() + other.z(),
        )
    }
}

impl Sub for Vector<f32> {
    // Subtracting vectors produces a vector.
    type Output = Vector<f32>;

    fn sub(self, other: Self) -> Self::Output {
        Vector::new(
            self.x() - other.x(),
            self.y() - other.y(),
            self.z() - other.z(),
        )
    }
}

impl Add for Color<f32> {
    type Output = Color<f32>;

    fn add(self, other: Self) -> Self::Output {
        Color::new(
            self.red + other.red,
            self.green + other.green,
            self.blue + other.blue
        )
    }
}

impl Sub for Color<f32> {
    type Output = Color<f32>;

    fn sub(self, other: Self) -> Self::Output {
        Color::new(
            self.red - other.red,
            self.green - other.green,
            self.blue - other.blue
        )
    }
}

impl Mul<f32> for Vector<f32> {
    type Output = Vector<f32>;

    fn mul(self, rhs: f32) -> Self::Output {
        Vector::new(
            self.x() * rhs,
            self.y() * rhs,
            self.z() * rhs
        )
    }
}
    
    

impl Mul<f32> for Color<f32> {
    type Output = Color<f32>;

    fn mul(self, rhs: f32) -> Self::Output {
        Color::new(
            self.red * rhs,
            self.green * rhs,
            self.blue * rhs
        )
    }
}

impl Mul for Color<f32> {
    type Output = Color<f32>;

    fn mul(self, other: Self) -> Self::Output {
        Color::new(
            self.red * other.red,
            self.green * other.green,
            self.blue * other.blue
        )
    }
}