use std::path::absolute;

use crate::features::compare_equal;

pub struct Tuple<T> {
    x: T,
    y: T,
    z: T,
    w: T
}

impl<T> Tuple<T> {
    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        Self {x, y, z, w}
    }

    pub fn x(&self) -> &T {
        &self.x
    }

    pub fn y(&self) -> &T {
        &self.y
    }

    pub fn z(&self) -> &T {
        &self.z
    }

    pub fn w(&self) -> &T {
        &self.w
    }

    
}

impl Tuple<f32> {
    pub fn is_point(&self) -> bool {
        compare_equal(*self.w(), 1.0)
    }
    
    pub fn is_vector(&self) -> bool {
        compare_equal(*self.w(), 0.)
    }
}