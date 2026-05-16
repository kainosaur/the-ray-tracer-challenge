use crate::objects::spheres::Sphere;
pub mod lights;
pub mod spheres;
pub mod materials;

#[derive(Clone, Copy, Debug)]
pub enum Object {
    SphereObject(Sphere),
}

impl Object {
    pub fn get_sphere(&self) -> Option<&Sphere> {
        match self {
            Object::SphereObject(s) => Some(s),
        }
    }
}


impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::SphereObject(l0), Self::SphereObject(r0)) => l0 == r0,
        }
    }
}