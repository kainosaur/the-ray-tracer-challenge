use std::ops::{Add, Mul, Sub, Div};

use crate::features::tuple::{Point, Tuple, Vector};

trait Magnitude {
    fn magnitude(&self) -> f32;
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
