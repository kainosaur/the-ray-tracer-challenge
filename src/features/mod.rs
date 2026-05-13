pub mod tuple;
pub mod operators;
pub mod canvas;

pub fn compare_equal(a: f32,b: f32) -> bool {
    if (a-b).abs() < 0.00001 {
        return true
    } else {
        return false
    }
}
