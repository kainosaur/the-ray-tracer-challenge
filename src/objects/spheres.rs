use std::sync::atomic::{AtomicUsize, Ordering};

static NEXT_SPHERE_ID: AtomicUsize = AtomicUsize::new(0);
use crate::features::{intersections::{Intersection, Intersections}, matrices::Matrix4x4, operators::Dot, rays::Ray, tuple::Point};

use crate::objects::Object::SphereObject;

#[derive(Clone, Copy, Debug)]
pub struct Sphere {
    pub transform: Matrix4x4<f32>,
    pub id: usize,
}

impl Sphere {
    pub fn new() -> Self {
        Self {
            transform: Matrix4x4::identity(),
            id: NEXT_SPHERE_ID.fetch_add(1, Ordering::Relaxed),
        }
    }

    pub fn intersect(&self, input_ray: Ray<f32>) -> Intersections {
        let ray = input_ray.transform(self.transform.inverse());
        let sphere_to_ray = ray.origin - Point::new(0.,0.,0.);

        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * ray.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.;

        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            Intersections::new(vec![])
        } else {
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

            Intersections::new(vec!
            [
                Intersection::new(t1, SphereObject(self.clone())),
                Intersection::new(t2, SphereObject(self.clone()))
            ])
        }
    }

    pub fn set_transform(&mut self, transform_matrix: Matrix4x4<f32>) -> Matrix4x4<f32> {
        self.transform * transform_matrix
    }
}

impl PartialEq for Sphere {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}