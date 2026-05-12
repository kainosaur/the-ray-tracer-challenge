pub mod features;

#[cfg(test)]
mod tests {
    use crate::features::{compare_equal, tuple::{Point, TFTuple, Tuple, Vector}, operators::{Magnitude, Normalize, Dot}};
    use std::ops::{Add, Sub, Mul, Div};

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
    fn sub_points() {
        let p1 = Point::new(3., 2., 1.);
        let p2 = Point::new(5., 6., 7.);
        assert!(p1.sub(p2).eq(&Vector::new(-2.,-4.,-6.)));
    }
    #[test]
    fn sub_vector_from_point() {
        let p = Point::new(3., 2., 1.,);
        let v = Vector::new(5., 6., 7.,);
        assert!(p.sub(v).eq(&Point::new(-2., -4., -6.)));
    }
    #[test]
    fn sub_vectors() {
        let v1 = Vector::new(3., 2., 1.);
        let v2 = Vector::new(5., 6., 7.);
        assert!(v1.sub(v2).eq(&Vector::new(-2., -4., -6.)));
    }
    #[test]
    fn sub_from_zero_vector() {
        let zero_vector = Vector::new(0.,0.,0.);
        let v = Vector::new(1.,-2., 3.);
        assert!(zero_vector.sub(v).eq(&Vector::new(-1., 2., -3.)));
    }
    #[test]
    fn negate_tuple() {
        let t = Tuple::new(1., -2., 3., -4.);
        assert!(t.negate().eq(&Tuple::new(-1., 2., -3., 4.)));
    }
    #[test]
    fn mul_scalar_tuple() {
        let a = Tuple::new(1., -2., 3., -4.);
        assert!(a.mul(3.5).eq(&Tuple::new(3.5, -7., 10.5, -14.)));
    }
    #[test]
    fn frac_mul_scalar_tuple() {
        let a = Tuple::new(1., -2., 3., -4.);
        assert!(a.mul(0.5).eq(&Tuple::new(0.5, -1., 1.5, -2.)));
    }
    #[test]
    fn div_scalar_tuple() {
        let a = Tuple::new(1., -2., 3., -4.);
        assert!(a.div(2.).eq(&Tuple::new(0.5, -1., 1.5, -2.)));
    }
    #[test]
    fn magnitude_unit_vector() {
        let v1 = Vector::new(1., 0., 0.);
        let v2 = Vector::new(0., 1., 0.);
        let v3 = Vector::new(0., 0., 1.);
        assert!(compare_equal(v1.magnitude(), 1.));
        assert!(compare_equal(v2.magnitude(), 1.));
        assert!(compare_equal(v3.magnitude(), 1.));
    }
    #[test]
    fn magnitude_of_vectors() {
        let v1 = Vector::new(1., 2., 3.);
        let v2 = Vector::new(-1., -2., -3.);
        assert!(compare_equal(v1.magnitude(), (14_f32).sqrt()));
        assert!(compare_equal(v2.magnitude(), (14_f32).sqrt()));
        assert!(compare_equal(v1.magnitude(), v2.magnitude()));
    }
    #[test]
    fn normalize_vector_basic() {
        let v = Vector::new(4., 0., 0.);
        assert!(v.normalize().eq(&Vector::new(1., 0., 0.)));
    }
    #[test]
    fn normalize_vector_and_check_if_unit() {
        let v = Vector::new(1., 2., 3.);
        assert!(v.normalize().eq(&Vector::new(1./(14_f32).sqrt(), 2./(14_f32).sqrt(), 3./(14_f32).sqrt())));
        assert!(compare_equal(v.normalize().magnitude(), 1.));
    }
    #[test]
    fn dot_vectors() {
        let a = Vector::new(1., 2., 3.);
        let b = Vector::new(2., 3., 4.);
        assert!(compare_equal(a.dot(&b), 20.));
    }
    #[test]
    fn cross_product_vectors() {
        let a = Vector::new(1., 2., 3.);
        let b = Vector::new(2., 3., 4.);
        assert!(a.cross(&b).eq(&Vector::new(-1., 2., -1.)));
        assert!(b.cross(&a).eq(&Vector::new(1., -2., 1.)));
    }
}