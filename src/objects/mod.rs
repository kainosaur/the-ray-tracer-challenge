use crate::objects::spheres::Sphere;

pub mod spheres;

#[derive(Clone, Copy, Debug)]
pub enum Object {
    SphereObject(Sphere),
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::SphereObject(l0), Self::SphereObject(r0)) => l0 == r0,
        }
    }
}