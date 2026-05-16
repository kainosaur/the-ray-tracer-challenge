use crate::features::tuple::{Color, Point};

#[derive(Clone, Copy, Debug)]
pub struct PointLight<T> {
    pub position: Point<T>,
    pub intensity: Color<T>
}

impl<T> PointLight<T> {
    pub fn new(position: Point<T>, intensity: Color<T>) -> Self {
        Self {
            position: position,
            intensity: intensity,
        }
    }
}