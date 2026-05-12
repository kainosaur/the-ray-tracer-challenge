pub mod features;

#[cfg(test)]
mod tests {
    use crate::features::{compare_equal, tuple::Tuple};

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn is_point_test() {
        let a = Tuple::new(4.3, -4.2, 3.1, 1.0);
        assert!(compare_equal(*a.x(), 4.3));
        assert!(compare_equal(*a.y(), -4.2));
        assert!(compare_equal(*a.z(), 3.1));
        assert!(compare_equal(*a.w(), 1.0));
        assert_eq!(a.is_point(), true);
        assert_eq!(a.is_vector(), false);
    }
    #[test]
    fn is_vector_test() {
        let b = Tuple::new(4.3, -4.2, 3.1, 0.);
        assert!(compare_equal(*b.x(), 4.3));
        assert!(compare_equal(*b.y(), -4.2));
        assert!(compare_equal(*b.z(), 3.1));
        assert!(compare_equal(*b.w(), 0.));
        assert_eq!(b.is_point(), false);
        assert_eq!(b.is_vector(), true);
    }
}