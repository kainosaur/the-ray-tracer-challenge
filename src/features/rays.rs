use crate::features::{matrices::Matrix4x4, tuple::{Point, Vector}};

pub struct Ray<T> {
    pub origin: Point<T>,
    pub direction: Vector<T>
}

impl<T> Ray<T> {
    pub fn new(origin: Point<T>, direction: Vector<T>) -> Self {
        Self {
            origin,
            direction
        }
    }
}

impl Ray<f32> {
    pub fn position(&self, distance: f32) -> Point<f32> {
        self.origin + self.direction * distance
    }

    pub fn transform(&self, transform_matrix: Matrix4x4<f32>) -> Self {
        Ray::new((transform_matrix * self.origin).as_point(), transform_matrix * self.direction)
    }
}