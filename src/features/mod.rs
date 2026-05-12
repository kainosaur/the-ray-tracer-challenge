use std::f32::EPSILON;

pub mod tuple;
pub mod operators;

pub fn compare_equal(a: f32,b: f32) -> bool {
    if (a-b).abs() < EPSILON {
        return true
    } else {
        return false
    }
}
