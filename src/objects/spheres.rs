use std::sync::atomic::{AtomicUsize, Ordering};

static NEXT_SPHERE_ID: AtomicUsize = AtomicUsize::new(0);
use crate::{features::{intersections::{Intersection, Intersections}, matrices::Matrix4x4, operators::{Dot, Normalize}, rays::Ray, tuple::{Point, Tuple, Vector}}, objects::materials::Material};

use crate::objects::Object::SphereObject;

#[derive(Clone, Copy, Debug)]
pub struct Sphere {
    pub transform: Matrix4x4<f32>,
    pub material: Material,
    pub id: usize,
}

impl Default for Sphere {
    fn default() -> Self {
        Self { 
            transform: Matrix4x4::identity(), 
            material: Default::default(), 
            id: NEXT_SPHERE_ID.fetch_add(1, Ordering::Relaxed)
        }
    }
}

impl Sphere {
    pub fn new(transform_matrix: Matrix4x4<f32>, object_material: Material) -> Self {
        Self {
            transform: transform_matrix,
            material: object_material,
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

    pub fn normal_at(&self, world_point: Point<f32>) -> Vector<f32> {
        let object_point = self.transform.inverse() * world_point;
        let object_normal = object_point - Tuple::new(0., 0., 0., 1.);
        let world_normal = (self.transform.inverse().transpose() * object_normal).assign(3, 0.0);

        world_normal.as_vector().normalize()
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