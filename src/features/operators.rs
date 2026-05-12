use std::ops::{Add, Sub};

use crate::features::tuple::Tuple;


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
