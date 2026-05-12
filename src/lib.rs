pub mod features;

#[cfg(test)]
mod tests {
    use crate::features::{compare_equal, tuple::{Point, TFTuple, Tuple, Vector}};
    use std::ops::{Add, Sub};

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn is_point_test() {
        let a = Tuple::new(4.3, -4.2, 3.1, 1.0);
        assert!(compare_equal(a.x(), 4.3));
        assert!(compare_equal(a.y(), -4.2));
        assert!(compare_equal(a.z(), 3.1));
        assert!(compare_equal(a.w(), 1.0));
        assert_eq!(a.is_point(), true);
        assert_eq!(a.is_vector(), false);
    }
    #[test]
    fn is_vector_test() {
        let b = Tuple::new(4.3, -4.2, 3.1, 0.);
        assert!(compare_equal(b.x(), 4.3));
        assert!(compare_equal(b.y(), -4.2));
        assert!(compare_equal(b.z(), 3.1));
        assert!(compare_equal(b.w(), 0.));
        assert_eq!(b.is_point(), false);
        assert_eq!(b.is_vector(), true);
    }
    #[test]
    fn not_equal_tuples() {
        let a = Tuple::new(1., 2., 3., 4.);
        let b = Tuple::new(5., 6., 7., 8.);
        assert!(!a.eq(&b));
    }
    #[test]
    fn point_as_tuple() {
        let p = Point::new(4., -4., 3.);
        let t = Tuple::new(4., -4., 3., 1.);
        assert!(p.as_tuple().eq(&t));
    }
    #[test]
    fn vector_as_tuple() {
        let v = Vector::new(4., -4., 3.);
        let t = Tuple::new(4., -4., 3., 0.);
        assert!(v.as_tuple().eq(&t));
    }
    #[test]
    fn add_tuples() {
        let a = Tuple::new(3., -2., 5., 1.);
        let b = Tuple::new(-2., 3., 1., 0.);
        assert!(a.add(b).eq(&Tuple::new(1., 1., 6., 1.)));
    }
    #[test]
    fn sub_vector_from_point() {
        let p = Point::new(3., 2., 1.,);
        let v = Vector::new(5., 6., 7.,);
        assert!(p.as_tuple().sub(v.as_tuple()).eq(&Point::new(-2., -4., -6.).as_tuple()))
    }
    #[test]
    fn sub_points() {
        let p1 = Point::new(3., 2., 1.);
        let p2 = Point::new(-2., -4., -6.);
        assert!(p1.as_tuple().sub(p2.as_tuple()).eq(&Vector::new(-2.,-4.,-6.).as_tuple()));
    }
}