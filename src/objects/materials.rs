use crate::{features::{operators::{Dot, Normalize}, tuple::{Color, Point, Vector}}, objects::lights::PointLight};

#[derive(Clone, Copy, Debug)]
pub struct Material {
    pub color: Color<f32>,
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub shininess: f32
}

pub fn lighting(
    material: Material, 
    light: PointLight<f32>, 
    point: Point<f32>, 
    camera_vector: Vector<f32>, 
    normal_vector: Vector<f32>
) -> Color<f32> {
    // Combine the surface color with the light's color/intensity
    let effective_color = material.color * light.intensity;
    // find the direction to the light source
    let light_vector = (light.position - point).normalize();
    // compute the ambient contribution
    let ambient = effective_color * material.ambient;
    let diffuse: Color<f32>;
    let specular: Color<f32>;
    // light_dot_normal represents the cosise of the angle between the
    // light vector and the normal vector. A negative number means the
    // light is on the other side of the surface.
    let light_dot_normal = light_vector.dot(&normal_vector);
    if light_dot_normal < 0. {
        diffuse = Color::new(0., 0., 0.);
        specular = Color::new(0., 0., 0.);
        ambient + diffuse + specular
    } else {
        // Compute the diffuse contribution
        diffuse = effective_color * material.diffuse * light_dot_normal;
        // reflect_dot_camera represents the cosine of the angle between the
        // reflection vector and the eye vector. A negative number means the
        // light reflects away from the eye.
        let reflect_vector = light_vector.negate().reflect(normal_vector);
        let reflect_dot_eye = reflect_vector.dot(&camera_vector);
        if reflect_dot_eye <= 0. {
            specular = Color::new(0., 0., 0.);
            ambient + diffuse + specular
        } else {
            // Compute the specular contribution
            let factor = reflect_dot_eye.powf(material.shininess);
            specular = light.intensity * material.specular * factor;
            ambient + diffuse + specular
        }
    }
} 

impl Default for Material {
    fn default() -> Self {
        Self { 
            color: Color::new(1., 1., 1.), 
            ambient: 0.1, 
            diffuse: 0.9, 
            specular: 0.9, 
            shininess: 200. 
        }
    }
}

impl PartialEq for Material {
    fn eq(&self, other: &Self) -> bool {
        self.color == other.color && 
        self.ambient == other.ambient && 
        self.diffuse == other.diffuse && 
        self.specular == other.specular && 
        self.shininess == other.shininess
    }
}