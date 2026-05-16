pub mod features;
pub mod objects;

#[cfg(test)]
mod tests {
    use crate::{features::{canvas::Canvas, compare_equal, intersections::{Intersection, Intersections}, matrices::{Axis, Matrix2x2, Matrix3x3, Matrix4x4}, operators::{Dot, Magnitude, Normalize}, rays::Ray, tuple::{Color, Point, TFTuple, Tuple, Vector}}, objects::{Object, spheres::Sphere}};
    use std::{f32::consts::PI, ops::{Add, Div, Mul, Sub}};
    use crate::tests::Object::SphereObject;

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
    #[test]
    fn color_tuple() {
        let c = Color::new(-0.5, 0.4, 1.7);
        assert!(compare_equal(c.red(), -0.5));
        assert!(compare_equal(c.green(), 0.4));
        assert!(compare_equal(c.blue(), 1.7));
    }
    #[test]
    fn add_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        assert!(c1.add(c2).eq(&Color::new(1.6, 0.7, 1.0)));
    }
    #[test]
    fn sub_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        assert!(c1.sub(c2).eq(&Color::new(0.2, 0.5, 0.5))) ;
    }
    #[test]
    fn mul_scalar_color() {
        let c = Color::new(0.2, 0.3, 0.4);
        assert!(c.mul(2.).eq(&Color::new(0.4, 0.6, 0.8)));
    }
    #[test]
    fn mul_colors_op() {
        let c1 = Color::new(1., 0.2, 0.4);
        let c2 = Color::new(0.9, 1., 0.1);
        assert_eq!(c1.mul(c2),Color::new(0.9, 0.2, 0.04));
    }
    #[test]
    fn all_pixels_black() {
        let x = 10;
        let y = 20;
        let canvas = Canvas::new(x,y);
        for i in 0..x {
            for j in 0..y {
                assert!(canvas.pixel_at(i, j).eq(&Color::new(0.,0.,0.)));
            }
        }
    }
    #[test]
    fn write_pixel_test() {
        let mut canvas = Canvas::new(10, 20);
        let red = Color::new(1., 0., 0.);
        canvas.write_pixel(2, 3, red);
        assert_eq!(canvas.pixel_at(2, 3), red);
        assert_eq!(canvas.pixel_at(2, 4), Color::new(0.,0.,0.));
    }
    #[test]
    fn write_file_string() {
        let mut c = Canvas::new(5,3);
        let c1 = Color::new(1.5, 0., 0.);
        let c2 = Color::new(0., 0.5, 0.);
        let c3 = Color::new(-0.5, 0., 1.);
        c.write_pixel(0, 0, c1);
        c.write_pixel(2, 1, c2);
        c.write_pixel(4, 2, c3);
        let ppm_string = c.create_file_string();
        let compared_ppm_string = String::from
            ("P3\n5 3\n255\n255 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 128 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 0 0 0 0 0 0 0 255");
        assert_eq!(ppm_string, compared_ppm_string);
    }
    #[test]
    fn pixel_formatting() {
        let mut canvas = Canvas::new(10,2);
        let color = Color::new(1., 0.8, 0.6);
        canvas.overwrite_all_pixels(color);
    }
    #[test]
    fn matrix4x4_test() {
        let m4x4 = Matrix4x4::new(1., 2., 3., 4., 5.5,
            6.5, 7.5, 8.5, 9.,
            10., 11., 12., 13.5,
            14.5, 15.5, 16.5
        );

        assert!(compare_equal(m4x4.at(0,0), 1.));
        assert!(compare_equal(m4x4.at(0,3), 4.));
        assert!(compare_equal(m4x4.at(1,0), 5.5));
        assert!(compare_equal(m4x4.at(1,2), 7.5));
        assert!(compare_equal(m4x4.at(2,2), 11.));
        assert!(compare_equal(m4x4.at(3,0), 13.5));
        assert!(compare_equal(m4x4.at(3,2), 15.5));
    }
    #[test]
    fn matrix2x2() {
        let m2x2 = Matrix2x2::new(-3.,5., 1., -2.);
        
        assert!(compare_equal(m2x2.at(0,0), -3.));
        assert!(compare_equal(m2x2.at(0,1), 5.));
        assert!(compare_equal(m2x2.at(1,0), 1.));
        assert!(compare_equal(m2x2.at(1,1), -2.));
    }
    #[test]
    fn matrix3x3() {
        let m3x3 = Matrix3x3::new(-3., 5., 0., 1., -2., -7., 0., 1., 1.);

        assert!(compare_equal(m3x3.at(0, 0), -3.));
        assert!(compare_equal(m3x3.at(1, 1), -2.));
        assert!(compare_equal(m3x3.at(2, 2), 1.));
    }
    #[test]
    fn matrix4x4_equality() {
        let a = Matrix4x4 ([1., 2., 3., 4.], [5., 6., 7., 8.], [9., 8., 7., 6.], [5., 4., 3., 2.]);
        let b = Matrix4x4 ([1., 2., 3., 4.], [5., 6., 7., 8.], [9., 8., 7., 6.], [5., 4., 3., 2.]);
        assert_eq!(a,b);
    }
    #[test]
    fn matrix4x4_nonequality() {
        let a = Matrix4x4 ([1., 2., 3., 4.], [5., 6., 7., 8.], [9., 8., 7., 6.], [5., 4., 3., 2.]);
        let b = Matrix4x4([2., 3., 4., 5.], [6., 7., 8., 9.], [8., 7., 6., 5.], [4., 3., 2., 1.]);
        assert_ne!(a,b);
    }
    #[test]
    fn mul_matrix4x4() {
        let a = Matrix4x4 ([1., 2., 3., 4.], [5., 6., 7., 8.], [9., 8., 7., 6.], [5., 4., 3., 2.]);
        let b = Matrix4x4([-2., 1., 2., 3.], [3., 2., 1., -1.], [4., 3., 6., 5.], [1., 2., 7., 8.]);
        let sol = Matrix4x4([20., 22., 50., 48.], [44., 54., 114., 108.], [40., 58., 110., 102.], [16., 26., 46., 42.]);
        assert_eq!(a * b, sol);
    }
    #[test]
    fn mul_matrix_tup() {
        let m = Matrix4x4([1., 2., 3., 4.], [2., 4., 4., 2.], [8., 6., 4., 1.], [0., 0., 0., 1.]);
        let t: Tuple<f32> = Tuple::new(1., 2., 3., 1.);
        let sol: Tuple<f32> = Tuple::new(18., 24., 33., 1.);
        assert_eq!(m * t, sol);
    }
    #[test]
    fn identity_matrix() {
        let a = Matrix4x4 ([1., 2., 3., 4.], [5., 6., 7., 8.], [9., 8., 7., 6.], [5., 4., 3., 2.]);
        assert_eq!(a * Matrix4x4::identity(), a)
    }
    #[test]
    fn transpose_matrix() {
        let m = Matrix4x4 (
            [0., 9., 3., 0.],
            [9., 8., 0., 8.],
            [1., 8., 5., 3.],
            [0., 0., 5., 8.]
        );
        let transpose_final = Matrix4x4 (
            [0., 9., 1., 0.],
            [9., 8., 8., 0.],
            [3., 0., 5., 5.],
            [0., 8., 3., 8.]
        );

        assert_eq!(m.transpose(), transpose_final);
    }
    #[test]
    fn transpose_identity_matrix() {
        let identity = Matrix4x4::identity();
        assert_eq!(identity.transpose(), identity);
    }
    #[test]
    fn submatrix_2x2() {
        let m = Matrix3x3 ([-1., 5., 0.], [-3., 2., 7.], [0., 6., -3.]);
        let subm = Matrix2x2 ([-3., 2.], [0., 6.]);
        assert_eq!(m.submatrix(0,2), subm);
    }
    #[test]
    fn submatrix_3x3() {
        let m = Matrix4x4(
            [-6., 1., 1., 6.],
            [-8., 5., 8., 6.],
            [-1., 0., 8., 2.],
            [-7., 1., -1., 1.]
        );

        let subm = Matrix3x3(
            [-6., 1., 6.],
            [-8., 8., 6.],
            [-7., -1., 1.]
        );

        assert_eq!(m.submatrix(2,1), subm);
    }
    #[test]
    fn minor_matrix3x3() {
        let a = Matrix3x3 (
            [3., 5., 0.],
            [2., -1., -7.],
            [6., -1., 5.]
        );
        let b = a.submatrix(1,0);
        assert_eq!(a.minor(1, 0), b.determinant());
    }
    #[test]
    fn cofactor_matrix3x3() {
        let a = Matrix3x3 (
            [3., 5., 0.],
            [2., -1., -7.],
            [6., -1., 5.]
        );
        assert!(compare_equal(a.minor(0,0), -12.));
        assert!(compare_equal(a.cofactor(0, 0), -12.));
        assert!(compare_equal(a.minor(1, 0), 25.));
        assert!(compare_equal(a.cofactor(1, 0), -25.))
    }
    #[test]
    fn determinate_matrix3x3() {
        let a = Matrix3x3 (
            [1., 2., 6.],
            [-5., 8., -4.],
            [2., 6., 4.]
        );

        assert_eq!(a.determinant(), -196.);
    }
    #[test]
    fn determinant_matrix4x4() {
        let a = Matrix4x4 (
            [-2., -8., 3., 5.],
            [-3., 1., 7., 3.],
            [1., 2., -9., 6.],
            [-6., 7., 7., -9.]
        );

        assert_eq!(a.determinant(), -4071.);
    }
    #[test]
    fn is_invertible() {
        let a = Matrix4x4 (
            [-2., -8., 3., 5.],
            [-3., 1., 7., 3.],
            [1., 2., -9., 6.],
            [-6., 7., 7., -9.]
        );
        
        assert!(a.is_invertible());
    }
    #[test]
    fn not_invertible() {
        let a = Matrix4x4 (
            [-2., -8., 3., 5.],
            [-3., 1., 7., 3.],
            [1., 2., -9., 6.],
            [0., 0., 0., 0.]
        );

        assert!(!a.is_invertible());
    }
    #[test]
    fn invert_matrix() {
        let a = Matrix4x4 (
            [-5., 2., 6., -8.],
            [1., -5., 1., 8.],
            [7., 7., -6., -7.,],
            [1., -3., 7., 4.]
        );

        let b = Matrix4x4 (
            [0.21805, 0.45113, 0.24060, -0.04511],
            [-0.80827, -1.45677, -0.44361, 0.52069],
            [-0.07895, -0.22368, -0.05263, 0.19737],
            [-0.52256, -0.81391, -0.30075, 0.30639]
        );

        assert_eq!(a.inverse(), b)
    }
    #[test]
    fn another_inverse() {
        let a = Matrix4x4 (
            [8., -5., 9., 2.],
            [7., 5., 6., 1.],
            [-6., 0., 9., 6.],
            [-3., 0., -9., -4.]
        );

        let inverse = Matrix4x4 (
            [-0.15385, -0.15385, -0.28205, -0.53846],
            [-0.07692, 0.12308, 0.02564, 0.03077],
            [0.35897, 0.35897, 0.43590, 0.92308],
            [-0.69231, -0.69231, -0.76923, -1.92308]
        );

        assert_eq!(a.inverse(), inverse)
    }
    #[test]
    fn another_inverse_2() {
        let a = Matrix4x4 (
            [9., 3., 0., 9.],
            [-5., -2., -6., -3.],
            [-4., 9., 6., 4.],
            [-7., 6., 6., 2.]
        );

        let inverse = Matrix4x4 (
            [-0.04074, -0.07778, 0.14444, -0.22222],
            [-0.07778, 0.03333, 0.36667, -0.33333],
            [-0.02901, -0.14630, -0.10926, 0.12963],
            [0.17778, 0.06667, -0.26667, 0.33333]
        );

        assert_eq!(a.inverse(), inverse);
    }
    #[test]
    fn mul_translation_point() {
        let transform = Matrix4x4::translation(5., -3., 2.);
        let p: Point<f32> = Point::new(-3., 4., 5.);
        assert_eq!((transform * p).as_point(), Point::new(2., 1., 7.))
    }
    #[test]
    fn mul_inverse_translation() {
        let transform = Matrix4x4::translation(5., -3., 2.);
        let inverse = transform.inverse();
        let p = Point::new(-3., 4., 5.);
        assert_eq!((inverse * p).as_point(), Point::new(-8., 7., 3.));
    }
    #[test]
    fn mul_translation_vector() {
        let transform = Matrix4x4::translation(5., -3., 2.);
        let v = Vector::new(-3., 4., 5.);
        assert_eq!(transform * v, v);
    }
    #[test]
    fn scaling_point() {
        let transform = Matrix4x4::scaling(2., 3., 4.);
        let p = Point::new(-4., 6., 8.);
        assert_eq!((transform * p).as_point(), Point::new(-8., 18., 32.));
    }
    #[test]
    fn scaling_vector() {
        let transform = Matrix4x4::scaling(2., 3., 4.);
        let v = Vector::new(-4., 6., 8.);
        assert_eq!(transform * v, Vector::new(-8., 18., 32.));
    }
    #[test]
    fn inverse_scaling_vector() {
        let transform = Matrix4x4::scaling(2., 3., 4.);
        let inv = transform.inverse();
        let v = Vector::new(-4., 6., 8.);
        assert_eq!(inv * v, Vector::new(-2., 2., 2.));
    }
    #[test]
    fn reflect_point_x() {
        let transform = Matrix4x4::scaling(-1., 1., 1.);
        let p = Point::new(2., 3., 4.);
        assert_eq!((transform * p).as_point(), Point::new(-2., 3., 4.));
    }
    #[test]
    fn rotation_about_x() {
        let p = Point::new(0., 1., 0.);
        let half_quarter = Matrix4x4::rotation(Axis::X, PI / 4.);
        let full_quarter = Matrix4x4::rotation(Axis::X, PI / 2.);
        assert_eq!((half_quarter * p).as_point(), Point::new(0., 2_f32.sqrt() / 2., 2_f32.sqrt() / 2.));
        assert_eq!((full_quarter * p).as_point(), Point::new(0., 0., 1.));
    }
    #[test]
    fn rotation_about_y() {
        let p = Point::new(0., 0., 1.);
        let half_quarter = Matrix4x4::rotation(Axis::Y, PI / 4.);
        let full_quarter = Matrix4x4::rotation(Axis::Y, PI / 2.);
        assert_eq!((half_quarter * p).as_point(), Point::new(2_f32.sqrt() / 2., 0., 2_f32.sqrt() / 2.));
        assert_eq!((full_quarter * p).as_point(), Point::new(1., 0., 0.));
    }
    #[test]
    fn rotation_about_z() {
        let p = Point::new(0., 1., 0.);
        let half_quarter = Matrix4x4::rotation(Axis::Z, PI / 4.);
        let full_quarter = Matrix4x4::rotation(Axis::Z, PI / 2.);
        assert_eq!((half_quarter * p).as_point(), Point::new(-2_f32.sqrt() / 2., 2_f32.sqrt() / 2., 0.));
        assert_eq!((full_quarter * p).as_point(), Point::new(-1., 0., 0.));
    }
    #[test]
    fn shear_x_proportion_y() {
        let transform = Matrix4x4::shearing(1., 0., 0., 0., 0., 0.,);
        let p = Point::new(2.,3., 4.);
        assert_eq!((transform * p).as_point(), Point::new(5., 3., 4.));
    }
    #[test]
    fn shear_x_proportion_z() {
        let transform = Matrix4x4::shearing(0., 1., 0., 0., 0., 0.,);
        let p = Point::new(2.,3., 4.);
        assert_eq!((transform * p).as_point(), Point::new(6., 3., 4.));
    }
    #[test]
    fn shear_y_proportion_x() {
        let transform = Matrix4x4::shearing(0., 0., 1., 0., 0., 0.,);
        let p = Point::new(2.,3., 4.);
        assert_eq!((transform * p).as_point(), Point::new(2., 5., 4.));
    }
    #[test]
    fn shear_y_proportion_z() {
        let transform = Matrix4x4::shearing(0., 0., 0., 1., 0., 0.,);
        let p = Point::new(2.,3., 4.);
        assert_eq!((transform * p).as_point(), Point::new(2., 7., 4.));
    }
    #[test]
    fn shear_z_proportion_x() {
        let transform = Matrix4x4::shearing(0., 0., 0., 0., 1., 0.,);
        let p = Point::new(2.,3., 4.);
        assert_eq!((transform * p).as_point(), Point::new(2., 3., 6.));
    }
    #[test]
    fn shear_z_proportion_y() {
        let transform = Matrix4x4::shearing(0., 0., 0., 0., 0., 1.);
        let p = Point::new(2.,3., 4.);
        assert_eq!((transform * p).as_point(), Point::new(2., 3., 7.));
    }
    #[test]
    fn sequence_applied() {
        let p = Point::new(1., 0., 1.);
        let rotation = Matrix4x4::rotation(Axis::X, PI / 2.);
        let scale = Matrix4x4::scaling(5., 5., 5.);
        let translation = Matrix4x4::translation(10., 5., 7.);

        let p2 = (rotation * p).as_point();
        let p3 = (scale * p2).as_point();
        let p4 = (translation * p3).as_point();
        assert_eq!(p2, Point::new(1., -1., 0.));
        assert_eq!(p3, Point::new(5., -5., 0.));
        assert_eq!(p4, Point::new(15., 0., 7.));

        let t = translation * scale * rotation;

        assert_eq!((t * p).as_point(), Point::new(15., 0., 7.));
    }
    #[test]
    fn query_ray() {
        let origin = Point::new(1., 2., 3.);
        let direction = Vector::new(4., 5., 6.);
        let ray = Ray::new(origin, direction);

        assert_eq!(ray.origin, origin);
        assert_eq!(ray.direction, direction);
    }
    #[test]
    fn point_from_distance() {
        let r = Ray::new(Point::new(2., 3., 4.), Vector::new(1., 0., 0.));
        assert_eq!(r.position(0.), Point::new(2., 3., 4.));
        assert_eq!(r.position(1.), Point::new(3., 3., 4.));
        assert_eq!(r.position(-1.), Point::new(1., 3., 4.));
        assert_eq!(r.position(2.5), Point::new(4.5, 3., 4.));
    }
    #[test]
    fn ray_intersect_sphere() {
        let r = Ray::new(Point::new(0.,0.,-5.), Vector::new(0., 0., 1.));
        let s = Sphere::new();
        let intersection_s = s.intersect(r);
        assert_eq!(intersection_s.len(), 2);
        assert_eq!(intersection_s[0].t, 4.0);
        assert_eq!(intersection_s[1].t, 6.0);
    }
    #[test]
    fn ray_tangent_to_sphere() {
        let r = Ray::new(Point::new(0.,1., -5.), Vector::new(0., 0., 1.));
        let s = Sphere::new();
        let intersection_s = s.intersect(r);
        assert_eq!(intersection_s.len(), 2);
        assert_eq!(intersection_s[0].t, 5.0);
        assert_eq!(intersection_s[1].t, 5.0);
    }
    #[test]
    fn ray_miss_sphere() {
        let r = Ray::new(Point::new(0.,2., -5.), Vector::new(0., 0., 1.));
        let s = Sphere::new();

        let intersection_s = s.intersect(r);
        assert_eq!(intersection_s.len(), 0);
    }
    #[test]
    fn ray_inside_sphere() {
        let r = Ray::new(Point::new(0.,0., 0.), Vector::new(0., 0., 1.));
        let s = Sphere::new();
        let intersection_s = s.intersect(r);
        assert_eq!(intersection_s.len(), 2);
        assert_eq!(intersection_s[0].t, -1.0);
        assert_eq!(intersection_s[1].t, 1.0);
    }
    #[test]
    fn ray_infront_of_sphere() {
        let r = Ray::new(Point::new(0.,0.,5.), Vector::new(0., 0., 1.));
        let s = Sphere::new();
        let intersection_s = s.intersect(r);
        assert_eq!(intersection_s.len(), 2);
        assert_eq!(intersection_s[0].t, -6.0);
        assert_eq!(intersection_s[1].t, -4.0);
    }
    #[test]
    fn ray_intersect_t_and_object() {
        let s = Sphere::new();
        let i = Intersection::new(3.5, SphereObject(s));
        assert_eq!(i.t, 3.5);
        assert_eq!(i.object, SphereObject(s));
    }
    #[test]
    fn aggregating_intersection() {
        let s = Sphere::new();
        let i1 = Intersection::new(1., SphereObject(s));
        let i2 = Intersection::new(2., SphereObject(s));
        let intersections_s = Intersections::new(vec![i1, i2]);
        assert_eq!(intersections_s.len(), 2);
        assert_eq!(intersections_s[0].t, 1.0);
        assert_eq!(intersections_s[1].t, 2.0);
    }
    #[test]
    fn object_set_intersection() {
        let r = Ray::new(Point::new(0., 0., -5.), Vector::new(0., 0., 1.));
        let s = Sphere::new();
        let intersections_s = s.intersect(r);
        assert_eq!(intersections_s.len(), 2);
        assert_eq!(intersections_s[0].object, SphereObject(s));
        assert_eq!(intersections_s[1].object, SphereObject(s));
    }
    #[test]
    fn hit_when_all_pos() {
        let s = Sphere::new();
        let i1 = Intersection::new(1., SphereObject(s));
        let i2 = Intersection::new(2., SphereObject(s));
        let intersections_s = Intersections::new(vec![i1, i2]);
        let i = intersections_s.hit();
        assert_eq!(i.unwrap(), i1);
    }
    #[test]
    fn hit_when_some_negative() {
        let s = Sphere::new();
        let i1 = Intersection::new(-1., SphereObject(s));
        let i2 = Intersection::new(1., SphereObject(s));
        let intersections_s = Intersections::new(vec![i1, i2]);
        let i = intersections_s.hit();
        assert_eq!(i.unwrap(), i2);
    }
    #[test]
    fn all_intersects_negative() {
        let s = Sphere::new();
        let i1 = Intersection::new(-2., SphereObject(s));
        let i2 = Intersection::new(-1., SphereObject(s));
        let intersections_s = Intersections::new(vec![i1, i2]);
        let i = intersections_s.hit();
        assert_eq!(i, None);
    }
    #[test]
    fn lowest_nonnegative_intersection() {
        let s = Sphere::new();
        let i1 = Intersection::new(5., SphereObject(s));
        let i2 = Intersection::new(7., SphereObject(s));
        let i3 = Intersection::new(-3., SphereObject(s));
        let i4 = Intersection::new(2., SphereObject(s));
        let intersections_s = Intersections::new(vec![i1, i2, i3, i4]);
        let i = intersections_s.hit();
        assert_eq!(i.unwrap(), i4);
    }
    #[test]
    fn translating_ray() {
        let r = Ray::new(Point::new(1., 2., 3.), Vector::new(0., 1., 0.));
        let m = Matrix4x4::translation(3., 4., 5.);
        let ray_translated = r.transform(m);
        assert_eq!(ray_translated.origin, Point::new(4., 6., 8.));
        assert_eq!(ray_translated.direction, Vector::new(0., 1., 0.));
    }
    #[test]
    fn scaling_ray() {
        let r = Ray::new(Point::new(1., 2., 3.), Vector::new(0., 1., 0.));
        let m = Matrix4x4::scaling(2., 3., 4.);
        let ray_translated = r.transform(m);
        assert_eq!(ray_translated.origin, Point::new(2., 6., 12.));
        assert_eq!(ray_translated.direction, Vector::new(0., 3., 0.));
    }
    #[test]
    fn identity_matrix_sphere() {
        let s = Sphere::new();
        assert_eq!(s.transform, Matrix4x4::identity());
    }
    #[test]
    fn change_sphere_transform() {
        let mut s = Sphere::new();
        let t = Matrix4x4::translation(2., 3., 4.);
        s.transform = s.set_transform(t);
        assert_eq!(s.transform, t);
    }
    #[test]
    fn intersect_scaled_sphere_with_ray() {
        let r = Ray::new(Point::new(0.,0.,-5.), Vector::new(0.,0.,1.));
        let mut s = Sphere::new();
        s.transform = s.set_transform(Matrix4x4::scaling(2., 2., 2.));
        let intersect_s = s.intersect(r);
        assert_eq!(intersect_s.len(), 2);
        assert_eq!(intersect_s[0].t, 3.);
        assert_eq!(intersect_s[1].t, 7.)
    }
    #[test]
    fn intersect_translated_sphere_with_ray() {
        let r = Ray::new(Point::new(0.,0.,-5.), Vector::new(0.,0.,1.));
        let mut s = Sphere::new();
        s.transform = s.set_transform(Matrix4x4::translation(5.,0.,0.));
        let intersect_s = s.intersect(r);
        assert_eq!(intersect_s.len(), 0);
    }
}
